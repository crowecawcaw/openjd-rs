// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! Rust half of the differential harness.
//!
//! Reads `{"cases": [{"id", "expr", "path_format"?}, ...]}` on stdin, evaluates
//! each expression with `openjd-expr`, and writes
//! `{"results": [{"id", "ok", "value"|"error", "type"}, ...]}` to stdout.
//!
//! This is intentionally a thin, dumb pipe: it knows nothing about expected
//! values, oracles, or pass/fail. All adjudication lives in
//! `differential/run.py`, so the reference semantics are defined in exactly one
//! place and this side cannot accidentally encode an expectation.
//!
//! An `example` rather than a test binary: it is a developer/CI tool that
//! reports observations, and must not fail the crate's own test suite.
//!
//! Run via `differential/run.py`, or standalone:
//!   echo '{"cases":[{"id":"x","expr":"1+1"}]}' \
//!     | cargo run -p openjd-expr --example differential_eval

use std::io::Read;

use openjd_expr::{ExprValue, ParsedExpression, PathFormat, SymbolTable};
use serde_json::{json, Value};

/// Render a value plus a type tag.
///
/// The type tag matters as much as the value: several known divergences are
/// wrong-TYPE-but-plausible-value (`float` where the spec says `int`), which a
/// string-only comparison would silently pass.
fn render(v: &ExprValue) -> (String, &'static str) {
    match v {
        ExprValue::Null => ("null".to_string(), "null"),
        ExprValue::Bool(b) => (b.to_string(), "bool"),
        ExprValue::Int(i) => (i.to_string(), "int"),
        // to_display_string() is the user-visible rendering (what lands in a
        // resolved template), so display bugs like exponent formatting are in
        // scope rather than hidden behind a debug format.
        ExprValue::Float(_) => (v.to_display_string(), "float"),
        ExprValue::String(s) => (s.clone(), "string"),
        ExprValue::Path { value, .. } => (value.clone(), "path"),
        other => (other.to_display_string(), "list"),
    }
}

fn main() {
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut input) {
        eprintln!("differential_eval: failed to read stdin: {e}");
        std::process::exit(2);
    }

    let parsed: Value = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("differential_eval: stdin is not valid JSON: {e}");
            std::process::exit(2);
        }
    };

    let Some(cases) = parsed.get("cases").and_then(|c| c.as_array()) else {
        eprintln!("differential_eval: expected a top-level 'cases' array");
        std::process::exit(2);
    };

    let mut results = Vec::with_capacity(cases.len());

    for case in cases {
        let id = case.get("id").and_then(|v| v.as_str()).unwrap_or("<no id>");
        let Some(expr) = case.get("expr").and_then(|v| v.as_str()) else {
            eprintln!("differential_eval: case '{id}' has no 'expr'");
            std::process::exit(2);
        };

        // Path-dependent cases must state their format explicitly. Defaulting
        // silently to the host format would make results differ by runner OS.
        let fmt = match case.get("path_format").and_then(|v| v.as_str()) {
            Some("windows") => PathFormat::Windows,
            Some("uri") => PathFormat::Uri,
            Some("posix") | None => PathFormat::Posix,
            Some(other) => {
                eprintln!("differential_eval: case '{id}': unknown path_format '{other}'");
                std::process::exit(2);
            }
        };

        let symtab = SymbolTable::new();
        let symtabs = [&symtab];
        let outcome =
            ParsedExpression::new(expr).and_then(|p| p.with_path_format(fmt).evaluate(&symtabs));

        results.push(match outcome {
            Ok(value) => {
                let (rendered, ty) = render(&value);
                json!({"id": id, "ok": true, "value": rendered, "type": ty})
            }
            Err(e) => {
                // First line only: the caret block below it is diagnostic
                // decoration and would make comparisons brittle.
                let msg = e.to_string();
                let first = msg.lines().next().unwrap_or("").to_string();
                json!({"id": id, "ok": false, "error": first, "type": "error"})
            }
        });
    }

    println!("{}", json!({"results": results}));
}
