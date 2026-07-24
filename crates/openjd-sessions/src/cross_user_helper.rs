// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! Persistent cross-user helper process for subprocess execution.
//!
//! On POSIX, the helper is launched via `sudo -u <user> -i <helper_path>`.
//! On Windows, the helper is launched via `CreateProcessAsUserW` / `CreateProcessWithLogonW`.
//!
//! In both cases the helper communicates over newline-delimited JSON on stdin/stdout,
//! avoiding per-action login costs and enabling reliable cancellation from the
//! target user's context.

use std::io::Read;
use std::path::Path;

use crate::action::{ActionMessage, ActionState};
use crate::error::SessionError;
use crate::logging::LogContent;

/// Maximum bytes for a single JSON response line from the helper's stdout.
/// Matches the helper binary's own `MAX_LINE_LENGTH` (128 KB).
const MAX_RESPONSE_LINE_LENGTH: usize = 128 * 1024;
use crate::session_log;
use crate::session_user::SessionUser;

/// Length of the shared auth token in characters.
///
/// Matches the helper's `AUTH_TOKEN_LEN` constant. The helper rejects
/// `--auth-token` values of any other length.
pub(crate) const AUTH_TOKEN_LEN: usize = 22;

/// 64-character URL-safe alphabet used to render the auth token.
///
/// Chosen so that `byte & 0x3F` is a uniform index with no rejection
/// sampling. The character set avoids `+` / `/` / `=` / whitespace so the
/// token is safe in argv, JSON, filenames, and log lines without quoting.
const AUTH_TOKEN_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Generate a fresh per-helper auth token: [`AUTH_TOKEN_LEN`] ASCII characters
/// drawn from [`AUTH_TOKEN_ALPHABET`]. Uses the OS CSPRNG via `getrandom::fill`.
///
/// Each character contributes 6 bits of entropy (64-char alphabet), so
/// 22 characters ≥ 128 bits of entropy.
pub(crate) fn generate_auth_token() -> Result<String, SessionError> {
    let mut raw = [0u8; AUTH_TOKEN_LEN];
    getrandom::fill(&mut raw).map_err(|e| {
        SessionError::HelperCommunication(format!("Failed to generate auth token: {e}"))
    })?;
    let mut out = String::with_capacity(AUTH_TOKEN_LEN);
    for b in raw {
        out.push(AUTH_TOKEN_ALPHABET[(b & 0x3F) as usize] as char);
    }
    Ok(out)
}

/// Return `cmd` extended with `"token": <auth_token>`.
///
/// `cmd` must be a JSON object, since every helper command is an object.
/// If the caller's map already contains a `"token"` entry, it is overwritten
/// — `augment_with_token` is the sole owner of that field in outgoing
/// commands.
fn augment_with_token(
    cmd: &serde_json::Value,
    auth_token: &str,
) -> Result<serde_json::Value, SessionError> {
    let serde_json::Value::Object(map) = cmd else {
        return Err(SessionError::HelperCommunication(format!(
            "helper command must be a JSON object, got: {cmd}"
        )));
    };
    let mut with_token = map.clone();
    with_token.insert(
        "token".to_string(),
        serde_json::Value::String(auth_token.to_string()),
    );
    Ok(serde_json::Value::Object(with_token))
}

// ── Async helper reader ──

/// Future type returned by `AsyncHelperReader::next_response`.
type NextResponseFuture<'a> = std::pin::Pin<
    Box<
        dyn std::future::Future<Output = Option<Result<serde_json::Value, SessionError>>>
            + Send
            + 'a,
    >,
>;

/// Async stream of JSON responses from the helper's stdout.
/// Both platforms use a reader thread + async channel.
pub(crate) trait AsyncHelperReader: Send {
    /// Receive the next JSON response. Returns None when the helper exits.
    fn next_response(&mut self) -> NextResponseFuture<'_>;
}

/// Unix: async reads using a reader thread + channel (same pattern as Windows).
/// While AsyncFd could theoretically provide zero-thread async reads on Unix,
/// it doesn't implement AsyncRead directly. The reader thread approach is
/// simple, uniform across platforms, and has negligible overhead.
#[cfg(unix)]
pub(crate) struct UnixAsyncHelperReader {
    rx: tokio::sync::mpsc::UnboundedReceiver<Result<serde_json::Value, SessionError>>,
    _thread: Option<std::thread::JoinHandle<()>>,
}

#[cfg(unix)]
impl UnixAsyncHelperReader {
    pub fn new(stdout: std::process::ChildStdout) -> Result<Self, SessionError> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let thread = std::thread::spawn(move || {
            use std::io::BufRead;
            let mut reader = std::io::BufReader::new(stdout);
            let mut line = String::new();
            loop {
                line.clear();
                match Read::take(&mut reader, MAX_RESPONSE_LINE_LENGTH as u64).read_line(&mut line)
                {
                    Ok(0) => break,
                    Ok(n) => {
                        if n == MAX_RESPONSE_LINE_LENGTH && !line.ends_with('\n') {
                            // Line exceeded limit — discard remainder and report error
                            let mut discard = Vec::new();
                            let _ = reader.read_until(b'\n', &mut discard);
                            let result = Err(SessionError::HelperCommunication(format!(
                                "response line exceeds {MAX_RESPONSE_LINE_LENGTH} byte limit"
                            )));
                            if tx.send(result).is_err() {
                                break;
                            }
                            continue;
                        }
                        let result = serde_json::from_str(line.trim_end()).map_err(|e| {
                            SessionError::HelperCommunication(format!("parse error: {e}"))
                        });
                        if tx.send(result).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        Ok(Self {
            rx,
            _thread: Some(thread),
        })
    }
}

#[cfg(unix)]
impl AsyncHelperReader for UnixAsyncHelperReader {
    fn next_response(&mut self) -> NextResponseFuture<'_> {
        Box::pin(async { self.rx.recv().await })
    }
}

/// Windows: reader thread relays lines from the sync pipe to an async channel.
#[cfg(windows)]
pub(crate) struct WindowsAsyncHelperReader {
    rx: tokio::sync::mpsc::UnboundedReceiver<Result<serde_json::Value, SessionError>>,
    _thread: Option<std::thread::JoinHandle<()>>,
}

#[cfg(windows)]
impl WindowsAsyncHelperReader {
    pub fn new(stdout: std::fs::File) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let thread = std::thread::spawn(move || {
            use std::io::BufRead;
            let mut reader = std::io::BufReader::new(stdout);
            let mut line = String::new();
            loop {
                line.clear();
                match Read::take(&mut reader, MAX_RESPONSE_LINE_LENGTH as u64).read_line(&mut line)
                {
                    Ok(0) => break,
                    Ok(n) => {
                        if n == MAX_RESPONSE_LINE_LENGTH && !line.ends_with('\n') {
                            // Line exceeded limit — discard remainder and report error
                            let mut discard = Vec::new();
                            let _ = reader.read_until(b'\n', &mut discard);
                            let result = Err(SessionError::HelperCommunication(format!(
                                "response line exceeds {MAX_RESPONSE_LINE_LENGTH} byte limit"
                            )));
                            if tx.send(result).is_err() {
                                break;
                            }
                            continue;
                        }
                        let result = serde_json::from_str(line.trim_end()).map_err(|e| {
                            SessionError::HelperCommunication(format!("parse error: {e}"))
                        });
                        if tx.send(result).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        Self {
            rx,
            _thread: Some(thread),
        }
    }
}

#[cfg(windows)]
impl AsyncHelperReader for WindowsAsyncHelperReader {
    fn next_response(&mut self) -> NextResponseFuture<'_> {
        Box::pin(async { self.rx.recv().await })
    }
}

/// Manages a long-lived helper process for cross-user command execution (POSIX).
///
/// On POSIX: launched via `sudo -u <user> -i <helper_path> --auth-token <TOKEN>`.
/// Communicates over newline-delimited JSON on stdin/stdout. Every request
/// carries the shared `auth_token` string; the helper rejects anything else.
#[cfg(unix)]
pub(crate) struct CrossUserHelper {
    child: std::process::Child,
    stdin: std::io::BufWriter<std::process::ChildStdin>,
    pub(crate) async_reader: UnixAsyncHelperReader,
    /// 22-char auth token shared with the helper. Included in every command
    /// sent over `stdin` and over the dup'd `cancel_writer`.
    auth_token: String,
}

/// Windows variant: the child is a raw process handle from `CreateProcessAsUserW`,
/// not a `std::process::Child`. We wrap stdin/stdout from the pipe handles.
#[cfg(windows)]
pub(crate) struct CrossUserHelperWin {
    process_handle: windows::Win32::Foundation::HANDLE,
    stdin: std::io::BufWriter<std::fs::File>,
    pub(crate) async_reader: WindowsAsyncHelperReader,
    /// 22-char auth token shared with the helper. Included in every command
    /// sent over `stdin` and over the dup'd `cancel_writer`.
    auth_token: String,
}

// SAFETY: `CrossUserHelperWin` is Send because all of its fields can be
// sent across threads:
// - `process_handle: HANDLE` is a Windows kernel object handle (pointer-
//   sized integer). Kernel handles are process-wide and safe to use from
//   any thread. `HANDLE` is `!Send` in `windows-rs` out of caution, but the
//   process handle here is only used for wait/terminate operations that
//   accept any thread's handle.
// - `stdin: BufWriter<File>` is already Send.
// - `async_reader: WindowsAsyncHelperReader` owns an `UnboundedReceiver`
//   and a `JoinHandle`, both of which are Send.
#[cfg(windows)]
unsafe impl Send for CrossUserHelperWin {}

#[cfg(unix)]
impl CrossUserHelper {
    /// Spawn the helper binary as the given user via sudo (POSIX).
    ///
    /// Generates a fresh per-helper auth token, passes it to the helper as
    /// `--auth-token <TOKEN>`, and stores it on the returned struct so every
    /// subsequent command can echo it back.
    ///
    /// Returns `(helper, cancel_writer)` where `cancel_writer` is a dup'd copy
    /// of the helper's stdin fd. This allows sending cancel commands even while
    /// the helper struct is moved to a runner during action execution.
    #[cfg(unix)]
    pub fn spawn(
        helper_path: &Path,
        user: &dyn SessionUser,
    ) -> Result<(Self, std::fs::File), SessionError> {
        let auth_token = generate_auth_token()?;
        let mut child = std::process::Command::new("sudo")
            .args([
                "-u",
                user.user(),
                "-i",
                &helper_path.to_string_lossy(),
                "--auth-token",
                &auth_token,
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|source| SessionError::SubprocessStart {
                // Don't include the token in error messages.
                command: format!(
                    "sudo -u {} -i {} --auth-token <redacted>",
                    user.user(),
                    helper_path.display()
                ),
                source,
            })?;

        let child_stdin = child.stdin.take().expect("stdin was piped");

        // Dup the stdin fd so we can write cancel commands from Session::cancel_action
        // while the helper is owned by a runner.
        use std::os::unix::io::{AsFd, FromRawFd, IntoRawFd};
        let dup_fd = nix::unistd::dup(child_stdin.as_fd()).map_err(|e| {
            SessionError::HelperCommunication(format!("Failed to dup helper stdin fd: {e}"))
        })?;
        let cancel_writer = unsafe { FromRawFd::from_raw_fd(dup_fd.into_raw_fd()) };

        let stdin = std::io::BufWriter::new(child_stdin);
        let async_reader =
            UnixAsyncHelperReader::new(child.stdout.take().expect("stdout was piped"))?;

        Ok((
            Self {
                child,
                stdin,
                async_reader,
                auth_token,
            },
            cancel_writer,
        ))
    }

    /// The shared auth token. Callers that write directly to the helper's
    /// stdin (like the cancel_writer path) must include this in every request.
    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }

    /// Send a JSON command to the helper, injecting the shared auth token.
    ///
    /// `cmd` must be a JSON object; the helper's protocol requires every
    /// command to be an object. The token is inserted as `"token"` before
    /// the line is written.
    pub fn send_command(&mut self, cmd: &serde_json::Value) -> Result<(), SessionError> {
        use std::io::Write;
        let payload = augment_with_token(cmd, &self.auth_token)?;
        serde_json::to_writer(&mut self.stdin, &payload).map_err(|e| {
            SessionError::HelperCommunication(format!("Failed to write to helper stdin: {e}"))
        })?;
        self.stdin.write_all(b"\n").map_err(|e| {
            SessionError::HelperCommunication(format!("Failed to write newline to helper: {e}"))
        })?;
        self.stdin.flush().map_err(|e| {
            SessionError::HelperCommunication(format!("Failed to flush helper stdin: {e}"))
        })?;
        Ok(())
    }

    /// Send a tokenized shutdown command and wait for the child to exit.
    pub fn shutdown(&mut self) {
        let _ = self.send_command(&serde_json::json!({"shutdown": true}));
        let _ = self.child.wait();
    }
}

#[cfg(unix)]
impl Drop for CrossUserHelper {
    fn drop(&mut self) {
        // Safety net: kill the child if shutdown() wasn't called.
        if let Ok(None) = self.child.try_wait() {
            log::warn!(target: "openjd.sessions", "CrossUserHelper dropped without shutdown(), killing child process");
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[cfg(windows)]
impl CrossUserHelperWin {
    /// Spawn the helper binary as the given user via `CreateProcessAsUserW` (Windows).
    ///
    /// Generates a fresh per-helper auth token, passes it to the helper as
    /// `--auth-token <TOKEN>`, and stores it on the returned struct so every
    /// subsequent command can echo it back.
    ///
    /// Returns `(helper, cancel_writer)` where `cancel_writer` is a `DuplicateHandle`'d
    /// copy of the helper's stdin pipe. This allows sending cancel commands even while
    /// the helper struct is moved to a runner during action execution.
    pub fn spawn(
        helper_path: &Path,
        user: &dyn SessionUser,
    ) -> Result<(Self, std::fs::File), SessionError> {
        use crate::session_user::WindowsSessionUser;
        use std::collections::HashMap;
        use std::os::windows::io::FromRawHandle;
        use windows::Win32::Foundation::{DuplicateHandle, DUPLICATE_SAME_ACCESS};
        use windows::Win32::System::Threading::GetCurrentProcess;

        let wu = user
            .as_any()
            .downcast_ref::<WindowsSessionUser>()
            .ok_or_else(|| {
                SessionError::Runtime("Cross-user on Windows requires WindowsSessionUser".into())
            })?;

        let auth_token = generate_auth_token()?;

        let spawned = crate::win32::spawn_as_user_with_stdin(
            &[
                helper_path.to_string_lossy().to_string(),
                "--auth-token".to_string(),
                auth_token.clone(),
            ],
            &HashMap::new(),
            None, // working_dir — helper doesn't need one
            wu.password(),
            wu.user(),
            wu.logon_token(),
        )
        .map_err(|e| SessionError::SubprocessStart {
            // Don't include the token in error messages.
            command: format!(
                "spawn_as_user_with_stdin {} --auth-token <redacted>",
                helper_path.display()
            ),
            source: std::io::Error::other(e),
        })?;

        let stdin_write = spawned.stdin_write.ok_or_else(|| {
            SessionError::HelperCommunication(
                "spawn_as_user_with_stdin did not return stdin pipe".into(),
            )
        })?;

        // Convert OwnedHandle → File for stdin
        let stdin_file: std::fs::File = stdin_write.into();

        // DuplicateHandle to create a cancel_writer (like dup() on POSIX)
        let cancel_writer = unsafe {
            use std::os::windows::io::AsRawHandle;
            let current_process = GetCurrentProcess();
            let src_handle = windows::Win32::Foundation::HANDLE(stdin_file.as_raw_handle());
            let mut dup_handle = windows::Win32::Foundation::HANDLE::default();
            DuplicateHandle(
                current_process,
                src_handle,
                current_process,
                &mut dup_handle,
                0,
                false,
                DUPLICATE_SAME_ACCESS,
            )
            .map_err(|e| {
                SessionError::HelperCommunication(format!(
                    "DuplicateHandle for cancel_writer failed: {e}"
                ))
            })?;
            std::fs::File::from_raw_handle(dup_handle.0 as std::os::windows::io::RawHandle)
        };

        // Convert stdout OwnedHandle → File
        let stdout_file: std::fs::File = spawned.stdout_read.into();

        let stdin = std::io::BufWriter::new(stdin_file);
        let async_reader = WindowsAsyncHelperReader::new(stdout_file);

        Ok((
            Self {
                process_handle: spawned.process_handle,
                stdin,
                async_reader,
                auth_token,
            },
            cancel_writer,
        ))
    }

    /// The shared auth token. Callers that write directly to the helper's
    /// stdin (like the cancel_writer path) must include this in every request.
    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }

    /// Send a JSON command to the helper, injecting the shared auth token.
    pub fn send_command(&mut self, cmd: &serde_json::Value) -> Result<(), SessionError> {
        use std::io::Write;
        let payload = augment_with_token(cmd, &self.auth_token)?;
        serde_json::to_writer(&mut self.stdin, &payload).map_err(|e| {
            SessionError::HelperCommunication(format!("Failed to write to helper stdin: {e}"))
        })?;
        self.stdin.write_all(b"\n").map_err(|e| {
            SessionError::HelperCommunication(format!("Failed to write newline to helper: {e}"))
        })?;
        self.stdin.flush().map_err(|e| {
            SessionError::HelperCommunication(format!("Failed to flush helper stdin: {e}"))
        })?;
        Ok(())
    }

    /// Send a tokenized shutdown command and wait for the process to exit.
    pub fn shutdown(&mut self) {
        let _ = self.send_command(&serde_json::json!({"shutdown": true}));
        unsafe {
            let _ =
                windows::Win32::System::Threading::WaitForSingleObject(self.process_handle, 5000);
            let _ = windows::Win32::Foundation::CloseHandle(self.process_handle);
            self.process_handle = windows::Win32::Foundation::INVALID_HANDLE_VALUE;
        }
    }
}

#[cfg(windows)]
impl Drop for CrossUserHelperWin {
    fn drop(&mut self) {
        // Safety net: terminate the process if shutdown() wasn't called.
        if !self.process_handle.is_invalid() {
            log::warn!(target: "openjd.sessions", "CrossUserHelperWin dropped without shutdown(), terminating child process");
            unsafe {
                let _ = windows::Win32::System::Threading::TerminateProcess(self.process_handle, 1);
                let _ = windows::Win32::Foundation::CloseHandle(self.process_handle);
            }
        }
    }
}

/// Trait for helpers that provide both async reading and sync command sending.
pub(crate) trait AsyncHelper: Send {
    fn async_reader(&mut self) -> &mut dyn AsyncHelperReader;
    fn send_command(&mut self, cmd: &serde_json::Value) -> Result<(), SessionError>;
    /// The helper's shared auth token. Used by callers that bypass
    /// `send_command` (e.g. the cancel_writer path in `run_via_helper`).
    fn auth_token(&self) -> &str;
}

#[cfg(unix)]
impl AsyncHelper for CrossUserHelper {
    fn async_reader(&mut self) -> &mut dyn AsyncHelperReader {
        &mut self.async_reader
    }
    fn send_command(&mut self, cmd: &serde_json::Value) -> Result<(), SessionError> {
        CrossUserHelper::send_command(self, cmd)
    }
    fn auth_token(&self) -> &str {
        CrossUserHelper::auth_token(self)
    }
}

#[cfg(windows)]
impl AsyncHelper for CrossUserHelperWin {
    fn async_reader(&mut self) -> &mut dyn AsyncHelperReader {
        &mut self.async_reader
    }
    fn send_command(&mut self, cmd: &serde_json::Value) -> Result<(), SessionError> {
        CrossUserHelperWin::send_command(self, cmd)
    }
    fn auth_token(&self) -> &str {
        CrossUserHelperWin::auth_token(self)
    }
}

/// Write a tokenized cancel command to the helper over `cancel_writer`.
///
/// Mirrors the framing that both the timeout path and `Session::cancel_action`
/// write directly to the helper's stdin: a `TERMINATE` command for immediate
/// cancels, or a `NOTIFY_THEN_TERMINATE` command carrying the grace period
/// otherwise. The token alphabet excludes `"` and `\`, so no escaping is
/// needed to embed it in a JSON literal.
fn write_cancel_to_helper(
    cancel_writer: &std::fs::File,
    auth_token: &str,
    cancel_method: &crate::runner::CancelMethod,
) -> Result<(), SessionError> {
    use std::io::Write;
    let mut w = cancel_writer.try_clone().map_err(|e| {
        SessionError::HelperCommunication(format!("Failed to clone cancel_writer: {e}"))
    })?;
    let notify_period = match cancel_method {
        crate::runner::CancelMethod::NotifyThenTerminate { terminate_delay } => {
            terminate_delay.as_secs()
        }
        crate::runner::CancelMethod::Terminate => 0,
    };
    if notify_period == 0 {
        let _ = writeln!(w, r#"{{"token":"{auth_token}","cancel":"TERMINATE"}}"#);
    } else {
        let _ = writeln!(
            w,
            r#"{{"token":"{auth_token}","cancel":"NOTIFY_THEN_TERMINATE","notifyPeriodInSeconds":{notify_period}}}"#
        );
    }
    let _ = w.flush();
    Ok(())
}

/// Execute a subprocess via a CrossUserHelper, returning the result.
///
/// This is async — the helper stdout is read via `AsyncHelperReader`, allowing
/// `drive_action`'s select loop to process `ActionMessage`s (progress, status,
/// env vars) concurrently while the helper runs.
///
/// Cancellation is observed over two channels, mirroring the same-user
/// `run_subprocess` loop:
///   * `cancel_token` — the per-action [`CancellationToken`], which an external
///     caller can trip directly via `SessionConfig.cancel_token` (a token-only
///     cancel that never touches the watch channel), and which
///     `Session::cancel_action` also fires.
///   * `config.cancel_request_rx` — the watch channel that `cancel_action`
///     mirrors onto the helper's stdin.
///
/// A fire on either channel maps the final action state to `Canceled`.
///
/// [`CancellationToken`]: tokio_util::sync::CancellationToken
pub(crate) async fn run_via_helper(
    helper: &mut dyn AsyncHelper,
    config: &crate::subprocess::SubprocessConfig,
    filter: &mut crate::action_filter::ActionFilter,
    session_id: &str,
    message_tx: tokio::sync::mpsc::UnboundedSender<ActionMessage>,
    cancel_writer: Option<&std::fs::File>,
    cancel_token: &tokio_util::sync::CancellationToken,
) -> Result<crate::subprocess::SubprocessResult, SessionError> {
    // Build the env map (only set values; unsets are excluded).
    let env: serde_json::Map<String, serde_json::Value> = config
        .env_vars
        .iter()
        .filter_map(|(k, v)| {
            v.as_ref()
                .map(|val| (k.clone(), serde_json::Value::String(val.clone())))
        })
        .collect();

    let cmd = serde_json::json!({
        "command": config.args[0],
        "args": &config.args[1..],
        "env": env,
        "cwd": config.working_dir,
    });

    helper.send_command(&cmd)?;

    // Log the actual command (not the helper protocol)
    session_log!(
        info,
        session_id,
        LogContent::FILE_PATH | LogContent::PROCESS_CONTROL,
        "Running command {}",
        crate::subprocess::format_command_for_log(&config.args)
    );

    // Timeout as async future instead of OS thread
    let timeout_fut = match config.timeout {
        Some(d) => {
            use futures_util::FutureExt;
            tokio::time::sleep(d).boxed()
        }
        None => {
            use futures_util::FutureExt;
            futures_util::future::pending::<()>().boxed()
        }
    };
    tokio::pin!(timeout_fut);
    let mut timed_out = false;

    let mut stdout_collected = String::new();
    let mut saw_fail = false;
    // Whether a cancel command has already been written to the helper. The
    // watch channel (fired by `Session::cancel_action`) also cancels the
    // token, so without this guard a `cancel_action`-initiated cancel would
    // deliver the cancel command twice (once via `cancel_action` writing to
    // the cancel_writer directly, once from the token branch below). A second
    // command is harmless to the helper protocol, but we avoid it when easy.
    let mut cancel_sent = false;

    loop {
        tokio::select! {
            biased;
            resp = helper.async_reader().next_response() => {
                let resp = match resp {
                    None => return Err(SessionError::HelperCommunication(
                        "Helper process closed stdout unexpectedly".into(),
                    )),
                    Some(r) => r?,
                };

                if let Some(pid) = resp.get("pid").and_then(|v| v.as_i64()) {
                    session_log!(
                        info,
                        session_id,
                        LogContent::PROCESS_CONTROL,
                        "Command started as pid: {}",
                        pid
                    );
                    continue;
                }

                if let Some(line) = resp.get("out").and_then(|v| v.as_str()) {
                    let line = crate::subprocess::truncate_line(line);
                    let (display, pass_through) = crate::subprocess::process_line(
                        line,
                        filter,
                        session_id,
                        &message_tx,
                        &mut saw_fail,
                    );
                    if pass_through && filter.min_log_level() <= 20 {
                        session_log!(info, session_id, LogContent::COMMAND_OUTPUT, "{}", display);
                    }
                    if config.debug_collect_stdout {
                        stdout_collected.push_str(&display);
                        stdout_collected.push('\n');
                    }
                    continue;
                }

                if let Some(code) = resp.get("exited").and_then(|v| v.as_i64()) {
                    let exit_code = code as i32;
                    session_log!(
                        info,
                        session_id,
                        LogContent::PROCESS_CONTROL,
                        "Process exit code: {}",
                        exit_code
                    );

                    // A cancel may arrive over either channel: the watch
                    // channel (mirrored by `Session::cancel_action`) or the
                    // `CancellationToken` (which an external caller can trip
                    // directly via `SessionConfig.cancel_token`). Treat a fire
                    // on either as a cancel — mirroring the sticky
                    // `is_cancelled()` check in the same-user `run_subprocess`.
                    let canceled = cancel_token.is_cancelled()
                        || config
                            .cancel_request_rx
                            .as_ref()
                            .is_some_and(|rx| rx.has_changed().unwrap_or(false));

                    let state = if canceled {
                        ActionState::Canceled
                    } else if timed_out {
                        ActionState::Timeout
                    } else if saw_fail {
                        ActionState::Failed
                    } else if exit_code == 0 {
                        ActionState::Success
                    } else {
                        ActionState::Failed
                    };
                    return Ok(crate::subprocess::SubprocessResult {
                        state,
                        exit_code: Some(exit_code),
                        stdout: stdout_collected,
                    });
                }

                if let Some(msg) = resp.get("error").and_then(|v| v.as_str()) {
                    return Err(SessionError::SubprocessStart {
                        command: config.args[0].clone(),
                        source: std::io::Error::other(msg.to_string()),
                    });
                }

                return Err(SessionError::HelperCommunication(format!(
                    "Unexpected helper response: {}",
                    resp
                )));
            }
            // External / per-action cancel via the CancellationToken. This is
            // the channel a worker agent trips through `SessionConfig.cancel_token`
            // and that `Session::cancel_action` also fires. Mirror the same-user
            // loop's `cancel_token.cancelled()` branch: deliver the cancel to the
            // helper over the cancel_writer so the child is actually stopped.
            _ = cancel_token.cancelled(), if !cancel_sent => {
                cancel_sent = true;
                if let Some(writer) = cancel_writer {
                    write_cancel_to_helper(writer, helper.auth_token(), &config.cancel_method)?;
                }
            }
            _ = &mut timeout_fut, if !timed_out => {
                timed_out = true;
                // Send cancel to helper via the cancel_writer
                if let Some(writer) = cancel_writer {
                    cancel_sent = true;
                    write_cancel_to_helper(writer, helper.auth_token(), &config.cancel_method)?;
                }
            }
        }
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn token_is_22_chars_from_url_safe_alphabet() {
        for _ in 0..100 {
            let t = generate_auth_token().unwrap();
            assert_eq!(
                t.len(),
                AUTH_TOKEN_LEN,
                "token should be {AUTH_TOKEN_LEN} chars",
            );
            assert!(t.is_ascii(), "token must be ASCII");
            for c in t.bytes() {
                assert!(
                    AUTH_TOKEN_ALPHABET.contains(&c),
                    "token contains non-alphabet byte: {c:#x}",
                );
            }
        }
    }

    #[test]
    fn generated_tokens_differ() {
        // This could technically collide, but the probability over two
        // 128-bit draws is astronomically small. If this ever flakes, the
        // CSPRNG is broken.
        let a = generate_auth_token().unwrap();
        let b = generate_auth_token().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn augment_with_token_inserts_token() {
        let cmd = serde_json::json!({"command": "echo", "args": ["hi"]});
        let out = augment_with_token(&cmd, "T-O-K-E-N").unwrap();
        assert_eq!(out.get("token").and_then(|v| v.as_str()), Some("T-O-K-E-N"),);
        assert_eq!(out.get("command").and_then(|v| v.as_str()), Some("echo"));
    }

    #[test]
    fn augment_rejects_non_object() {
        let err = augment_with_token(&serde_json::json!("shutdown"), "tok").unwrap_err();
        assert!(err.to_string().contains("must be a JSON object"));
    }

    #[test]
    fn augment_overrides_caller_supplied_token() {
        // augment_with_token is the sole owner of the "token" field on
        // outgoing commands — a caller-supplied one must be overwritten
        // so callers can't accidentally (or intentionally) poison it.
        let cmd = serde_json::json!({"token": "bad", "command": "x"});
        let out = augment_with_token(&cmd, "good").unwrap();
        assert_eq!(
            out.get("token").and_then(|v| v.as_str()),
            Some("good"),
            "augment_with_token must not let the caller override the token",
        );
    }
}

/// Tests that `run_via_helper` observes the per-action `CancellationToken`.
///
/// These exercise the cross-user cancel path without Docker/root by faking the
/// helper: `ScriptedHelper` implements the `pub(crate)` `AsyncHelper` trait,
/// feeding scripted stdout responses and recording the auth token, while a real
/// OS pipe stands in for the `cancel_writer` so the test can read back exactly
/// what `run_via_helper` wrote to the helper's stdin.
#[cfg(unix)]
#[cfg(test)]
mod cancel_token_tests {
    use super::*;
    use tokio_util::sync::CancellationToken;

    /// A scripted stdout reader that models a still-running child:
    ///   * call 1 → a `pid` line
    ///   * call 2 → never resolves (child running); the only way out of the
    ///     loop iteration is the cancel/timeout branch, which drops this future
    ///   * call 3+ → the `exited` response the helper emits once the child dies
    ///
    /// This ordering guarantees `run_via_helper` writes the cancel command to
    /// the cancel_writer (call-2 future is pending when the token fires, so the
    /// biased `select!` takes the cancel branch) *before* it observes `exited`
    /// and computes the final state.
    struct MockReader {
        exit_code: i64,
        calls: u32,
    }

    impl AsyncHelperReader for MockReader {
        fn next_response(&mut self) -> NextResponseFuture<'_> {
            self.calls += 1;
            let call = self.calls;
            let exit_code = self.exit_code;
            Box::pin(async move {
                match call {
                    1 => Some(Ok(serde_json::json!({"pid": 4242}))),
                    2 => std::future::pending().await,
                    _ => Some(Ok(serde_json::json!({"exited": exit_code}))),
                }
            })
        }
    }

    struct ScriptedHelper {
        auth_token: String,
        reader: MockReader,
    }

    impl AsyncHelper for ScriptedHelper {
        fn async_reader(&mut self) -> &mut dyn AsyncHelperReader {
            &mut self.reader
        }
        fn send_command(&mut self, _cmd: &serde_json::Value) -> Result<(), SessionError> {
            Ok(())
        }
        fn auth_token(&self) -> &str {
            &self.auth_token
        }
    }

    fn base_config(
        cancel_method: crate::runner::CancelMethod,
    ) -> crate::subprocess::SubprocessConfig {
        crate::subprocess::SubprocessConfig {
            args: vec!["sleep".into(), "100".into()],
            env_vars: std::collections::HashMap::new(),
            working_dir: None,
            timeout: None,
            user: None,
            cancel_method,
            cancel_request_rx: None,
            debug_collect_stdout: false,
        }
    }

    /// The core regression test: a token-only external cancel (as delivered by
    /// `SessionConfig.cancel_token`, which never touches the watch channel)
    /// must (a) be written to the helper over the cancel_writer, and (b) map
    /// the final action state to `Canceled` — not `Success` — even though the
    /// child then reports exit code 0.
    #[tokio::test(flavor = "multi_thread")]
    async fn token_cancel_writes_to_helper_and_maps_to_canceled() {
        use std::os::unix::io::{FromRawFd, IntoRawFd};

        // Real OS pipe: the write end is the cancel_writer; the read end lets
        // the test observe exactly what run_via_helper wrote to the helper.
        let (read_fd, write_fd) = nix::unistd::pipe().unwrap();
        let cancel_writer = unsafe { std::fs::File::from_raw_fd(write_fd.into_raw_fd()) };
        let mut read_end = unsafe { std::fs::File::from_raw_fd(read_fd.into_raw_fd()) };

        let cancel_token = CancellationToken::new();
        let mut helper = ScriptedHelper {
            auth_token: "AUTHTOKENxxxxxxxxxxxxx".into(),
            // Child reports success — the token is the ONLY cancel signal.
            reader: MockReader {
                exit_code: 0,
                calls: 0,
            },
        };
        let config = base_config(crate::runner::CancelMethod::Terminate);
        let mut filter = crate::action_filter::ActionFilter::new("test", true, false);
        let (msg_tx, _msg_rx) = tokio::sync::mpsc::unbounded_channel();

        // Fire the token shortly after the loop parks on the pending reader.
        let token_for_task = cancel_token.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            token_for_task.cancel();
        });

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            run_via_helper(
                &mut helper,
                &config,
                &mut filter,
                "test",
                msg_tx,
                Some(&cancel_writer),
                &cancel_token,
            ),
        )
        .await
        .expect("run_via_helper should return after the token-driven cancel")
        .expect("run_via_helper should not error");

        // (b) A token-only cancel maps the final state to Canceled.
        assert_eq!(
            result.state,
            ActionState::Canceled,
            "a token-only external cancel must map the action state to Canceled, not {:?}",
            result.state
        );

        // (a) The cancel command was written to the helper over cancel_writer.
        // Drop our write-end handles so the read cannot block; the child in
        // run_via_helper holds a clone it flushes and then drops on return.
        drop(cancel_writer);
        let mut buf = Vec::new();
        read_end.read_to_end(&mut buf).unwrap();
        let written = String::from_utf8(buf).unwrap();
        let parsed: serde_json::Value = written
            .lines()
            .next()
            .map(|l| serde_json::from_str(l).unwrap())
            .expect("run_via_helper must write a cancel command to the helper");
        assert_eq!(parsed["cancel"].as_str(), Some("TERMINATE"));
        assert_eq!(
            parsed["token"].as_str(),
            Some("AUTHTOKENxxxxxxxxxxxxx"),
            "the cancel command must carry the helper's auth token",
        );
    }

    /// The final-state computation itself: a fired token maps to Canceled even
    /// with no cancel_writer present (mirrors the sticky `is_cancelled()` check
    /// in the same-user `run_subprocess`). Complements the test above by
    /// isolating the state mapping from the cancel-delivery side effect.
    #[tokio::test(flavor = "multi_thread")]
    async fn fired_token_maps_exit_zero_to_canceled_without_writer() {
        let cancel_token = CancellationToken::new();
        cancel_token.cancel(); // already cancelled before the child exits

        let mut helper = ScriptedHelper {
            auth_token: "AUTHTOKENxxxxxxxxxxxxx".into(),
            // Skip the pid + pending calls: first response is `exited`.
            reader: MockReader {
                exit_code: 0,
                calls: 2,
            },
        };
        let config = base_config(crate::runner::CancelMethod::Terminate);
        let mut filter = crate::action_filter::ActionFilter::new("test", true, false);
        let (msg_tx, _msg_rx) = tokio::sync::mpsc::unbounded_channel();

        let result = run_via_helper(
            &mut helper,
            &config,
            &mut filter,
            "test",
            msg_tx,
            None,
            &cancel_token,
        )
        .await
        .expect("run_via_helper should not error");

        assert_eq!(result.state, ActionState::Canceled);
    }
}
