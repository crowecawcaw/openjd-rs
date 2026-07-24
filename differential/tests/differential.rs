// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! The differential test entry points.
//!
//! Two cadences, both gated by the presence of the Python reference (see
//! [`try_oracle`]); when the reference is absent the tests print a skip line and
//! pass, so a plain `cargo test` for a developer without the checkout never
//! fails spuriously. CI provides the reference and sets `OPENJD_DIFF_REQUIRED=1`
//! so a missing reference there is a hard failure, not a silent skip.
//!
//! - [`conformance_and_regressions`] — the deterministic corpora
//!   (`corpus/*.jsonl`). Fast; runs on every PR. This is what stops the
//!   historical findings from being re-derived by hand every regeneration.
//! - [`generative_differential`] — the grammar-aware generator. Bounded by
//!   `OPENJD_DIFF_GEN_CASES` (default small for PRs; CI's nightly job raises
//!   it). Any divergence it finds is printed as a ready-to-paste corpus line so
//!   it can be frozen into `corpus/regressions.jsonl`.

use std::io::Write;
use std::path::{Path, PathBuf};

use openjd_differential::adapters::ExprCase;
use openjd_differential::generator::Generator;
use openjd_differential::oracle::{resolve_reference_src, OracleAnswer};
use openjd_differential::{compare, Allowlist, Oracle, Outcome};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Spawn the oracle, or return `None` (with an explanation) when the Python
/// reference isn't available. In `OPENJD_DIFF_REQUIRED=1` mode a missing/broken
/// reference panics instead — CI must never quietly skip.
fn try_oracle() -> Option<Oracle> {
    let required = std::env::var("OPENJD_DIFF_REQUIRED").as_deref() == Ok("1");
    let src = resolve_reference_src();
    if !src.exists() {
        let msg = format!(
            "Python reference not found at {}. Set OPENJD_PYTHON_REF_SRC or check out \
             openjd-model-for-python (branch `expr`) beside this repo.",
            src.display()
        );
        if required {
            panic!("OPENJD_DIFF_REQUIRED=1 but {msg}");
        }
        eprintln!("SKIP: {msg}");
        return None;
    }
    match Oracle::spawn(&src) {
        Ok(o) => Some(o),
        Err(e) => {
            if required {
                panic!("OPENJD_DIFF_REQUIRED=1 but failed to spawn oracle: {e}");
            }
            eprintln!("SKIP: failed to spawn oracle: {e}");
            None
        }
    }
}

fn load_cases(path: &Path) -> Vec<ExprCase> {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("reading corpus {}: {e}", path.display()));
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("//"))
        .map(|l| {
            serde_json::from_str::<ExprCase>(l)
                .unwrap_or_else(|e| panic!("bad corpus line in {}: {l:?}: {e}", path.display()))
        })
        .collect()
}

/// Run a batch of cases through the full differential and collect the
/// divergences that survive the allowlist. `internal`/`skipped` counts are
/// returned so callers can surface coverage rather than pretend everything ran.
struct RunReport {
    checked: usize,
    skipped_internal: usize,
    divergences: Vec<String>,
}

fn run_cases(oracle: &mut Oracle, allow: &Allowlist, cases: &[ExprCase]) -> RunReport {
    // The Rust evaluator can panic on some inputs (overflow-checks); those are
    // caught in `eval_rust` and reported as divergences. Silence the default
    // panic hook for the duration of the case loop so the run's output isn't
    // buried under backtraces — the divergence report names each panicking
    // input — then restore it so the test's own assertion failure prints.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let mut report = RunReport {
        checked: 0,
        skipped_internal: 0,
        divergences: Vec::new(),
    };
    for case in cases {
        let rust = case.eval_rust();
        let python = match case.eval_python(oracle) {
            Ok(OracleAnswer::Result(r)) => r,
            // Oracle faulted on this case: skip, don't fail. An oracle bug must
            // never masquerade as a Rust divergence.
            Ok(OracleAnswer::Internal(msg)) => {
                eprintln!("oracle internal error on {:?}: {msg}", case.expr);
                report.skipped_internal += 1;
                continue;
            }
            Err(e) => panic!("oracle transport error on {:?}: {e}", case.expr),
        };
        report.checked += 1;
        if let Outcome::Diverge(d) = compare(&rust, &python) {
            if let Some(entry) = allow.is_allowed(&case.expr) {
                eprintln!(
                    "ALLOWED divergence on {:?} ({}): {:?}",
                    case.expr, entry.reason, d.kind
                );
                continue;
            }
            report.divergences.push(format!(
                "  {:?}\n      rust={:?}\n      python={:?}\n      {:?}",
                case.expr, d.rust, d.python, d.kind
            ));
        }
    }
    std::panic::set_hook(prev_hook);
    report
}

#[test]
fn conformance_and_regressions() {
    let Some(mut oracle) = try_oracle() else {
        return;
    };
    eprintln!(
        "differential: oracle reference = {}",
        oracle.reference_src().display()
    );
    let allow = Allowlist::load_default().expect("load allowlist");

    let mut all = load_cases(&manifest_dir().join("corpus/conformance.jsonl"));
    all.extend(load_cases(&manifest_dir().join("corpus/regressions.jsonl")));

    let report = run_cases(&mut oracle, &allow, &all);
    eprintln!(
        "differential: {} cases checked, {} skipped (oracle-internal), allowlist has {} entries",
        report.checked,
        report.skipped_internal,
        allow.len()
    );

    assert!(
        report.divergences.is_empty(),
        "\n{} unallowed divergence(s) between Rust and Python:\n{}\n\n\
         Each is either a Rust bug to fix, or — if intentional — an entry to add to \
         allowlist.json with a written reason.",
        report.divergences.len(),
        report.divergences.join("\n")
    );
}

#[test]
fn generative_differential() {
    let Some(mut oracle) = try_oracle() else {
        return;
    };
    let allow = Allowlist::load_default().expect("load allowlist");

    let n: usize = std::env::var("OPENJD_DIFF_GEN_CASES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2_000);
    let seed: u64 = std::env::var("OPENJD_DIFF_GEN_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0x0123_4567_89AB_CDEFu64);

    let mut gen = Generator::new(seed);
    let cases: Vec<ExprCase> = (0..n).map(|_| gen.next_case()).collect();

    let report = run_cases(&mut oracle, &allow, &cases);
    eprintln!(
        "differential(generative): seed={seed} generated={n} checked={} skipped_internal={}",
        report.checked, report.skipped_internal
    );

    if !report.divergences.is_empty() {
        // Emit reproducers to a file the CI job uploads, so a failure is
        // actionable: paste these lines into corpus/regressions.jsonl.
        let out = manifest_dir().join(format!("target/generative-divergences-{seed}.txt"));
        if let Ok(mut f) = std::fs::File::create(&out) {
            let _ = writeln!(f, "# seed={seed}");
            for d in &report.divergences {
                let _ = writeln!(f, "{d}");
            }
        }
    }

    assert!(
        report.divergences.is_empty(),
        "\n{} generative divergence(s) (seed={seed}):\n{}\n\n\
         Minimize and freeze each into corpus/regressions.jsonl, then fix the Rust side \
         or add an allowlist.json entry.",
        report.divergences.len(),
        report.divergences.join("\n")
    );
}
