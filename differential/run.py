#!/usr/bin/env python3
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# Copyright by contributors to this project.
# SPDX-License-Identifier: (Apache-2.0 OR MIT)

"""Differential harness: openjd-expr vs. an explicitly-attributed oracle.

Why this exists
---------------
openjd-expr implements the EXPR language, which the 2026-02 Expression Language
spec defines as a subset of Python. Whole classes of bug in such an
implementation are silent-wrong-value: the expression evaluates, returns a
plausible result of the right shape, and is wrong in the last few digits or in
which side of a string got the extra space. Crash-probing and hand review do not
find these -- upstream's own quality report reviewed the exact functions that
carry the divergences below and recorded "no confirmed issues" for them. Finding
them needs a reference implementation to diff against.

On the choice of oracle
-----------------------
The obvious oracle -- `openjd-model-for-python` -- is NOT usable, and this is
worth stating plainly because it looks like it should be. On upstream mainline,
`src/openjd/expr/` is two files; `__init__.py` re-exports from
`openjd._openjd_rs`, and `rust-bindings/Cargo.toml` depends on
`openjd-expr` -- the crate under test. Diffing against it compares openjd-expr
to itself: it agrees 100%, forever, and reports zero bugs. A harness wired that
way would look healthy while testing nothing. (The only independent Python
implementation is an unmerged personal fork, which upstream has since replaced.)

So the oracle here is CPython itself for the Python-semantics subset, plus the
spec where the spec speaks. Each case names its own authority -- see
`cases.json` for what the four `oracle` values mean. This matters because the
spec and CPython genuinely conflict in places (float `//` return type,
`round` trailing zeros), and a harness that assumed CPython was always right
would report spec-mandated behavior as a bug. An earlier evaluation of this
crate did exactly that, which is why attribution is a required field.

Usage
-----
    differential/run.py                      # build, run, report; exit 1 on divergence
    differential/run.py --no-build            # reuse an existing example binary
    differential/run.py --filter paths        # only cases whose id/area matches
    differential/run.py --json report.json    # also write a machine-readable report
    differential/run.py --update-expected     # refresh `expected` from the oracle

Requires only a Python 3.9+ stdlib interpreter and a cargo toolchain.
"""

from __future__ import annotations

import argparse
import json
import ntpath  # noqa: F401  -- in scope for case `python` snippets
import posixpath  # noqa: F401  -- in scope for case `python` snippets
import pathlib  # noqa: F401  -- in scope for case `python` snippets
import subprocess
import sys
from pathlib import Path
from typing import Any, Optional

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
CASES = HERE / "cases.json"

# Oracles whose mismatch fails the build. `open` is advisory by design: those
# cases exist to surface questions the spec has not answered, and gating CI on an
# undecided question would make the harness unactionable.
BLOCKING_ORACLES = {"cpython", "spec", "invariant"}


class Colors:
    """ANSI colors, suppressed when stdout is not a TTY (e.g. CI logs)."""

    def __init__(self, enabled: bool) -> None:
        self.red = "\033[31m" if enabled else ""
        self.green = "\033[32m" if enabled else ""
        self.yellow = "\033[33m" if enabled else ""
        self.cyan = "\033[36m" if enabled else ""
        self.bold = "\033[1m" if enabled else ""
        self.dim = "\033[2m" if enabled else ""
        self.off = "\033[0m" if enabled else ""


def load_cases() -> list[dict[str, Any]]:
    with CASES.open(encoding="utf-8") as fh:
        data = json.load(fh)
    cases = data.get("cases")
    if not isinstance(cases, list) or not cases:
        sys.exit(f"{CASES}: no 'cases' array")

    seen: set[str] = set()
    for case in cases:
        for field in ("id", "expr", "oracle", "python"):
            if field not in case:
                sys.exit(f"{CASES}: case {case.get('id', '<no id>')!r} missing {field!r}")
        cid = case["id"]
        if cid in seen:
            # Duplicate ids would silently shadow each other when results are
            # keyed by id, quietly dropping a case from the run.
            sys.exit(f"{CASES}: duplicate case id {cid!r}")
        seen.add(cid)
    return cases


def python_reference(case: dict[str, Any]) -> tuple[Optional[str], Optional[str], Optional[str]]:
    """Evaluate a case's `python` snippet.

    Returns `(value, type_name, error)`. Rendering goes through `str()` so the
    comparison is against Python's user-visible form -- the same thing
    openjd-expr's `to_display_string()` produces -- which is what makes display
    bugs like exponent spelling visible instead of normalized away.

    The snippets come from a repo-controlled file, not from user input; `eval` is
    the point rather than a shortcut, since it lets a case express its reference
    as ordinary Python (`'ab'.center(5)`, `ntpath.join(...)`).
    """
    snippet = case["python"]
    try:
        value = eval(snippet, {"ntpath": ntpath, "posixpath": posixpath, "pathlib": pathlib})  # noqa: S307
    except Exception as exc:  # noqa: BLE001 -- a raising oracle is a real datum
        return None, None, f"{type(exc).__name__}: {exc}"

    type_name = {bool: "bool", int: "int", float: "float", str: "string"}.get(
        type(value), type(value).__name__
    )
    # bool before int: bool is an int subclass, and dict lookup on type() is
    # exact so this is safe, but the ordering documents the intent.
    #
    # Python renders booleans as True/False and EXPR as true/false. That is a
    # surface-syntax difference between two languages, not a divergence, so
    # normalize rather than reporting it as a bug on every boolean case.
    if isinstance(value, bool):
        return ("true" if value else "false"), type_name, None
    return str(value), type_name, None


def build_example(colors: Colors) -> Path:
    print(f"{colors.dim}Building differential_eval example...{colors.off}")
    proc = subprocess.run(
        ["cargo", "build", "-p", "openjd-expr", "--example", "differential_eval",
         "--message-format=json"],
        cwd=REPO,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        sys.stderr.write(proc.stderr)
        sys.exit(f"{colors.red}cargo build failed{colors.off}")

    # Parse cargo's JSON output for the artifact path rather than guessing
    # target/debug/examples/, which is wrong under CARGO_TARGET_DIR, custom
    # profiles, or cross-compilation.
    path: Optional[str] = None
    for line in proc.stdout.splitlines():
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        if msg.get("reason") == "compiler-artifact" and msg.get("executable"):
            if msg.get("target", {}).get("name") == "differential_eval":
                path = msg["executable"]
    if not path:
        sys.exit("could not determine the differential_eval binary path from cargo output")
    return Path(path)


def find_example() -> Path:
    candidates = sorted(REPO.glob("target/*/examples/differential_eval*"))
    candidates = [c for c in candidates if c.is_file() and c.suffix in ("", ".exe")]
    if not candidates:
        sys.exit("no prebuilt differential_eval found; drop --no-build")
    return candidates[0]


def run_rust(binary: Path, cases: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    payload = {
        "cases": [
            {"id": c["id"], "expr": c["expr"], **({"path_format": c["path_format"]} if "path_format" in c else {})}
            for c in cases
        ]
    }
    proc = subprocess.run(
        [str(binary)], input=json.dumps(payload), capture_output=True, text=True, cwd=REPO
    )
    if proc.returncode != 0:
        sys.stderr.write(proc.stderr)
        sys.exit(f"differential_eval exited {proc.returncode}")
    try:
        results = json.loads(proc.stdout)["results"]
    except (json.JSONDecodeError, KeyError) as exc:
        sys.stderr.write(proc.stdout[:2000])
        sys.exit(f"could not parse differential_eval output: {exc}")
    return {r["id"]: r for r in results}


def adjudicate(case: dict[str, Any], rust: dict[str, Any]) -> dict[str, Any]:
    """Compare one case and classify the outcome.

    The `expected` field, when present, is the authority -- it is the reviewed,
    committed answer. `python` is the oracle used to derive it and is always
    evaluated so the report can show all three values and flag drift between the
    committed expectation and what the oracle says today (e.g. after a CPython
    upgrade changes a float repr).
    """
    py_value, py_type, py_error = python_reference(case)
    oracle = case["oracle"]

    expected = case.get("expected")
    # Only an explicitly stated `expected_type` is a type expectation. The
    # oracle's own Python type is NOT usable as one: Python has no PATH type, so
    # `ntpath.join(...)` is a `str` while the correct EXPR result is a `path`.
    # Inferring from the oracle would fail every path case on a type difference
    # that is a property of the two type systems, not a bug. Cases that mean to
    # pin a type (float-floordiv-return-type) say so.
    expected_type = case.get("expected_type")
    if expected is None and oracle != "open":
        # No committed expectation: fall back to the live oracle. Only sound for
        # cpython/spec-derived values; `invariant` cases always carry `expected`.
        expected = py_value

    actual = rust.get("value") if rust.get("ok") else None
    actual_type = rust.get("type")

    if not rust.get("ok"):
        status = "error"
        detail = rust.get("error", "")
    elif expected is None:
        status = "info"
        detail = "no expectation recorded"
    else:
        value_match = actual == expected
        # A type mismatch only counts when an expectation was recorded, so a
        # case that pins only the value (float-floordiv-tie) is not failed by
        # the oracle's differing return type.
        type_match = expected_type is None or actual_type == expected_type
        if value_match and type_match:
            status = "pass"
            detail = ""
        else:
            status = "fail"
            bits = []
            if not value_match:
                bits.append(f"value: got {actual!r}, want {expected!r}")
            if not type_match:
                bits.append(f"type: got {actual_type!r}, want {expected_type!r}")
            detail = "; ".join(bits)

    if oracle == "open" and status in ("fail", "error"):
        # Undecided by design: report, never gate.
        status = "info"

    blocking = oracle in BLOCKING_ORACLES and status in ("fail", "error")

    return {
        "id": case["id"],
        "area": case.get("area", ""),
        "expr": case["expr"],
        "oracle": oracle,
        "spec_ref": case.get("spec_ref"),
        "status": status,
        "blocking": blocking,
        "actual": actual,
        "actual_type": actual_type,
        "expected": expected,
        "expected_type": expected_type,
        "python_value": py_value,
        "python_type": py_type,
        "python_error": py_error,
        "detail": detail,
        "note": case.get("note", ""),
    }


def report(outcomes: list[dict[str, Any]], colors: Colors) -> None:
    order = {"fail": 0, "error": 1, "info": 2, "pass": 3}
    icons = {
        "fail": f"{colors.red}FAIL{colors.off}",
        "error": f"{colors.red}ERR {colors.off}",
        "info": f"{colors.yellow}INFO{colors.off}",
        "pass": f"{colors.green}PASS{colors.off}",
    }

    print(f"\n{colors.bold}Differential results: openjd-expr vs. oracle{colors.off}")
    print(f"{colors.dim}{'-' * 72}{colors.off}")

    for out in sorted(outcomes, key=lambda o: (order[o["status"]], o["id"])):
        print(f"{icons[out['status']]}  {colors.bold}{out['id']}{colors.off}"
              f"  {colors.dim}[{out['oracle']}]{colors.off}")
        print(f"      expr     {out['expr']}")
        if out["status"] in ("fail", "error", "info"):
            if out["status"] == "error":
                print(f"      error    {out['detail']}")
            else:
                print(f"      actual   {out['actual']!r} ({out['actual_type']})")
                print(f"      expected {out['expected']!r}"
                      + (f" ({out['expected_type']})" if out["expected_type"] else ""))
            # Show the oracle separately from the expectation: when they differ
            # (spec cases) that difference is the interesting part.
            if out["python_error"]:
                print(f"      cpython  raises {out['python_error']}")
            elif out["python_value"] != out["expected"]:
                print(f"      cpython  {out['python_value']!r} ({out['python_type']})"
                      f"  {colors.dim}<- differs from expectation{colors.off}")
            if out["spec_ref"]:
                print(f"      spec     {out['spec_ref']}")
            if out["note"]:
                print(f"      {colors.dim}{out['note']}{colors.off}")
        print()

    counts = {k: sum(1 for o in outcomes if o["status"] == k) for k in order}
    blocking = [o for o in outcomes if o["blocking"]]
    print(f"{colors.dim}{'-' * 72}{colors.off}")
    print(f"{len(outcomes)} cases: "
          f"{colors.green}{counts['pass']} pass{colors.off}, "
          f"{colors.red}{counts['fail']} fail{colors.off}, "
          f"{colors.red}{counts['error']} error{colors.off}, "
          f"{colors.yellow}{counts['info']} info{colors.off}")

    if blocking:
        print(f"\n{colors.red}{colors.bold}{len(blocking)} blocking divergence(s):{colors.off}")
        for out in blocking:
            print(f"  - {out['id']} ({out['area']}): {out['detail']}")
        print(f"\n{colors.dim}These are silent-wrong-value divergences: each expression{colors.off}")
        print(f"{colors.dim}evaluates successfully and returns a wrong result.{colors.off}")
    if counts["info"]:
        print(f"\n{colors.yellow}Advisory (spec questions, non-gating):{colors.off}")
        for out in outcomes:
            if out["status"] == "info":
                print(f"  - {out['id']}: {out['detail'] or 'see note'}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--no-build", action="store_true", help="reuse an existing example binary")
    ap.add_argument("--filter", help="only run cases whose id or area contains this substring")
    ap.add_argument("--json", metavar="PATH", help="write a machine-readable report")
    ap.add_argument("--update-expected", action="store_true",
                    help="rewrite cases.json `expected` from the oracle (review the diff!)")
    ap.add_argument("--no-color", action="store_true")
    args = ap.parse_args()

    colors = Colors(sys.stdout.isatty() and not args.no_color)

    cases = load_cases()
    if args.filter:
        needle = args.filter.lower()
        cases = [c for c in cases
                 if needle in c["id"].lower() or needle in c.get("area", "").lower()]
        if not cases:
            sys.exit(f"no cases matched {args.filter!r}")

    binary = find_example() if args.no_build else build_example(colors)
    rust_results = run_rust(binary, cases)

    missing = [c["id"] for c in cases if c["id"] not in rust_results]
    if missing:
        sys.exit(f"differential_eval returned no result for: {', '.join(missing)}")

    outcomes = [adjudicate(c, rust_results[c["id"]]) for c in cases]

    if args.update_expected:
        with CASES.open(encoding="utf-8") as fh:
            doc = json.load(fh)
        by_id = {o["id"]: o for o in outcomes}
        for case in doc["cases"]:
            out = by_id.get(case["id"])
            if out and out["oracle"] in ("cpython", "spec") and out["python_value"] is not None:
                case["expected"] = out["python_value"]
                case["expected_type"] = out["python_type"]
        with CASES.open("w", encoding="utf-8") as fh:
            json.dump(doc, fh, indent=2, ensure_ascii=False)
            fh.write("\n")
        print(f"{colors.cyan}cases.json updated -- review the diff before committing.{colors.off}")
        return 0

    report(outcomes, colors)

    if args.json:
        Path(args.json).write_text(json.dumps({"outcomes": outcomes}, indent=2), encoding="utf-8")
        print(f"\n{colors.dim}Report written to {args.json}{colors.off}")

    return 1 if any(o["blocking"] for o in outcomes) else 0


if __name__ == "__main__":
    sys.exit(main())
