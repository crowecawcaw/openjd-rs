// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! The equivalence relation — the subtle heart of the harness.
//!
//! Given the Rust result and the Python result for the same input, decide
//! whether they agree. It is a three-way decision, and getting each arm right
//! is what separates a useful oracle from a false-positive generator:
//!
//! | Rust \ Python | Ok                              | Err                    |
//! |---------------|---------------------------------|------------------------|
//! | **Ok**        | compare typed tags (below)      | **DIVERGENCE**         |
//! | **Err**       | **DIVERGENCE**                  | agree (don't compare)  |
//!
//! - **(Ok, Ok):** compare the *typed* transport tags for structural equality.
//!   The tag carries `type` separately from `value`, so `int 9223372036854775808`
//!   and `float 9.2e18` are a genuine mismatch, not a formatting artifact. Both
//!   sides canonicalize the value string identically (same `format_float`, same
//!   list rendering), so equal typed value ⟺ equal tags.
//!
//! - **(Err, Err):** agree. We do **not** compare error messages or categories.
//!   Python raises a flat `ExpressionError`; Rust carries a structured
//!   `ExpressionErrorKind`. Their wordings will never match, and requiring them
//!   to would drown every real finding in noise. "Both rejected the input" is
//!   the invariant that matters.
//!
//! - **(Ok, Err) / (Err, Ok):** the divergence that matters most. This single
//!   asymmetry is exactly the "silent saturation instead of error" and
//!   "silent-wrong-value" class: one side computed a value, the other refused.

use crate::oracle::TaggedResult;

/// The result of comparing the two implementations on one input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The implementations agree (equal typed value, or both errored).
    Agree,
    /// The implementations disagree. Carries a structured description.
    Diverge(Divergence),
}

/// A described divergence, ready to print in a test failure or freeze into the
/// regression corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Divergence {
    pub kind: DivergenceKind,
    pub rust: TaggedResult,
    pub python: TaggedResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DivergenceKind {
    /// Both produced a value but the typed tags differ. `detail` names the
    /// axis (type vs. value) to make triage instant.
    ValueMismatch { detail: String },
    /// Rust returned a value; Python raised. Classic "silent wrong value".
    RustOkPythonErr,
    /// Python returned a value; Rust raised. The inverse — Rust is stricter.
    PythonOkRustErr,
    /// The Rust evaluator panicked. Always a divergence: Python never panics,
    /// and in a release build (overflow-checks off) the same code path would
    /// silently wrap into a wrong value instead of aborting.
    RustPanicked,
}

/// Apply the three-way equivalence relation.
pub fn compare(rust: &TaggedResult, python: &TaggedResult) -> Outcome {
    // A Rust panic is unconditionally a divergence, whatever Python did.
    if let TaggedResult::Panicked = rust {
        return Outcome::Diverge(Divergence {
            kind: DivergenceKind::RustPanicked,
            rust: rust.clone(),
            python: python.clone(),
        });
    }
    match (rust, python) {
        (TaggedResult::Ok(r), TaggedResult::Ok(p)) => {
            if r == p {
                Outcome::Agree
            } else {
                let detail = if r.type_str != p.type_str {
                    format!("type: rust={:?} python={:?}", r.type_str, p.type_str)
                } else {
                    format!("value: rust={} python={}", r.value, p.value)
                };
                Outcome::Diverge(Divergence {
                    kind: DivergenceKind::ValueMismatch { detail },
                    rust: rust.clone(),
                    python: python.clone(),
                })
            }
        }
        // Both errored: agree, regardless of category. (Categories are logged
        // by callers for triage but never gate here.)
        (TaggedResult::Err { .. }, TaggedResult::Err { .. }) => Outcome::Agree,
        (TaggedResult::Ok(_), TaggedResult::Err { .. }) => Outcome::Diverge(Divergence {
            kind: DivergenceKind::RustOkPythonErr,
            rust: rust.clone(),
            python: python.clone(),
        }),
        (TaggedResult::Err { .. }, TaggedResult::Ok(_)) => Outcome::Diverge(Divergence {
            kind: DivergenceKind::PythonOkRustErr,
            rust: rust.clone(),
            python: python.clone(),
        }),
        // Rust `Panicked` is handled above. Python never yields `Panicked`
        // (non-`ExpressionError` becomes an internal_error the harness skips),
        // so a `Panicked` on the Python side is unreachable; treat any residual
        // pairing as a divergence rather than silently agreeing.
        _ => Outcome::Diverge(Divergence {
            kind: DivergenceKind::RustPanicked,
            rust: rust.clone(),
            python: python.clone(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oracle::TransportTag;

    fn ok(t: &str, v: serde_json::Value) -> TaggedResult {
        TaggedResult::Ok(TransportTag {
            type_str: t.to_string(),
            value: v,
        })
    }
    fn err(c: &str) -> TaggedResult {
        TaggedResult::Err {
            category: c.to_string(),
        }
    }

    #[test]
    fn equal_typed_values_agree() {
        assert_eq!(
            compare(&ok("int", "3".into()), &ok("int", "3".into())),
            Outcome::Agree
        );
    }

    #[test]
    fn int_vs_float_same_display_is_a_divergence() {
        // The 2**63 vs 2.0**63 shape: same-ish number, different type.
        let out = compare(&ok("int", "9".into()), &ok("float", "9".into()));
        assert!(matches!(
            out,
            Outcome::Diverge(Divergence {
                kind: DivergenceKind::ValueMismatch { .. },
                ..
            })
        ));
    }

    #[test]
    fn both_err_agree_even_with_different_categories() {
        assert_eq!(compare(&err("overflow"), &err("type")), Outcome::Agree);
    }

    #[test]
    fn rust_ok_python_err_is_the_silent_wrong_value_class() {
        let out = compare(&ok("int", "42".into()), &err("overflow"));
        assert!(matches!(
            out,
            Outcome::Diverge(Divergence {
                kind: DivergenceKind::RustOkPythonErr,
                ..
            })
        ));
    }

    #[test]
    fn python_ok_rust_err_flags_over_strict_rust() {
        let out = compare(&err("overflow"), &ok("int", "42".into()));
        assert!(matches!(
            out,
            Outcome::Diverge(Divergence {
                kind: DivergenceKind::PythonOkRustErr,
                ..
            })
        ));
    }
}
