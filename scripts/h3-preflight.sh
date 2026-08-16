#!/usr/bin/env bash
set -euo pipefail

fail() { echo "H3_PREFLIGHT: FAIL — $*" >&2; exit 1; }

[[ -f /sys/fs/cgroup/cgroup.controllers ]] || fail "cgroup v2 unavailable"
controllers="$(cat /sys/fs/cgroup/cgroup.controllers)"
[[ " $controllers " == *" cpu "* ]] || fail "cpu controller unavailable"

workload_root="${1:-/sys/fs/cgroup}"
[[ -d "$workload_root" && -w "$workload_root" ]] || fail "cgroup root is not writable: $workload_root"

cpu_weight_root="$(cat /sys/fs/cgroup/cgroup.controllers | tr ' ' '\n' | grep -Fx cpu >/dev/null && echo yes || echo no)"
[[ "$cpu_weight_root" == yes ]] || fail "cpu controller check failed"

command -v taskset >/dev/null || fail "taskset unavailable"

printf 'H3_PREFLIGHT: READY\n'
printf 'cgroup_type=%s\n' "$(cat /sys/fs/cgroup/cgroup.type 2>/dev/null || true)"
printf 'cgroup_controllers=%s\n' "$controllers"
printf 'runner_cpu=%s\n' "$(nproc)"
printf 'taskset=%s\n' "$(command -v taskset)"
