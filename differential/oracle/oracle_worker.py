# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# Copyright by contributors to this project.
# SPDX-License-Identifier: (Apache-2.0 OR MIT)

"""Long-lived differential oracle for the Python OpenJD expression reference.

The Rust differential harness spawns this once and speaks a trivial
newline-delimited JSON protocol over stdin/stdout: one request object per line
in, one response object per line out, in the same order. Paying interpreter and
import startup a single time lets the harness push tens of thousands of cases
through without per-case process overhead.

Request  (one JSON object per line):
    {"expr": "<source>", "symbols": {..}|null, "path_format": "POSIX"|"WINDOWS"|null}

Response (one JSON object per line):
    {"ok":  {"type": "<exprtype>", "value": <str | nested-array-of-str>}}   # evaluated
    {"err": {"category": "<mapped-category>"}}                              # raised ExpressionError
    {"internal_error": "<repr>"}                                           # oracle bug, NOT a finding

The `ok.value` shape is byte-for-byte the same transport format the Rust side
produces via `ExprValue::to_json_transport()`: a `type` string (e.g. `int`,
`float`, `list[int]`, `path`) plus a `value` that is the display string for
scalars and a nested array of display strings for lists. The Rust harness
compares these tags structurally, so the two sides agree iff they computed the
same typed value. Type is carried separately from value on purpose: that is
exactly what makes `2**63` (int, an error here) distinguishable from `2.0**63`
(float) rather than both collapsing to a bare number.

Error *messages* are deliberately NOT returned. Python raises a flat
`ExpressionError`/`ExpressionTypeError`, while Rust carries a rich
`ExpressionErrorKind` enum; the wordings will never match and comparing them
produces only false positives. The harness treats (err, err) as agreement
regardless of category; the coarse category below is emitted for triage/logging
only.
"""

from __future__ import annotations

import json
import sys
import traceback
from typing import Any, Optional

# The reference is a namespace package under `src/`; the harness sets PYTHONPATH
# to the reference checkout's `src` before spawning us.
from openjd.expr._eval import evaluate_expression
from openjd.expr._errors import ExpressionError
from openjd.expr._path_mapping import PathFormat
from openjd.expr._types import TypeCode


def _path_format(name: Optional[str]) -> Optional[PathFormat]:
    if name is None:
        return None
    return PathFormat[name]  # "POSIX" | "WINDOWS"; KeyError surfaces as internal_error


def _transport_value(v: Any) -> Any:
    """Mirror Rust `ExprValue::transport_value`.

    Scalars become their display string (`to_string`); lists become a nested
    array of the elements' transport values. `to_string` is the same
    canonicalizer Rust's `to_display_string` uses, so `1` stays `"1"` (int) and
    `1.0` stays `"1.0"` (float) — the type tag, not the string, distinguishes
    them.

    One deliberate exception: a null value. Rust's *transport* form for null is
    the literal `"null"` (it round-trips back to `ExprValue::Null` via
    `from_str_coerce`), whereas Python's `to_string()` renders null as the empty
    string for human display. Both sides hold the same null value; emit Rust's
    transport spelling so the tags compare equal rather than flagging a spurious
    `"" != "null"` divergence. `null` carries no type parameters, so this is
    checked before the list branch.
    """
    if v.is_null:
        return "null"
    tc = v.type.type_code
    if tc == TypeCode.LIST:
        return [_transport_value(e) for e in v.to_expr_value_list()]
    return v.to_string()


def _transport_tag(v: Any) -> dict[str, Any]:
    """Mirror Rust `ExprValue::to_json_transport`: {"type": str, "value": ...}."""
    return {"type": str(v.type), "value": _transport_value(v)}


def _error_category(exc: ExpressionError) -> str:
    """Coarse, best-effort bucket for triage only — never used for pass/fail.

    Python has no structured error kind, so this sniffs the message. It exists
    so a human reading a divergence log can see "both erred, roughly here"
    without implying the categories are authoritative or comparable to Rust's.
    """
    msg = str(exc).lower()
    if "overflow" in msg:
        return "overflow"
    if "by zero" in msg or "division" in msg:
        return "division_by_zero"
    if "undefined" in msg or "not found" in msg:
        return "undefined_symbol"
    if "syntax" in msg:
        return "parse"
    if "index" in msg or "out of range" in msg or "out of bounds" in msg:
        return "index"
    if "type" in msg or type(exc).__name__ == "ExpressionTypeError":
        return "type"
    return "other"


def _handle(req: dict[str, Any]) -> dict[str, Any]:
    expr = req["expr"]
    symbols = req.get("symbols")
    path_format = _path_format(req.get("path_format"))
    try:
        value = evaluate_expression(
            expr,
            values=symbols if symbols else None,
            path_format=path_format,
        )
    except ExpressionError as exc:
        return {"err": {"category": _error_category(exc)}}
    # Any *other* exception is a bug in the reference or the oracle, not a
    # divergence. Tag it so the harness can skip (not fail) the case and a human
    # can investigate the oracle rather than the Rust port.
    return {"ok": _transport_tag(value)}


def main() -> None:
    out = sys.stdout
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
            resp = _handle(req)
        except ExpressionError:
            # Belt-and-suspenders: _handle already catches these, but keep the
            # loop alive on any that slip through.
            resp = {"err": {"category": "other"}}
        except Exception:  # noqa: BLE001 - report, don't crash the worker
            resp = {"internal_error": traceback.format_exc(limit=3)}
        out.write(json.dumps(resp))
        out.write("\n")
        out.flush()


if __name__ == "__main__":
    main()
