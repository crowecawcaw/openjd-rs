// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! Per-domain adapters.
//!
//! An adapter knows how to take one raw input and produce both sides'
//! [`TaggedResult`] for a given OpenJD surface. The differential *engine*
//! (oracle + equivalence + allowlist) is domain-agnostic; only the adapter
//! changes when we point the harness at a new surface.
//!
//! Today this implements the `expr` adapter (evaluate an expression against a
//! symbol table under a path format). The other class-3 surfaces the analysis
//! names — `Manifest` decode+validate and path-mapping — are the same shape
//! (decode on both sides, tag the model or the validation-error presence) and
//! add here as sibling adapters without touching the engine.

use openjd_expr::{ParsedExpression, PathFormat, SymbolTable};

use crate::oracle::{Oracle, OracleAnswer, TaggedResult};

/// A fully-specified differential case for the expression surface.
///
/// This is what the seed corpus deserializes into and what the generator emits.
/// The `symbols` object and `path_format` are shared verbatim by both sides, so
/// the only thing that can differ is the implementations' behavior.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExprCase {
    /// The expression source.
    pub expr: String,
    /// Optional symbol-table bindings: a JSON object of (possibly dotted) name
    /// → scalar value. `None`/absent means an empty environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<serde_json::Value>,
    /// `"POSIX"`, `"WINDOWS"`, or `None` for host-native.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_format: Option<String>,
}

impl ExprCase {
    pub fn new(expr: impl Into<String>) -> Self {
        Self {
            expr: expr.into(),
            symbols: None,
            path_format: None,
        }
    }

    pub fn with_symbols(mut self, symbols: serde_json::Value) -> Self {
        self.symbols = Some(symbols);
        self
    }

    pub fn with_path_format(mut self, pf: impl Into<String>) -> Self {
        self.path_format = Some(pf.into());
        self
    }

    /// Evaluate this case on the Rust side, producing a tag.
    ///
    /// Wrapped in [`catch_unwind`](std::panic::catch_unwind) because the Rust
    /// evaluator can still *panic* on some inputs (e.g. arithmetic overflow
    /// under debug/overflow-checks). A panic is a first-class differential
    /// outcome: Python never panics, so it is always a divergence — and in a
    /// release build the same overflow would silently wrap into a wrong value,
    /// which is exactly this harness's target class. Catching it also keeps one
    /// bad case from aborting the whole run.
    pub fn eval_rust(&self) -> TaggedResult {
        let expr = self.expr.clone();
        let symbols = self.symbols.clone();
        let path_format = self.path_format.clone();
        let evaluated = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let symtab = build_symbol_table(symbols.as_ref());
            let format = parse_path_format(path_format.as_deref());
            match ParsedExpression::new(&expr) {
                Ok(parsed) => {
                    let builder = match format {
                        Some(fmt) => parsed.with_path_format(fmt),
                        None => parsed.with_path_format(PathFormat::Posix),
                    };
                    builder.evaluate(&[&symtab])
                }
                Err(e) => Err(e),
            }
        }));
        match evaluated {
            Ok(result) => TaggedResult::from_expr_result(result),
            Err(_) => TaggedResult::Panicked,
        }
    }

    /// Evaluate this case on the Python side via the oracle.
    pub fn eval_python(&self, oracle: &mut Oracle) -> std::io::Result<OracleAnswer> {
        oracle.eval_expr(
            &self.expr,
            self.symbols.as_ref(),
            self.path_format.as_deref(),
        )
    }
}

/// Build a Rust `SymbolTable` from the JSON `symbols` object.
///
/// Mirrors the fuzzer's `symtab_from_json`: iterate scalar entries and `set`
/// each by its (possibly dotted) name, which the crate namespaces into nested
/// scopes exactly as job parameters and session variables are. `set` conflicts
/// (e.g. both `"A"` and `"A.B"` present) are ignored — a partial table is still
/// a valid environment, and the Python side resolves the same bindings the same
/// way, so any resulting undefined-symbol error occurs symmetrically.
fn build_symbol_table(symbols: Option<&serde_json::Value>) -> SymbolTable {
    let mut st = SymbolTable::new();
    if let Some(serde_json::Value::Object(obj)) = symbols {
        for (k, v) in obj {
            let _ = match v {
                serde_json::Value::String(s) => st.set(k, s.as_str()),
                serde_json::Value::Bool(b) => st.set(k, *b),
                serde_json::Value::Number(n) => match n.as_i64() {
                    Some(i) => st.set(k, i),
                    None => continue, // non-i64 numbers aren't representable; skip symmetrically
                },
                _ => continue,
            };
        }
    }
    st
}

fn parse_path_format(name: Option<&str>) -> Option<PathFormat> {
    match name {
        Some("POSIX") => Some(PathFormat::Posix),
        Some("WINDOWS") => Some(PathFormat::Windows),
        _ => None,
    }
}
