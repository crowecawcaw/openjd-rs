// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! A grammar-aware expression generator.
//!
//! Random *bytes* are near-useless as a differential input: they parse-error on
//! both sides and trivially "agree", exercising no evaluation path. To reach
//! the arithmetic / coercion / path code where silent divergences live, we
//! generate structurally valid-ish expressions from a small model of the EXPR
//! grammar — literals, operators, and real library-function calls — biased
//! toward the edge values the quality reports kept flagging (`i64::MIN`,
//! `2**63`, negative counts, multibyte strings).
//!
//! The generator is deterministic given its seed (`SmallRng`), so any divergence
//! it finds reproduces exactly and can be frozen into the regression corpus.

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::adapters::ExprCase;

/// Deterministic generator of expression cases.
pub struct Generator {
    rng: SmallRng,
    max_depth: u32,
}

impl Generator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(seed),
            max_depth: 4,
        }
    }

    /// Generate one case: an expression plus (sometimes) a symbol table.
    pub fn next_case(&mut self) -> ExprCase {
        let expr = self.expr(self.max_depth);
        let mut case = ExprCase::new(expr);
        // Occasionally attach a small symbol table and/or a path format so the
        // Windows path branch and symbol resolution get exercised.
        if self.rng.gen_bool(0.3) {
            case = case.with_symbols(self.symbols());
        }
        match self.rng.gen_range(0..3) {
            0 => case = case.with_path_format("POSIX"),
            1 => case = case.with_path_format("WINDOWS"),
            _ => {}
        }
        case
    }

    fn symbols(&mut self) -> serde_json::Value {
        let mut obj = serde_json::Map::new();
        for name in ["N", "Param.Count", "S", "P"] {
            if self.rng.gen_bool(0.6) {
                let v = match self.rng.gen_range(0..3) {
                    0 => serde_json::Value::from(self.edge_int()),
                    1 => serde_json::Value::from(self.string_literal_body()),
                    _ => serde_json::Value::from(self.rng.gen::<bool>()),
                };
                obj.insert(name.to_string(), v);
            }
        }
        serde_json::Value::Object(obj)
    }

    fn expr(&mut self, depth: u32) -> String {
        if depth == 0 {
            return self.atom();
        }
        match self.rng.gen_range(0..10) {
            0..=3 => {
                // binary op
                let op = *pick(&mut self.rng, &["+", "-", "*", "/", "//", "%", "**"]);
                format!("({} {} {})", self.expr(depth - 1), op, self.expr(depth - 1))
            }
            4 => {
                // comparison — the int/float coercion surface (2**63 == 2.0**63)
                let op = *pick(&mut self.rng, &["==", "!=", "<", ">", "<=", ">="]);
                format!("({} {} {})", self.expr(depth - 1), op, self.expr(depth - 1))
            }
            5 => format!("(-{})", self.expr(depth - 1)),
            6..=8 => self.call(depth),
            _ => self.atom(),
        }
    }

    fn call(&mut self, depth: u32) -> String {
        // A curated set of real library functions with argument shapes that
        // reach the historically-buggy code (negative counts, char-boundary
        // work, rounding). Note the deliberate split: arithmetic *operands* use
        // `edge_int()` (including `i64::MAX`) because `i64::MAX + 1` is a cheap,
        // high-value overflow probe — but *count* positions (round ndigits,
        // center width, split maxsplit) use `count_int()`, which stays small.
        // A huge value there is not an interesting differential: both Python
        // and Rust would just build a multi-gigabyte string, blowing the CI
        // budget for zero signal. The genuine extreme-count edges (i64::MIN
        // ndigits, the precision-too-big boundary) live in the frozen
        // regression corpus instead, where they hit fast error/int paths.
        match self.rng.gen_range(0..12) {
            0 => format!("abs({})", self.expr(depth - 1)),
            1 => format!("round({}, {})", self.expr(depth - 1), self.count_int()),
            2 => format!("min({}, {})", self.expr(depth - 1), self.expr(depth - 1)),
            3 => format!("max({}, {})", self.expr(depth - 1), self.expr(depth - 1)),
            4 => format!("floor({})", self.expr(depth - 1)),
            5 => format!("ceil({})", self.expr(depth - 1)),
            6 => format!("int({})", self.expr(depth - 1)),
            7 => format!("float({})", self.expr(depth - 1)),
            8 => format!("len({})", self.string_literal()),
            9 => format!("center({}, {})", self.string_literal(), self.count_int()),
            10 => format!("split({}, {})", self.string_literal(), self.count_int()),
            _ => format!("path({}).name", self.path_literal()),
        }
    }

    /// A small integer for *count* positions (ndigits, width, maxsplit),
    /// including small negatives to exercise the negative-count paths that were
    /// buggy — but never so large that formatting the result allocates a huge
    /// string. See the note in [`call`](Self::call).
    fn count_int(&mut self) -> i64 {
        self.rng.gen_range(-8..12)
    }

    fn atom(&mut self) -> String {
        match self.rng.gen_range(0..6) {
            0 => self.edge_int().to_string(),
            1 => self.float_literal(),
            2 => self.string_literal(),
            3 => self.path_literal_expr(),
            4 => (*pick(&mut self.rng, &["true", "false", "null"])).to_string(),
            _ => (*pick(&mut self.rng, &["N", "Param.Count", "S", "P"])).to_string(),
        }
    }

    /// Integers biased toward overflow-triggering edges.
    fn edge_int(&mut self) -> i64 {
        let edges = [
            0,
            1,
            -1,
            i64::MAX,
            i64::MIN,
            i64::MAX - 1,
            i64::MIN + 1,
            4_611_686_018_427_387_904, // 2**62
            9_223_372_036_854_775_807, // i64::MAX
        ];
        if self.rng.gen_bool(0.6) {
            *pick(&mut self.rng, &edges)
        } else {
            self.rng.gen_range(-1000..1000)
        }
    }

    fn float_literal(&mut self) -> String {
        let choices = ["0.0", "1.5", "-9.2e18", "9.2e18", "1e308", "-1e308", "0.1"];
        (*pick(&mut self.rng, &choices)).to_string()
    }

    fn string_literal(&mut self) -> String {
        format!("'{}'", self.string_literal_body())
    }

    fn string_literal_body(&mut self) -> String {
        // Include multibyte content — the char-boundary surface (path('日')).
        let choices = ["", "a", "ab", "12.5", "日本語", "a b c", "prefix"];
        (*pick(&mut self.rng, &choices)).to_string()
    }

    fn path_literal(&mut self) -> String {
        let choices = [
            "'/tmp/x'",
            "'C:\\\\a\\\\b'",
            "'日/本'",
            "'a'",
            "'/a/b/c.txt'",
        ];
        (*pick(&mut self.rng, &choices)).to_string()
    }

    fn path_literal_expr(&mut self) -> String {
        format!("path({})", self.path_literal())
    }
}

fn pick<'a, T>(rng: &mut SmallRng, items: &'a [T]) -> &'a T {
    &items[rng.gen_range(0..items.len())]
}
