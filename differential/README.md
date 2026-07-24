<!--
Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
Copyright by contributors to this project.
SPDX-License-Identifier: (Apache-2.0 OR MIT)
-->

# openjd-rs differential testing

Runs the Rust expression engine (`openjd-expr`) and the **Python OpenJD
reference implementation** on the same input and asserts they agree. This
catches the *silent-wrong-value* class of divergence — where the Rust port
neither panics nor errors but returns a value that disagrees with Python — which
the fuzzer (whose only invariant is "does not crash") and human review both miss.

It is the complement of the sibling [`fuzz/`](../fuzz) crate: the fuzzer asks
"does any input crash?", this asks "does any input compute a *different answer*
than the reference?".

This crate is **not** a member of the root workspace (it has its own empty
`[workspace]` table) because it depends on an out-of-tree Python checkout. It
runs in its own [`.github/workflows/differential.yml`](../.github/workflows/differential.yml)
workflow rather than the stable CI matrix.

## How it works

```
   input ──┬──▶ Rust: ParsedExpression::new + evaluate ──▶ TaggedResult ──┐
           │                                                              ├─▶ compare()
           └──▶ Python worker (long-lived process) ───────▶ TaggedResult ─┘        │
                                                                                   ▼
                                                             Agree │ Diverge (checked against allowlist)
```

- **Oracle** (`src/oracle.rs`, `oracle/oracle_worker.py`) — a long-lived Python
  worker speaking newline-delimited JSON over stdin/stdout. Interpreter + import
  startup is paid once; each case is one write + read. Both sides emit
  `openjd-expr`'s own `{"type", "value"}` transport tag, so comparison is a plain
  structural equality — and an `int` result is distinguishable from a `float`
  that displays the same (the `2**63` vs `2.0**63` case).
- **Equivalence** (`src/equivalence.rs`) — the three-way decision: compare typed
  values on `(ok, ok)`; pass on `(err, err)` **without** comparing error messages
  (Python and Rust word them differently); flag `(ok, err)`/`(err, ok)` and any
  Rust panic as divergences.
- **Allowlist** (`src/allowlist.rs`, `allowlist.json`) — intentional, reviewed
  divergences. A divergence is a pass only if its exact input is listed with a
  written reason. Keep this list short; prefer fixing the Rust side.
- **Adapters** (`src/adapters.rs`) — turn one input into both sides' result for a
  domain. Today: the `expr` surface. `Manifest` decode and path-mapping are the
  same shape and slot in the same way.
- **Generator** (`src/generator.rs`) — a deterministic, grammar-aware expression
  generator biased toward the historically-buggy edges (`i64::MIN`, `2**63`,
  negative counts, multibyte strings). Random bytes would trivially agree on both
  sides; structurally-valid expressions exercise real evaluation paths.

## Inputs

- **`corpus/conformance.jsonl`** — common expressions that must always agree.
- **`corpus/regressions.jsonl`** — every past divergence, frozen with its input
  so it can never silently regress. New divergences the generator finds are
  minimized and added here.
- **The generator** — unbounded structured inputs, count-boxed per run.

## Running locally

The harness needs the Python reference checked out (branch `expr` of the mwiebe
fork), side by side with this repo — the same layout the `eval-crate` skill sets
up:

```
<parent>/
├── openjd-rs/                 # this repo
└── openjd-model-for-python/   # git clone -b expr https://github.com/mwiebe/openjd-model-for-python
```

```sh
# From differential/. The reference is auto-resolved from the side-by-side
# checkout, or point at it explicitly:
export OPENJD_PYTHON_REF_SRC=/path/to/openjd-model-for-python/src

# Fast deterministic gate (conformance + regressions):
cargo test --test differential conformance_and_regressions -- --nocapture

# Generative differential (count-boxed; raise for a deeper local run):
OPENJD_DIFF_GEN_CASES=50000 cargo test --test differential generative_differential -- --nocapture

# Reproduce a specific generator seed:
OPENJD_DIFF_GEN_SEED=42 OPENJD_DIFF_GEN_CASES=25000 \
  cargo test --test differential generative_differential -- --nocapture
```

Without the reference present, the tests **skip** (print a `SKIP:` line and
pass) so a plain `cargo test` never fails spuriously. In CI, `OPENJD_DIFF_REQUIRED=1`
turns a missing reference into a hard failure — the job must never silently skip.

### Environment variables

| Variable | Purpose |
|----------|---------|
| `OPENJD_PYTHON_REF_SRC` | Path to the reference `src/`. Overrides side-by-side auto-resolution. |
| `OPENJD_DIFF_REQUIRED` | `1` → a missing/broken reference is a hard failure (CI). Otherwise skip. |
| `OPENJD_DIFF_GEN_CASES` | Number of generated cases (default 2000). |
| `OPENJD_DIFF_GEN_SEED` | Generator seed (default fixed) for reproducibility. |
| `OPENJD_DIFF_PYTHON` | Python interpreter to run the worker (default `python3`). |

## When a divergence is found

The failing test prints each divergence with its input and both sides' results,
and the generative test writes ready-to-freeze corpus lines to
`target/generative-divergences-<seed>.txt`. For each one, decide:

1. **Rust bug** — fix `openjd-expr`, then add the input to
   `corpus/regressions.jsonl` so it stays fixed. (This is the common case.)
2. **Intentional** — add an entry to `allowlist.json` with a written reason and
   a tracking reference.

Either way the input ends up recorded, so the finding is never re-derived by
hand.

## CI cadence

The same checks run on every PR and on merges to main (`differential` job):
conformance + regressions (the fast gate) plus a count-boxed generative run at
the fixed default seed. For deeper coverage, run a multi-seed sweep locally
before landing large changes:

```sh
for seed in 1 2 3 42 100 777; do
  OPENJD_DIFF_GEN_SEED=$seed OPENJD_DIFF_GEN_CASES=25000 \
    cargo test --test differential generative_differential -- --nocapture
done
```

Any new divergence found that way should be minimized and frozen into
`corpus/regressions.jsonl`.
