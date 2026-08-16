#!/usr/bin/env bash
set -euo pipefail

fail() { echo "H3_PREFLIGHT: FAIL — $*" >&2; exit 1; }

CGROUP_ROOT="${H3_CGROUP_ROOT:-/sys/fs/cgroup}"
[[ -f /sys/fs/cgroup/cgroup.controllers ]] || fail "cgroup v2 unavailable"
controllers="$(cat /sys/fs/cgroup/cgroup.controllers)"
[[ " $controllers " == *" cpu "* ]] || fail "cpu controller unavailable"

[[ -d "$CGROUP_ROOT" && -w "$CGROUP_ROOT" ]] || fail "cgroup root is not writable: $CGROUP_ROOT"
command -v taskset >/dev/null || fail "taskset unavailable"

# Test the exact A2 capability without changing the runner's controller configuration:
# a child cgroup must be creatable and cpu.weight must be writable/readable there.
probe="$CGROUP_ROOT/h3-preflight-$$"
cleanup() { rmdir "$probe" 2>/dev/null || true; }
trap cleanup EXIT
mkdir "$probe" || fail "cannot create child cgroup: $probe"

[[ -f "$probe/cpu.weight" ]] || fail "cpu controller not delegated to child cgroup"
printf '%s\n' 100 > "$probe/cpu.weight" || fail "cpu.weight is not writable in child cgroup"
actual_weight="$(cat "$probe/cpu.weight")"
[[ "$actual_weight" == 100 ]] || fail "cpu.weight readback mismatch: $actual_weight"

printf 'H3_PREFLIGHT: READY\n'
printf 'cgroup_root=%s\n' "$CGROUP_ROOT"
printf 'cgroup_type=%s\n' "$(cat /sys/fs/cgroup/cgroup.type 2>/dev/null || true)"
printf 'cgroup_controllers=%s\n' "$controllers"
printf 'runner_cpu=%s\n' "$(nproc)"
printf 'taskset=%s\n' "$(command -v taskset)"
printf 'cpu_weight_child_probe=100\n'
