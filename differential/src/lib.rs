// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

//! Differential oracle: run the Rust `openjd-expr` implementation and the
//! Python OpenJD reference on the same input and assert they agree.
//!
//! This crate catches the *silent-wrong-value* class of divergence — the case
//! where the Rust port neither panics nor errors but returns a value that
//! disagrees with what Python would produce for the same input. That class is
//! invisible to the fuzzer (whose only invariant is "don't crash") and nearly
//! invisible to a human reading the code, which is why the quality-evaluation
//! reports kept re-litigating it. Here it becomes a failing test.
//!
//! # Shape
//!
//! - [`Oracle`] owns a long-lived Python worker process and answers "what does
//!   Python compute for this input?" as a [`TaggedResult`].
//! - [`compare`] is the three-way equivalence relation over
//!   `(rust_result, python_result)` — the subtle core (see its docs).
//! - [`Allowlist`] records the intentional, reviewed divergences so that
//!   "match Python except for these N documented choices" is enforceable rather
//!   than aspirational.
//! - The [`adapters`] module turns a raw input into both sides' [`TaggedResult`]
//!   for a given domain (expression evaluation today; manifest decode and
//!   path-mapping are the same shape and slot in the same way).
//!
//! The value representation exchanged by both sides is `openjd-expr`'s own
//! transport tag (`{"type": ..., "value": ...}`, see
//! [`openjd_expr::ExprValue::to_json_transport`]); the Python worker emits the
//! byte-identical shape. Comparing tags rather than raw strings is what makes
//! an `int` result distinguishable from a `float` result that displays the
//! same — precisely the `2**63` vs `2.0**63` finding.

pub mod adapters;
pub mod allowlist;
pub mod equivalence;
pub mod generator;
pub mod oracle;

pub use allowlist::Allowlist;
pub use equivalence::{compare, Divergence, Outcome};
pub use oracle::{Oracle, TaggedResult};
