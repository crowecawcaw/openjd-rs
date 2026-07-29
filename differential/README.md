# Differential testing for `openjd-expr`

Runs `openjd-expr` and a reference oracle against the same expressions and
reports where they disagree.

```bash
differential/run.py                 # build, run, report (exit 1 on divergence)
differential/run.py --no-build      # reuse an existing example binary
differential/run.py --filter paths  # only cases whose id or area matches
differential/run.py --json out.json # also write a machine-readable report
```

Needs only a stdlib Python 3.9+ and a cargo toolchain.

## Why

The EXPR language is defined by the 2026-02 Expression Language spec as a subset
of Python. The bug class that matters here is **silent-wrong-value**: the
expression parses, evaluates, returns the right shape, and is wrong in the last
few digits or in which side of a string got the extra space.

Nothing else in the pipeline catches these. They are not crashes, so fuzzing and
panic-probing miss them. They are not obviously wrong on inspection — upstream's
own quality report reviewed the exact functions carrying the divergences below
and recorded `functions/comparison.rs` … "had no confirmed issues", and called
the neighbouring int paths "exemplary". Catching them requires diffing against
an independent implementation.

## The oracle, and why it isn't `openjd-model-for-python`

`openjd-model-for-python` is the obvious choice and it does not work. On
upstream mainline, `src/openjd/expr/` is two files: `__init__.py` re-exports
from `openjd._openjd_rs`, and `rust-bindings/Cargo.toml` depends on
`openjd-expr` — the crate under test. Diffing against it compares
`openjd-expr` to itself: 100% agreement, zero bugs, permanently green. A harness
wired that way looks healthy while testing nothing.

(The only independent Python implementation of EXPR was an unmerged personal
fork, which upstream has since replaced with these bindings.)

So the oracle is **CPython itself** for the Python-semantics subset, plus **the
spec** where the spec speaks. Every case declares which authority governs it,
because the two genuinely conflict — float `//` return type and `round`
trailing-zero display are both cases where the spec overrides CPython. A harness
that assumed CPython was always right would report spec-mandated behavior as a
bug; an earlier evaluation of this crate did exactly that.

| `oracle` | Meaning | Gates CI |
|---|---|---|
| `cpython` | Spec defers to Python semantics but doesn't pin this edge case. CPython is the de-facto definition. | yes |
| `spec` | Spec states the answer (`spec_ref` cites the line). Spec wins over CPython. | yes |
| `invariant` | An algebraic property that must hold whatever the spec chooses (e.g. antisymmetry). No external oracle needed. | yes |
| `open` | Plausible oracles disagree; the spec must decide. Reported as INFO. | no |

## Layout

- `cases.json` — the corpus. Each case carries an `expr`, the `python` snippet
  computing the reference, its `oracle`, and a `note` explaining the mechanism.
- `run.py` — builds the example, runs the cases, adjudicates, reports.
- `crates/openjd-expr/examples/differential_eval.rs` — evaluates expressions and
  emits JSON. Deliberately knows nothing about expected values or pass/fail, so
  the reference semantics live in exactly one place.

An `example` rather than a test binary, so a divergence reports as a finding and
never fails `cargo test`.

## Adding a case

Give it a stable `id`, an `expr`, a `python` snippet, and an `oracle`. Then:

```bash
differential/run.py --update-expected   # fills `expected` from the oracle
```

**Review that diff by hand.** Blind-accepting would bake a current bug in as the
expected answer, which is the one failure mode that makes a harness like this
worse than nothing.

Two conventions worth knowing:

- **Only set `expected_type` when the type is what you're testing.** The oracle's
  own Python type is not a cross-language expectation — Python has no PATH type,
  so `ntpath.join(...)` returns `str` where the correct EXPR result is `path`.
- **Booleans** are normalized: Python's `True`/`False` versus EXPR's
  `true`/`false` is surface syntax between two languages, not a divergence.

## Self-check

The harness must be able to fail. To confirm the comparison isn't a no-op,
corrupt a passing case's `expected` and check it reports FAIL:

```bash
differential/run.py --filter center-even-padding
```

This guards against the tautology described above, in which a harness reports
agreement because it is unwittingly comparing an implementation to itself.
