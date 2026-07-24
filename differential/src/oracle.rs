// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! The Python oracle: a long-lived worker process queried over a
//! newline-delimited JSON protocol.
//!
//! Startup (interpreter boot + reference import) is paid once; each query is a
//! single `write` + `read_line`. This keeps a corpus of tens of thousands of
//! cases well under a CI budget.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde::{Deserialize, Serialize};

/// A tagged result from either implementation.
///
/// This is the unit the equivalence relation compares. Both the Rust side (via
/// [`from_expr_result`](TaggedResult::from_expr_result)) and the Python worker
/// produce it, so agreement means "same typed value, or both errored".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaggedResult {
    /// Evaluation succeeded. Carries the `openjd-expr` transport tag:
    /// `{"type": "<exprtype>", "value": <string | nested array of strings>}`.
    Ok(TransportTag),
    /// Evaluation raised an expression error. The category is best-effort and
    /// used for triage/logging only — it is NOT part of the equivalence
    /// decision (Python and Rust categorize errors differently; see
    /// [`crate::equivalence::compare`]).
    Err { category: String },
    /// The Rust evaluator *panicked* (e.g. arithmetic overflow under
    /// overflow-checks). Only ever produced by the Rust side — the Python
    /// worker turns any non-`ExpressionError` into `internal_error`, which the
    /// harness skips. A panic is always a divergence against Python.
    Panicked,
}

/// The `{"type", "value"}` transport tag, mirrored on both sides.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportTag {
    #[serde(rename = "type")]
    pub type_str: String,
    pub value: serde_json::Value,
}

impl TaggedResult {
    /// Build a tag from a Rust `openjd-expr` evaluation result.
    ///
    /// Uses the crate's own [`openjd_expr::ExprValue::to_json_transport`], so
    /// the emitted shape is guaranteed identical to what the Python worker
    /// constructs — the comparison is then a plain structural equality on
    /// `(type_str, value)`.
    pub fn from_expr_result(
        result: Result<openjd_expr::ExprValue, openjd_expr::ExpressionError>,
    ) -> Self {
        match result {
            Ok(v) => {
                let tag = v.to_json_transport();
                TaggedResult::Ok(TransportTag {
                    type_str: tag
                        .get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    value: tag.get("value").cloned().unwrap_or(serde_json::Value::Null),
                })
            }
            Err(e) => TaggedResult::Err {
                category: rust_error_category(&e),
            },
        }
    }
}

/// Coarse category for a Rust expression error — triage/logging only, never
/// used for pass/fail (mirrors the Python worker's `_error_category`).
fn rust_error_category(e: &openjd_expr::ExpressionError) -> String {
    use openjd_expr::ExpressionErrorKind::*;
    match e.kind() {
        IntegerOverflow => "overflow",
        DivisionByZero { .. } => "division_by_zero",
        UndefinedVariable { .. } => "undefined_symbol",
        ParseError(_) | UnsupportedSyntax { .. } => "parse",
        IndexOutOfBounds { .. } => "index",
        TypeError { .. } | UnknownFunction { .. } => "type",
        _ => "other",
    }
    .to_string()
}

/// A single request to the worker.
#[derive(Debug, Serialize)]
struct Request<'a> {
    expr: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<&'a serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_format: Option<&'a str>,
}

/// The raw worker response before it is normalized into a [`TaggedResult`].
#[derive(Debug, Deserialize)]
struct RawResponse {
    ok: Option<TransportTag>,
    err: Option<ErrBody>,
    internal_error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ErrBody {
    category: String,
}

/// An answer from the Python side. `Internal` means the *oracle* faulted (an
/// unexpected non-`ExpressionError` in the reference or a protocol error) — the
/// case must be skipped, not counted as a divergence, so that oracle bugs never
/// masquerade as Rust bugs.
#[derive(Debug, Clone)]
pub enum OracleAnswer {
    Result(TaggedResult),
    Internal(String),
}

/// A long-lived Python oracle worker.
pub struct Oracle {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    reference_src: PathBuf,
}

impl Oracle {
    /// Spawn the worker, importing the Python reference from `reference_src`
    /// (the reference checkout's `src/` directory) via `PYTHONPATH`.
    pub fn spawn(reference_src: impl AsRef<Path>) -> std::io::Result<Self> {
        let reference_src = reference_src.as_ref().to_path_buf();
        let worker = worker_script_path();
        let mut child = Command::new(python_bin())
            .arg(&worker)
            .env("PYTHONPATH", &reference_src)
            // Unbuffered so single-line responses arrive without a flush race.
            .env("PYTHONUNBUFFERED", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = child.stdin.take().expect("piped stdin");
        let stdout = BufReader::new(child.stdout.take().expect("piped stdout"));
        Ok(Oracle {
            child,
            stdin,
            stdout,
            reference_src,
        })
    }

    /// Spawn using the reference location resolved from the environment or the
    /// conventional side-by-side checkout. See [`resolve_reference_src`].
    pub fn spawn_default() -> std::io::Result<Self> {
        Self::spawn(resolve_reference_src())
    }

    /// The reference `src/` this oracle imports from (for provenance logging).
    pub fn reference_src(&self) -> &Path {
        &self.reference_src
    }

    /// Evaluate one expression on the Python side.
    ///
    /// `symbols` is an optional JSON object mapping (possibly dotted) names to
    /// scalar values — the same shape the Rust adapter builds its `SymbolTable`
    /// from, so both sides see identical bindings. `path_format` is `"POSIX"`,
    /// `"WINDOWS"`, or `None` for host-native.
    pub fn eval_expr(
        &mut self,
        expr: &str,
        symbols: Option<&serde_json::Value>,
        path_format: Option<&str>,
    ) -> std::io::Result<OracleAnswer> {
        let req = Request {
            expr,
            symbols,
            path_format,
        };
        let line = serde_json::to_string(&req).expect("request serializes");
        self.stdin.write_all(line.as_bytes())?;
        self.stdin.write_all(b"\n")?;
        self.stdin.flush()?;

        let mut resp_line = String::new();
        let n = self.stdout.read_line(&mut resp_line)?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "oracle worker closed stdout (crashed?)",
            ));
        }
        let raw: RawResponse = serde_json::from_str(resp_line.trim()).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("malformed worker response {resp_line:?}: {e}"),
            )
        })?;
        Ok(match raw {
            RawResponse {
                internal_error: Some(msg),
                ..
            } => OracleAnswer::Internal(msg),
            RawResponse { ok: Some(tag), .. } => OracleAnswer::Result(TaggedResult::Ok(tag)),
            RawResponse {
                err: Some(body), ..
            } => OracleAnswer::Result(TaggedResult::Err {
                category: body.category,
            }),
            other => OracleAnswer::Internal(format!("empty worker response: {other:?}")),
        })
    }
}

impl Drop for Oracle {
    fn drop(&mut self) {
        // Close stdin so the worker's `for line in sys.stdin` loop ends, then
        // reap. Best-effort: a dead worker is fine at teardown.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn python_bin() -> String {
    std::env::var("OPENJD_DIFF_PYTHON").unwrap_or_else(|_| "python3".to_string())
}

fn worker_script_path() -> PathBuf {
    // The worker ships alongside this crate; resolve from CARGO_MANIFEST_DIR so
    // it works regardless of the cwd the test runner uses.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("oracle")
        .join("oracle_worker.py")
}

/// Resolve the Python reference `src/` directory.
///
/// Order: `OPENJD_PYTHON_REF_SRC` env var (CI sets this explicitly), else the
/// conventional side-by-side checkout `../../openjd-model-for-python/src`
/// relative to this crate (the layout the `eval-crate` skill sets up).
pub fn resolve_reference_src() -> PathBuf {
    if let Ok(p) = std::env::var("OPENJD_PYTHON_REF_SRC") {
        return PathBuf::from(p);
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("openjd-model-for-python")
        .join("src")
}
