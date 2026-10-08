# Changelog

All notable changes to this crate are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.13.1](https://github.com/crowecawcaw/openjd-rs/releases/tag/python-openjd-sessions-v0.13.1) - 2026-10-08

### Bug fixes

- Render a PATH parameter in the host's path format (OpenJobDescription/openjd-sessions-for-python#364) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Raise openjd-model floor to >= 0.11.4 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Do not cache failed command lookups ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Address automated review findings on the trusted-path resolver ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Resolve system commands from trusted dirs, not PATH ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Use absolute paths for system commands to prevent PATH injection ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Forward step_name through the _v1 Session.run_task wrapper (OpenJobDescription/openjd-sessions-for-python#345) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Do not load the native extension to build an empty rules list ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Decode subprocess output with errors=backslashreplace (OpenJobDescription/openjd-sessions-for-python#343) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Resolve a legacy non-string argument instead of crashing ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Do not load the native EXPR extension unless EXPR is used ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Send the kill signal before announcing it ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- A failed launch must release wait_until_started() ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Run() must own its child from creation, not after logging ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- WrappedAction.Environment must be session-lifetime (RFC 0008 MUST) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Give _materialize_path_mapping a failure path (openjd-rs parity) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Cancel_info.json handler caught the wrong exception type ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Do not gate terminate on leader liveness (openjd-rs parity) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Anchor the openjd_env near-miss regex (openjd-rs parity) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Poll through CANCELING state to deliver terminal callback (OpenJobDescription/openjd-sessions-for-python#331) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Only allow basenames for embedded files (OpenJobDescription/openjd-sessions-for-python#326) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Notify and cancel windows (OpenJobDescription/openjd-sessions-for-python#245) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Sdist failed to install (OpenJobDescription/openjd-sessions-for-python#240) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Cleanup session dir on windows (OpenJobDescription/openjd-sessions-for-python#241) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Use default timeout of 5 minutes for environment exits (OpenJobDescription/openjd-sessions-for-python#213) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Unhandled exception in cancellation workflow (OpenJobDescription/openjd-sessions-for-python#186) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Run Windows Session cleanup at high priority. (OpenJobDescription/openjd-sessions-for-python#173) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Run python instead of pythonservice on windows to signal subprocesses (OpenJobDescription/openjd-sessions-for-python#171) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Session stuck reading on STDOUT stream (OpenJobDescription/openjd-sessions-for-python#162) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Upper case all env vars on Windows (OpenJobDescription/openjd-sessions-for-python#161) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Ensure process exit code is a 32-bit signed integer (OpenJobDescription/openjd-sessions-for-python#148) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Windows locate_executable finds wrong binary to run (OpenJobDescription/openjd-sessions-for-python#141) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Eliminate TranslateName usage on Windows systems (OpenJobDescription/openjd-sessions-for-python#144) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Failing to parse openjd_env and openjd_unset_env should fail session action (OpenJobDescription/openjd-sessions-for-python#111) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Restrict handles inherited by win32 subprocess (OpenJobDescription/openjd-sessions-for-python#112) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Remove methods from public interface of WindowsSessionUser (OpenJobDescription/openjd-sessions-for-python#91) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Make tempdir create parent dir if nonexistent (OpenJobDescription/openjd-sessions-for-python#86) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Allow openjd_env to set vars to empty (OpenJobDescription/openjd-sessions-for-python#74) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Change default windows working directory to the "C:\ProgramData\Amazon\OpenJD" (OpenJobDescription/openjd-sessions-for-python#63) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add logging for setting environment variables (OpenJobDescription/openjd-sessions-for-python#57) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Parameter name for signal_win_process (OpenJobDescription/openjd-sessions-for-python#40) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Properly delete working dir with Windows impersonation (OpenJobDescription/openjd-sessions-for-python#35) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Make psutil a runtime dependency on Windows (OpenJobDescription/openjd-sessions-for-python#36) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Remove embedded_files.write_file_for_user Windows exception (OpenJobDescription/openjd-sessions-for-python#32) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Make tempdir permissions inherited by descendants on Windows (OpenJobDescription/openjd-sessions-for-python#29) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Package missing signal subprocess shell script (OpenJobDescription/openjd-sessions-for-python#24) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Exporting WindowsSessionUser class (OpenJobDescription/openjd-sessions-for-python#22) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Use psutil to kill the process instead of using taskkill. (OpenJobDescription/openjd-sessions-for-python#18) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Properly cleanup working dir with posix cross-user (OpenJobDescription/openjd-sessions-for-python#13) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Updates to path mapping ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Allow subprocess user to be the current user (OpenJobDescription/openjd-sessions-for-python#6) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Remove misleading 'rm' error message (OpenJobDescription/openjd-sessions-for-python#10) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### CI

- Specify permissions that workflows pass to jobs/actions (OpenJobDescription/openjd-sessions-for-python#287) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add changelog section for performance improvements (OpenJobDescription/openjd-sessions-for-python#279) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add pr labeling (OpenJobDescription/openjd-sessions-for-python#258) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Update release workflows (OpenJobDescription/openjd-sessions-for-python#244) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add codeql analysis (OpenJobDescription/openjd-sessions-for-python#158) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add PyPI publish job to publish workflow (OpenJobDescription/openjd-sessions-for-python#127) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add reusable workflows (OpenJobDescription/openjd-sessions-for-python#123) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Change publish project name (OpenJobDescription/openjd-sessions-for-python#122) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Mark test that fails in CodeBuild as xfail (OpenJobDescription/openjd-sessions-for-python#104) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Set release to watch mainline changelog (OpenJobDescription/openjd-sessions-for-python#95) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Remove merge back from release workflow (OpenJobDescription/openjd-sessions-for-python#90) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### Documentation

- Document the Python packages in openjd-rs ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Lower-case the remaining 0.12.0 changelog entry ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Mark the 0.12.0 entry as a breaking release ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Reframe resolver comments around problem and solution ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Explain the ignored ChildProcessError in a test cleanup ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add openjd-rs cross-port checklist to PR template (OpenJobDescription/openjd-sessions-for-python#312) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Python 3.13 and 3.14 (OpenJobDescription/openjd-sessions-for-python#292) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Enhance contributing guidelines to be more accessible (OpenJobDescription/openjd-sessions-for-python#168) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Fix minor issues (OpenJobDescription/openjd-sessions-for-python#75) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### Features

- [**breaking**] Require openjd-model 0.13 (OpenJobDescription/openjd-sessions-for-python#368) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Accept a resolved symbol table on the v0 session (OpenJobDescription/openjd-sessions-for-python#357) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Shorten the session working directory name for Windows MAX_PATH (OpenJobDescription/openjd-sessions-for-python#348) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Support running Sessions as a jobRunAsUser on macOS (OpenJobDescription/openjd-sessions-for-python#335) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- RFC 0008 environment wrap actions (OpenJobDescription/openjd-sessions-for-python#333) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Rust-backed openjd.sessions._v1 via PyO3 (OpenJobDescription/openjd-sessions-for-python#316) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Enable Claude PR review ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add support for domain users (OpenJobDescription/openjd-sessions-for-python#311) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add OPENJD_SESSION_WORKING_DIR environment variable (OpenJobDescription/openjd-sessions-for-python#309) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Implement FEATURE_BUNDLE_1 RFC 0004 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add run_subprocess function to the Session class ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Adding support for redacted environment variable values through… (OpenJobDescription/openjd-sessions-for-python#232) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Allow opt-in of running tasks after an env exit ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add ability to not log banner when running a task (OpenJobDescription/openjd-sessions-for-python#204) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Directly signal session action processes using CAP_KILL (OpenJobDescription/openjd-sessions-for-python#196) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Update openjd-model to 0.5.* (OpenJobDescription/openjd-sessions-for-python#194) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add log content metadata to log records (OpenJobDescription/openjd-sessions-for-python#175) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Introduce log content on log records (OpenJobDescription/openjd-sessions-for-python#170) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Log process return code on process exit (OpenJobDescription/openjd-sessions-for-python#150) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Support for multi line env variables in enter env (OpenJobDescription/openjd-sessions-for-python#115) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Resolve windows command location prior to run (OpenJobDescription/openjd-sessions-for-python#116) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Remove group property from WindowsSessionUser (OpenJobDescription/openjd-sessions-for-python#102) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Sessions can now be run in a Windows Service context (OpenJobDescription/openjd-sessions-for-python#97) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Public release (OpenJobDescription/openjd-sessions-for-python#80) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Update to openjd-model 0.3.0 (OpenJobDescription/openjd-sessions-for-python#73) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add option to supply location to create Working Directory (OpenJobDescription/openjd-sessions-for-python#56) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Unify parameter data shapes with openjd-model (OpenJobDescription/openjd-sessions-for-python#55) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Differentiate canceled/timed-out actions (OpenJobDescription/openjd-sessions-for-python#54) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Validate username and password in Windows. (OpenJobDescription/openjd-sessions-for-python#48) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Reuse ParameterValueType from model package (OpenJobDescription/openjd-sessions-for-python#49) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Modify logging to be easier to understand (OpenJobDescription/openjd-sessions-for-python#43) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Allow adding env vars when running an action (OpenJobDescription/openjd-sessions-for-python#42) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Support notify feature on Windows. (OpenJobDescription/openjd-sessions-for-python#28) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Export package version (OpenJobDescription/openjd-sessions-for-python#31) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Support session.cleanup() on Windows (OpenJobDescription/openjd-sessions-for-python#26) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Support impersonation in tempdir permissions (OpenJobDescription/openjd-sessions-for-python#21) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Change the Start-Process to Start-Job to support impersonation. (OpenJobDescription/openjd-sessions-for-python#17) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add Windows session user (OpenJobDescription/openjd-sessions-for-python#16) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Import Windows implementation from internal repository (OpenJobDescription/openjd-sessions-for-python#12) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Import from internal repository (OpenJobDescription/openjd-sessions-for-python#1) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### Miscellaneous

- Openjd-model 0.14.0, openjd-sessions 0.13.1, openjd-cli 0.7.8 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.13.0 (OpenJobDescription/openjd-sessions-for-python#369) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.12.1 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump actions/checkout from 4 to 7 (OpenJobDescription/openjd-sessions-for-python#349) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.12.0 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Pin GitPython for release bump ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Raise the openjd-model floor to 0.11.6 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.11.0 (OpenJobDescription/openjd-sessions-for-python#355) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.14 (OpenJobDescription/openjd-sessions-for-python#347) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.13 (OpenJobDescription/openjd-sessions-for-python#346) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.12 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.11 (OpenJobDescription/openjd-sessions-for-python#340) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Group dependabot PRs (OpenJobDescription/openjd-sessions-for-python#338) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.10 (OpenJobDescription/openjd-sessions-for-python#327) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump actions/checkout from 6 to 7 (OpenJobDescription/openjd-sessions-for-python#317) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.9 (OpenJobDescription/openjd-sessions-for-python#313) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump dependabot/fetch-metadata from 2 to 3 (OpenJobDescription/openjd-sessions-for-python#308) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.8 (OpenJobDescription/openjd-sessions-for-python#310) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Pin virtualenv<21 for python 3.9 (OpenJobDescription/openjd-sessions-for-python#305) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Revert "chore: temporary pin virtualenv<21 to fix build (OpenJobDescription/openjd-sessions-for-python#303)" (OpenJobDescription/openjd-sessions-for-python#304) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Temporary pin virtualenv<21 to fix build (OpenJobDescription/openjd-sessions-for-python#303) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.7 (OpenJobDescription/openjd-sessions-for-python#301) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Update changelog description ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.7 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.6 (OpenJobDescription/openjd-sessions-for-python#290) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump actions/checkout from 5 to 6 (OpenJobDescription/openjd-sessions-for-python#286) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- As of hatch 1.16.0, 'hatch build' can't be run in non-builder envs (OpenJobDescription/openjd-sessions-for-python#288) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.5 (OpenJobDescription/openjd-sessions-for-python#283) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add private _run_task_without_session_env function to the Session class (OpenJobDescription/openjd-sessions-for-python#281) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Revert "chore: pin the version of click that hatch uses to <8.… (OpenJobDescription/openjd-sessions-for-python#273) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump actions/setup-python from 5 to 6 (OpenJobDescription/openjd-sessions-for-python#266) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Pin the version of click that hatch uses to <8.3 due to Sentin… (OpenJobDescription/openjd-sessions-for-python#272) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add responded and stale issue/pr workflows (OpenJobDescription/openjd-sessions-for-python#263) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Enable patch releases (OpenJobDescription/openjd-sessions-for-python#262) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump actions/checkout from 4 to 5 (OpenJobDescription/openjd-sessions-for-python#259) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.4 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Update openpgp key documentation (OpenJobDescription/openjd-sessions-for-python#254) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Change publish to pypi to use tag ref instead of release branch. (OpenJobDescription/openjd-sessions-for-python#252) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Allow zero version when releasing ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.3 (OpenJobDescription/openjd-sessions-for-python#242) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.2 (OpenJobDescription/openjd-sessions-for-python#230) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add internal toggle to disable Running action banner (OpenJobDescription/openjd-sessions-for-python#229) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.1 ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.10.0 (OpenJobDescription/openjd-sessions-for-python#219) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- [**breaking**] Update to new openjd-model version and cleanup warnings ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add maintenance label to maintenance GitHub template (OpenJobDescription/openjd-sessions-for-python#211) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Update GitHub issue templates (OpenJobDescription/openjd-sessions-for-python#205) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.9.1 (OpenJobDescription/openjd-sessions-for-python#199) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Upgrade test containers from buster to bookworm (OpenJobDescription/openjd-sessions-for-python#178) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.9.0 (OpenJobDescription/openjd-sessions-for-python#195) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.8.4 (OpenJobDescription/openjd-sessions-for-python#176) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.8.3 (OpenJobDescription/openjd-sessions-for-python#174) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.8.2 (OpenJobDescription/openjd-sessions-for-python#166) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.8.2 (OpenJobDescription/openjd-sessions-for-python#163) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.8.1 (OpenJobDescription/openjd-sessions-for-python#155) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.8.1 (OpenJobDescription/openjd-sessions-for-python#153) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.8.0 (OpenJobDescription/openjd-sessions-for-python#146) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.7.2 (OpenJobDescription/openjd-sessions-for-python#121) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump dependabot/fetch-metadata from 1 to 2 (OpenJobDescription/openjd-sessions-for-python#118) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Remove experimental warning for windows (OpenJobDescription/openjd-sessions-for-python#120) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.7.1 (OpenJobDescription/openjd-sessions-for-python#119) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.7.1 (OpenJobDescription/openjd-sessions-for-python#114) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.7.0 (OpenJobDescription/openjd-sessions-for-python#110) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.6.1 (OpenJobDescription/openjd-sessions-for-python#105) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.6.0 (OpenJobDescription/openjd-sessions-for-python#103) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.5.1 (OpenJobDescription/openjd-sessions-for-python#96) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.5.1 (OpenJobDescription/openjd-sessions-for-python#93) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add semantic commit parser options (OpenJobDescription/openjd-sessions-for-python#89) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add windows embedded_files impersonation tests (OpenJobDescription/openjd-sessions-for-python#77) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add Windows subprocess impersonation tests (OpenJobDescription/openjd-sessions-for-python#72) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.5.0 (OpenJobDescription/openjd-sessions-for-python#85) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Publish to PyPI (OpenJobDescription/openjd-sessions-for-python#84) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Backfill the CHANGELOG (OpenJobDescription/openjd-sessions-for-python#83) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.4.0 (OpenJobDescription/openjd-sessions-for-python#82) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Sign GitHub release artifacts with gpg (OpenJobDescription/openjd-sessions-for-python#79) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Use win apis to run process (OpenJobDescription/openjd-sessions-for-python#58) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add gpg verification instructions (OpenJobDescription/openjd-sessions-for-python#78) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Fix release workflow (OpenJobDescription/openjd-sessions-for-python#76) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add new release workflow (OpenJobDescription/openjd-sessions-for-python#67) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Enable macOS in CI (OpenJobDescription/openjd-sessions-for-python#61) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Flesh out the initial README (OpenJobDescription/openjd-sessions-for-python#50) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Update codeowners file (OpenJobDescription/openjd-sessions-for-python#47) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump actions/setup-python from 4 to 5 (OpenJobDescription/openjd-sessions-for-python#39) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add Customer Domain Env to Release Workflow (OpenJobDescription/openjd-sessions-for-python#25) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump actions/checkout from 3 to 4 (OpenJobDescription/openjd-sessions-for-python#2) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Bump aws-actions/configure-aws-credentials from 2 to 4 (OpenJobDescription/openjd-sessions-for-python#11) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add publishing (OpenJobDescription/openjd-sessions-for-python#9) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### Refactor

- Put every openjd.expr crossing behind the one sys.modules guard ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Updating openjd-model dependency version and tests to use F… (OpenJobDescription/openjd-sessions-for-python#233) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### Testing

- Make the "Trapped" signal handler signal-safe ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Use posixpath.isabs for the POSIX trusted-directory entries ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Wrap a RANGE_EXPR parameter in string() before repr_sh ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Pin wrapper-before-symlink-farm ordering on NixOS ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Pin the trusted-path resolver's security properties ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Pin the observable state the ownership fix changed ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Copyright test failing on new comment by setuptools-scm (OpenJobDescription/openjd-sessions-for-python#307) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Cancel notify - message immediately after 'Trapped' may not be digit (OpenJobDescription/openjd-sessions-for-python#294) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add timestamp for captured logging (OpenJobDescription/openjd-sessions-for-python#293) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add Python versions 3.13 and 3.14 to the github action test matrix ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Timezone information was dropped when parsing the cancel NotifyEnd (OpenJobDescription/openjd-sessions-for-python#261) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Testing containers failing to build (OpenJobDescription/openjd-sessions-for-python#243) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Broaden regex to ignore copyright header file created by setupt… (OpenJobDescription/openjd-sessions-for-python#216) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Upgrade docker testing compatibility to Docker 25.x (OpenJobDescription/openjd-sessions-for-python#172) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Fix test where ping is not available (OpenJobDescription/openjd-sessions-for-python#164) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add cross-user Tempdir tests on Windows (OpenJobDescription/openjd-sessions-for-python#68) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add cross-user Session tests on Windows (OpenJobDescription/openjd-sessions-for-python#66) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add test_runner_base windows impersonation tests (OpenJobDescription/openjd-sessions-for-python#65) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Add test user in Windows github actions (OpenJobDescription/openjd-sessions-for-python#62) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Fix missing `assert` in the test_session.py (OpenJobDescription/openjd-sessions-for-python#64) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Increase test_run_action timeout on Windows (OpenJobDescription/openjd-sessions-for-python#27) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### Build

- [**breaking**] Build the Python packages from the Cargo workspace ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### Revert

- "chore(release): 0.8.2 (OpenJobDescription/openjd-sessions-for-python#163)" (OpenJobDescription/openjd-sessions-for-python#165) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- "chore(release): 0.8.1" (OpenJobDescription/openjd-sessions-for-python#154) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- Ensure process exit code is a 32-bit signed integer (OpenJobDescription/openjd-sessions-for-python#149) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))

- 0.5.1 (OpenJobDescription/openjd-sessions-for-python#94) ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


### Style

- Reformat with black 26.x ([#4](https://github.com/OpenJobDescription/openjd-rs/pull/4))


## [0.13.1](https://github.com/OpenJobDescription/openjd-rs/releases/tag/python-openjd-sessions-v0.13.1) - 2026-10-08

### Miscellaneous

- Require openjd-model 0.14.
- Move the package from openjd-sessions-for-python into the openjd-rs repository, under `python/openjd-sessions`. The version now comes from the package's `Cargo.toml`.

## 0.13.0 (2026-10-07)


### Features
* Require openjd-model 0.13 (#368) ([`dfa8c75`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/dfa8c758d4526ee1b8e253d710a6b475854a3661))



## 0.12.1 (2026-09-04)



### Bug Fixes
* Render a PATH parameter in the host's path format (#364) ([`43b2640`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/43b26407f2f831c7e8f428649e63e4b7f97d1e75))
* Render a PATH parameter in the host's path format ([`43b2640`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/43b26407f2f831c7e8f428649e63e4b7f97d1e75))
* Address review on the PATH host-format change ([`43b2640`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/43b26407f2f831c7e8f428649e63e4b7f97d1e75))


## 0.12.0 (2026-08-26)


### ⚠ BREAKING CHANGES
* `extra_let_bindings` is removed from `Session.enter_environment`, `Session.exit_environment` and `Session.run_task` (#357). The parameter was public in 0.11.0. Callers must deliver step-scope EXPR `let` values through the new `resolved_symtab` parameter — the resolved symbol table the service already produces per step and per environment — instead. `step_name` is unchanged on both methods. ([`457a364`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/457a364371cb44bf5a6253e612a451e5678621d3))


### Features
* accept a resolved symbol table on the v0 session (#357) ([`457a364`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/457a364371cb44bf5a6253e612a451e5678621d3))
* deliver step-scope let bindings to run_task — added `extra_let_bindings` to `run_task`, then superseded within this same release; see BREAKING CHANGES above ([`457a364`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/457a364371cb44bf5a6253e612a451e5678621d3))
* accept resolved symbol table on v0 session ([`457a364`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/457a364371cb44bf5a6253e612a451e5678621d3))

### Bug Fixes
* drain exit step context before failures ([`457a364`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/457a364371cb44bf5a6253e612a451e5678621d3))
* seed the resolved base into wrap hook scope ([`457a364`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/457a364371cb44bf5a6253e612a451e5678621d3))
* replay a wrap env's own base into hook scope ([`457a364`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/457a364371cb44bf5a6253e612a451e5678621d3))
* type the hook-scope capture stand-in exactly ([`457a364`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/457a364371cb44bf5a6253e612a451e5678621d3))


## 0.11.0 (2026-08-20)

### ⚠ BREAKING CHANGES 

* shorten the session working directory name for Windows MAX_PATH — session working dir is no longer prefixed with session ID; `embedded_files<random>` renamed to `ef<random>` (#348) 

### Bug Fixes 

* resolve system commands from trusted dirs, not PATH (#351) 
* do not cache failed command lookups (#351)
* address automated review findings on the trusted-path resolver (#351)

## 0.10.14 (2026-08-11)

### Features
* support running Sessions as a jobRunAsUser on macOS (#335) ([`2df2d99`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/2df2d999e359d5a14d333d9948209c3a1634ab44))


## 0.10.13 (2026-08-06)



### Bug Fixes
* forward step_name through the _v1 Session.run_task wrapper (#345) ([`9ca3a4a`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/9ca3a4ab58951c655d63e8db0190fe818e00d621))


## 0.10.12 (2026-08-04)



### Bug Fixes
* Do not load the native extension to build an empty rules list ([`80b1e70`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/80b1e70bd1b765a74abc7ae8eb37fa24c4056300))
* decode subprocess output with errors=backslashreplace (#343) ([`e2e60d3`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/e2e60d39a5af6f8c948382b08ac4c175be216752))
* Resolve a legacy non-string argument instead of crashing ([`748629f`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/748629fbaa4dcb0b82b04a46f086831abbd449d5))
* Do not load the native EXPR extension unless EXPR is used ([`5c546b2`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/5c546b2e22e5cb9fe86eb61f4bbab09fabab6a8f))
* Send the kill signal before announcing it ([`7bcac6e`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/7bcac6e26047e4c8109611bcf7cc11fe8e26a0ff))
* A failed launch must release wait_until_started() ([`f160a74`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/f160a74904419e282d73e3dcfd0773e427f8e85a))
* run() must own its child from creation, not after logging ([`5e73e7f`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/5e73e7ff5daa40b1cfc6eb6edc5acb25e0487984))
* WrappedAction.Environment must be session-lifetime (RFC 0008 MUST) ([`26655b4`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/26655b484768632041b7a712a078e6e5178f4b40))
* Give _materialize_path_mapping a failure path (openjd-rs parity) ([`7585d4d`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/7585d4d4ae418fc76f203365d6dc1d5d7a349f43))
* cancel_info.json handler caught the wrong exception type ([`33cb7f0`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/33cb7f03de689962d44e53ad8333d080de19b15f))
* Do not gate terminate on leader liveness (openjd-rs parity) ([`4ffa320`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/4ffa320686c94e9cf133a3f8fecdd90353d3b681))
* Anchor the openjd_env near-miss regex (openjd-rs parity) ([`b4ef573`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/b4ef573e89606ab599f477ed3fa131074d0faf45))


## 0.10.11 (2026-07-28)


### Features
* RFC 0008 environment wrap actions (#333) ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* RFC 0008 environment wrap actions with EXPR runtime parity Implement the WRAP_ACTIONS extension (RFC 0008) in the v0 session, with EXPR (RFC 0007) runtime parity with openjd-rs: - Wrap-hook dispatch: an environment's onWrapEnvEnter / onWrapTaskRun / onWrapEnvExit runs in place of an inner environment's onEnter/onExit or a step's onRun, with WrappedAction.Command/Args/Environment/ Timeout/Cancelation.* and WrappedEnv.Name/WrappedStep.Name injected. At most one wrap-defining environment may be active (enforced at enter time). - Two strictly separated scopes (openjd-rs #277 parity): the wrapped action's values resolve against the INNER entity's own scope (its script-level let bindings and embedded files, materialized in runner order: paths, lets, contents); the hook script resolves against the wrap environment's own scope (its lets, evaluated by the runner) plus the WrappedAction.* overlay. Same-named lets on the two sides each resolve to their own value. - WrappedAction.Environment carries every session-defined variable: openjd_env definitions and entered environments' declarative variables: maps; host-inherited variables are excluded. - EXPR runtime: runner-evaluated script-level let bindings ordered around embedded-file materialization (paths before lets, contents after), typed symbol tables, enter_environment(extra_let_bindings=...) so a step's environments see the step-level lets. - Cancelation: WrappedAction.Cancelation.Mode/NotifyPeriodInSeconds resolved through the enforcement path, with Template Schemas 5.3.2 defaults (120s task onRun / 30s otherwise). ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))

### Bug Fixes
* Address review findings — let-binding crash paths, Step.Name, perf ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* RFC 0005/0008 typed-arg and null-vs-empty parity with openjd-rs ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Close review findings on failure paths, timeout bounds and typing ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Narrow Optional script before setattr in wrap cancelation test ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Close wrap-hook scope gap, cancel races, and path-mapping parity ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Make wrap and URI tests host-independent on Windows ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Do not fail an action whose subprocess exits immediately ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Serialize the pending-cancel handoff against action launch ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* **concurrency**: Close race windows in cancel and completion paths ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* **concurrency**: Close round-4 review findings (R4-1, R4-5, R4-6, R4-G7, R4-G8) ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Close round-5 review findings R5-1 through R5-9 ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Close defects introduced by the round-5 fix commit ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* **wrap-actions**: Isolate a wrap hook's scope from the wrapped entity's ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* **subprocess**: Own the child on every exit path from run() ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Report chown and pgrep failures through the paths that handle them ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Silence Windows mypy on POSIX-only os and signal attributes ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Do not open a directory descriptor on Windows ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* Address the live CodeQL alerts on this PR ([`df96902`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df96902d1721637c99aab6ec460100e11ed0ddb9))
* poll through CANCELING state to deliver terminal callback (#331) ([`d57e477`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/d57e4775518c0659a0749de50f995ef383c8a718))


## 0.10.10 (2026-07-03)


### Features
* **sessions**: Rust-backed openjd.sessions._v1 via PyO3 (#316) ([`06c30f1`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/06c30f15c43ade9ccd8ecbc6f8883bf010fcd2c9))

### Bug Fixes
* only allow basenames for embedded files (#326) ([`0c0bdb5`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/0c0bdb5bbfac1656b9da9b7488877136128cb1b6))


## 0.10.9 (2026-05-25)


### Features
* add support for domain users (#311) ([`5da7006`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/5da7006f50630a90bb57b44b7bfc17ce9a1f5956))



## 0.10.8 (2026-05-11)


### Features
* Add OPENJD_SESSION_WORKING_DIR environment variable (#309) ([`9e28bc5`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/9e28bc520aa62e325cd74fde596c5acb469de1b5))



## 0.10.7 (2026-02-03)


### Features
* Implement [FEATURE_BUNDLE_1 RFC 0004](https://github.com/OpenJobDescription/openjd-specifications/blob/mainline/rfcs/0004-enhanced-limits-and-capabilities.md), increasing limits
  for job parameter counts and name lengths, enabling format strings in integer properties, providing control over embedded file line endings, and adding syntax sugar
  to simplify templates that run simple scripts with common interpreters ([`b9f3b8c`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/b9f3b8cd42b5b0be208fa859956dfd8956d72cb8))


## 0.10.6 (2025-12-08)


### Features
* Add run_subprocess function to the Session class ([`80026e9`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/80026e95c940a530c10e45f1631aaea0d81e9dfe))



## 0.10.5 (2025-11-07)

* Dependencies update, and other non-functional updates

## 0.10.4 (2025-07-22)



### Bug Fixes
* notify and cancel windows (#245) ([`77e5e2b`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/77e5e2bdaaba464610df521cb7846cdd1f5a333d))

## 0.10.3 (2025-06-05)


### Features
* Adding support for redacted environment variable values through openjd_redacted_env (#232) ([`b949a16`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/b949a16ee71866845537923b9ea4799c59a629e0))

### Bug Fixes
* sdist failed to install (#240) ([`c855d29`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/c855d296c361f5193f02f71682d9a071575fad76))
* cleanup session dir on windows (#241) ([`f1f8933`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/f1f89335ea909be097faa164a57d7429a8acf018))

## 0.10.2 (2025-04-30)

* Dependencies update, and other non-functional updates


## 0.10.1 (2025-03-03)


### Features
* Allow opt-in of running tasks after an env exit ([`bb75463`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/bb75463a6ee673f0f7e6d80af789e50c157f4872))


## 0.10.0 (2025-02-25)

### BREAKING CHANGES
* When a timeout is not specified for an environment exit action, it now has a timeout of 300 seconds or five minutes. Before this change, environment exit actions would run indefinintely. To have long-running environment exit actions, job templates can specify a large timeout value when defining the action in a job or environment template.
* The dependency openjd-model-for-python updated from Pydantic V1 to V2, see its [release notes](https://github.com/OpenJobDescription/openjd-model-for-python/releases/tag/0.6.0) for details.

### Features
* add ability to not log banner when running a task (#204) ([`e8acc8a`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/e8acc8a761e5712c9fc436d94e0935fbbf616184))

### Bug Fixes
* use default timeout of 5 minutes for environment exits (#213) ([`8bf93a0`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/8bf93a043b3bcb68eb552fa01bf8e92c02cdeaa7))

## 0.9.1 (2024-12-12)


### Features
* directly signal session action processes using CAP_KILL (#196) ([`84008be`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/84008be79e80cdd9b06095933ea0c58baee89c92))


## 0.9.0 (2024-11-13)


### Features
* **deps**: update openjd-model to 0.5.* (#194) ([`61bac54`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/61bac54274328c8629c4ca2abe3c74d807ffbacd))

### Bug Fixes
* unhandled exception in cancellation workflow (#186) ([`f8a9a48`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/f8a9a4825bc5f049f15502e1e6c423bf3a21c077))

## 0.8.4 (2024-09-20)


### Features
* add log content metadata to log records (#175) ([`a50e94f`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/a50e94f1ed3bc8fc72093aaa5f06e6082a45b060))


## 0.8.3 (2024-09-19)


### Features
* introduce log content on log records (#170) ([`6e98f16`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/6e98f16d8506893f82d7a7f4dbc217874d0ae1e9))

### Bug Fixes
* Run Windows Session cleanup at high priority. (#173) ([`6fb09b5`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/6fb09b53c2addd9ddb2607d4191ae92b36247789))
* run python instead of pythonservice on windows to signal subprocesses (#171) ([`186b2a7`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/186b2a7cf90ae6728decd75fc2d20e8c035c9b12))

## 0.8.2 (2024-08-12)



### Bug Fixes
* Session stuck reading on STDOUT stream (#162) ([`d30595f`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/d30595fdd8fcce12fb4f7a6ca5408f62e8e310e3))
* upper case all env vars on Windows (#161) ([`de03049`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/de030495c9a4af07a6a2ed25bdbe7675443db85e))

## 0.8.1 (2024-06-24)


### Features
* log process return code on process exit (#150) ([`c716e09`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/c716e090356a3cc11343175d34696e346d2ec816))

## 0.8.0 (2024-06-17)

### BREAKING CHANGES
* Windows locate_executable finds wrong binary to run (#141) ([`03defd9`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/03defd98e333eb08edf8dc64536165c9cdb10115))
  * Adds a new requirement that when impersonating a user for subprocesses, the Python installation hosting the library can be run by the impersonated user as well.


### Bug Fixes
* eliminate TranslateName usage on Windows systems (#144) ([`f27b422`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/f27b422f91905f9a93dd83c974c45a56de703905))

## 0.7.2 (2024-04-16)

### Documenation
* Windows is no longer marked as experimental in documentation.


## 0.7.1 (2024-03-25)


### Features
* Support for multi line env variables in enter env (#115) ([`96e02ae`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/96e02ae2c7502e7019a1bc0600444020564c0fdc))
* resolve windows command location prior to run (#116) ([`69f72e3`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/69f72e35b7c169d2f98295fde80cc1f1cce7008d))

### Bug Fixes
* Failing to parse openjd_env and openjd_unset_env should fail session action (#111) ([`8576a73`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/8576a732011e32deb8151a88e309be3fa970a241))
* restrict handles inherited by win32 subprocess (#112) ([`aba3071`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/aba3071439b42cb09194718b84ceee7780206c36))

## 0.7.1 (2024-03-19)



### Bug Fixes
* Failing to parse openjd_env and openjd_unset_env should fail session action (#111) ([`8576a73`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/8576a732011e32deb8151a88e309be3fa970a241))
* restrict handles inherited by win32 subprocess (#112) ([`aba3071`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/aba3071439b42cb09194718b84ceee7780206c36))

## 0.7.0 (2024-03-11)

### BREAKING CHANGES
* remove group property from WindowsSessionUser (#102) ([`5fa8bf2`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/5fa8bf20df4f868995c30bea94006e7e542265e9))



## 0.6.1 (2024-03-05)

This release does not contain any functional changes. It is functionally identical to 0.6.0.
This release was only made to fix an issue with the tests in our internal systems.


## 0.6.0 (2024-03-05)

### BREAKING CHANGES
* remove methods from public interface of WindowsSessionUser (#91) ([`788a503`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/788a50356b293dd496669c4fc71ef752fb90e333))

### Features
* Sessions can now be run in a Windows Service context (#97) ([`72ff65b`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/72ff65b385bee48236014268993b83e2fd7c87a3))


## 0.5.1 (2024-02-26)



### Bug Fixes
* Make tempdir create parent dir if nonexistent (#86) ([`243f4b7`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/243f4b7693c19f3397f8598d8728c2eaf0881957))

## 0.5.0 (2024-02-13)

### BREAKING CHANGES
* public release (#80) ([`86ef7a7`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/86ef7a757f5c42755a455cae1b26143cbc337e39))



## 0.4.0 (2024-02-12)

### BREAKING CHANGES
* update to openjd-model 0.3.0 (#73) ([`719a8ff`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/719a8ff4ebf92b4ab5f1811d67991a59d6166d4c))
* unify parameter data shapes with openjd-model (#55) ([`d52c208`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/d52c208eab6836030de11f1fa3aeaa9d6d0e9a57))
* differentiate canceled/timed-out actions (#54) ([`658620b`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/658620bb2c3a6a6d6cfc028c0b204607ae8e5ce0))

### Features
* add option to supply location to create Working Directory (#56) ([`df72089`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df72089b45fe48a313b40decb24a8147b6bd216c))

### Bug Fixes
* Allow openjd_env to set vars to empty (#74) ([`c5ac75e`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/c5ac75e9e974acb404a5c73a1703337342d1ea44))
* Change default windows working directory to the &#34;C:\ProgramData\Amazon\OpenJD&#34; (#63) ([`36263d3`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/36263d3de64846755788dcd1fac9135c9d28d009))
* add logging for setting environment variables (#57) ([`4dd764b`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/4dd764bb51c73d0a9ae4c4b3e309f13a07d8141c))

## 0.3.0 (2024-01-18)

### BREAKING CHANGES
* **deps**: update to 0.2.0 of openjd-model (#51) ([`df09b9f`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/df09b9f7352ec383415fe2ad6b370a6cc9c661af))
* reuse ParameterValueType from model package (#49) ([`14fb1f3`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/14fb1f33c25ea63cb020b10bcd0e946a223e4ba1))

### Features
* Validate username and password in Windows. (#48) ([`ed23e54`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/ed23e542586a6e8b36f62429b73e551077f272a0))
* modify logging to be easier to understand (#43) ([`8aa7747`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/8aa77471478065ca0ca4cd67e0c68dcc642d16b6))
* allow adding env vars when running an action (#42) ([`c381877`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/c38187756129e1896cfb7d8b8e3202c8525dc422))
* Support notify feature on Windows. (#28) ([`8e816c8`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/8e816c88729aee2acef327ec013e60a6777059b0))

### Bug Fixes
* parameter name for signal_win_process (#40) ([`f54ad11`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/f54ad1131596286662b0be16abf9c10d5b932eea))
* properly delete working dir with Windows impersonation (#35) ([`5aae7ba`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/5aae7ba2ceab0631e66b857d690af25c7f42f4c3))
* make psutil a runtime dependency on Windows (#36) ([`a73fa39`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/a73fa3929e154bc12a227a582f0e53deef5746e7))

## 0.2.3 (2023-11-07)


### Features
* export package version (#31) ([`a8b7f30`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/a8b7f30c7255eb4ab98244a41a8c1ae1af27d996))
* Support session.cleanup() on Windows (#26) ([`7eaeecb`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/7eaeecb8245a8678bb1fe72ea9bc66ae2dc975e1))
* Support impersonation in tempdir permissions (#21) ([`02205f3`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/02205f3d7d46a60e1870b183325da0f897cef27b))

### Bug Fixes
* Remove embedded_files.write_file_for_user Windows exception (#32) ([`dc3ffbe`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/dc3ffbec0be4efd0a38b4cf90bfe2441e6a0152b))
* Make tempdir permissions inherited by descendants on Windows (#29) ([`5a06c8f`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/5a06c8fb914796528956bc9ae7246f3517beacd6))

## 0.2.2 (2023-10-27)




## 0.2.1 (2023-10-25)


### Features
* Change the Start-Process to Start-Job to support impersonation. (#17) ([`330cbde`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/330cbdebc26cf108ff80640a29998665038c6e71))
* Add Windows session user (#16) ([`4e954e6`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/4e954e6366b21ce6864ef0a83bc3220d96c43451))
* Import Windows implementation from internal repository (#12) ([`7b22f33`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/7b22f337ac6d5c6654243784e58ae7a6a70f13ba))

### Bug Fixes
* package missing signal subprocess shell script (#24) ([`57f68a9`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/57f68a908365fc0c8769b98d61a790e233e58030))
* exporting WindowsSessionUser class (#22) ([`63b2685`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/63b2685d9b1b1154727c7bb6b6fa1e48b6e882ce))
* Use psutil to kill the process instead of using taskkill. (#18) ([`4bba2ae`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/4bba2aeef9ebb5a5605ac7a3f09089a864808000))
* properly cleanup working dir with posix cross-user (#13) ([`6eb7aa3`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/6eb7aa3b2b5c78597b9da959a97a1572b80f1ef3))

## 0.2.0 (2023-09-15)

### BREAKING CHANGES
* updates to path mapping ([`2321af9`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/2321af9fd3190deebec4fa0530583c0865c28f54))


### Bug Fixes
* allow subprocess user to be the current user (#6) ([`8907765`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/89077656f45c0e68ba8108775cbe6b8349d20315))
* remove misleading &#39;rm&#39; error message (#10) ([`e25a41e`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/e25a41ea52d18a6d458d994b1b45c0277adde386))

## 0.1.0 (2023-09-12)

### BREAKING CHANGES
* Import from internal repository (#1) ([`abec10e`](https://github.com/OpenJobDescription/openjd-sessions-for-python/commit/abec10e2a8b1af8d81438b1c0ebf69bbc1a6ee52))



