#!/usr/bin/env bash
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# Copyright by contributors to this project.
# SPDX-License-Identifier: (Apache-2.0 OR MIT)
#
# Runs the verifier capability probe from docs/dev/quality-criteria.md §6
# "Verifier capability": writes a scratch integration test that calls one
# public item of <crate>, runs it with cargo, confirms that it passed, deletes
# it, and confirms that `git status --short` no longer lists it. Only the
# probe's own file is checked, so verifiers probing the same tree at once do
# not see each other's probes as a change.
#
# Usage: scripts/audit_probe.sh <crate> <id>
#
#   <crate>  openjd-expr, openjd-model, openjd-sessions, or openjd-snapshots
#            (use openjd-expr when no crate is under review)
#   <id>     unique to the verifier: letters, digits, and underscores
#
# Exits 0 and prints "capability probe passed" on success. On failure, prints
# "ENVIRONMENT FAILURE", the step that failed, the command, and its output,
# and exits 1. Exits 2 on a usage error.

set -euo pipefail

usage() {
    echo "usage: scripts/audit_probe.sh <crate> <id>" >&2
    echo "  <crate>: openjd-expr, openjd-model, openjd-sessions, or openjd-snapshots" >&2
    echo "  <id>: letters, digits, and underscores, unique to the verifier" >&2
    exit 2
}

[[ $# -eq 2 ]] || usage
crate=$1
id=$2
[[ $id =~ ^[A-Za-z0-9_]+$ ]] || usage

case $crate in
    openjd-expr)
        body='assert_eq!(openjd_expr::escape_format_string("plain"), "plain");' ;;
    openjd-model)
        body='assert!(openjd_model::decode_job_template(
        serde_json::json!({"specificationVersion": "jobtemplate-2023-09", "name": "Probe", "steps": [{"name": "S", "script": {"actions": {"onRun": {"command": "echo"}}}}]}),
        None,
        &openjd_model::CallerLimits::default(),
    ).is_ok());' ;;
    openjd-sessions)
        body='assert_eq!(openjd_sessions::SessionLimits::default(), openjd_sessions::SessionLimits::default());' ;;
    openjd-snapshots)
        body='assert!(openjd_snapshots::decode_manifest("{}").is_err());' ;;
    *)
        usage ;;
esac

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
file="$ROOT/crates/$crate/tests/audit_probe_$id.rs"
test_cmd="cargo test -p $crate --test audit_probe_$id"

if [[ -e $file ]]; then
    echo "audit_probe.sh: $file already exists; pick another <id>" >&2
    exit 2
fi

fail() {
    echo "ENVIRONMENT FAILURE" >&2
    echo "step: $1" >&2
    echo "command: $2" >&2
    if [[ -n ${3:-} ]]; then
        echo "output:" >&2
        printf '%s\n' "$3" >&2
    fi
    exit 1
}

# The scratch file goes however the script exits.
trap 'rm -f "$file"' EXIT

# Step 1: write the probe.
cat >"$file" <<EOF ||
// Scratch capability probe written by scripts/audit_probe.sh. Delete it if
// it is left behind.

#[test]
fn audit_probe() {
    $body
}
EOF
    fail "1 (write the probe)" "write $file"

# Step 2: run it, and confirm it ran and passed.
if ! out=$(cd "$ROOT" && $test_cmd 2>&1); then
    fail "2 (run the probe)" "$test_cmd" "$out"
fi
grep -q "test result: ok\. 1 passed" <<<"$out" ||
    fail "2 (run the probe: expected '1 passed')" "$test_cmd" "$out"

# Step 3: delete it, and confirm git no longer sees it.
rm -f "$file" || fail "3 (delete the probe)" "rm -f $file"
if ! left=$(git -C "$ROOT" status --short -- "$file" 2>&1); then
    fail "3 (check the tree)" "git -C $ROOT status --short -- $file" "$left"
fi
if [[ -n $left ]]; then
    fail "3 (the probe is still in the tree)" "git -C $ROOT status --short -- $file" "$left"
fi

echo "capability probe passed: $crate ($test_cmd)"
