#!/usr/bin/env bash
set -euo pipefail

REGISTRATION_COMMIT="d7f32d45c181082e01e38b4cfb529f9eed8da18a"
MAPPING_ID="A2-cgroup-linear-v1"
ROOT="${H3_CGROUP_ROOT:-/sys/fs/cgroup/evolution-contract-h3}"
CPU="${H3_CPU:-0}"
WINDOW="${H3_WINDOW_SECONDS:-5}"
TRIAL_ID="${1:?trial id required}"
EFFICIENCY="${2:?efficiency required}"
OUT="${3:?output json required}"

weight_for() {
  local e="$1"
  echo $((100 + (9900 * e / 255)))
}

CONTROL_WEIGHT=100
INTERVENTION_WEIGHT="$(weight_for "$EFFICIENCY")"
TRIAL_ROOT="$ROOT/trial-$TRIAL_ID"
CONTROL="$TRIAL_ROOT/control"
INTERVENTION="$TRIAL_ROOT/intervention"

cleanup() {
  set +e
  [[ -n "${CONTROL_PID:-}" ]] && kill -TERM "$CONTROL_PID" 2>/dev/null || true
  [[ -n "${INTERVENTION_PID:-}" ]] && kill -TERM "$INTERVENTION_PID" 2>/dev/null || true
  [[ -n "${CONTROL_PID:-}" ]] && wait "$CONTROL_PID" 2>/dev/null || true
  [[ -n "${INTERVENTION_PID:-}" ]] && wait "$INTERVENTION_PID" 2>/dev/null || true
  [[ -d "$TRIAL_ROOT" ]] && rmdir "$CONTROL" "$INTERVENTION" "$TRIAL_ROOT" 2>/dev/null || true
}
trap cleanup EXIT

mkdir -p "$CONTROL" "$INTERVENTION"
printf '%s\n' "$CONTROL_WEIGHT" > "$CONTROL/cpu.weight"
printf '%s\n' "$INTERVENTION_WEIGHT" > "$INTERVENTION/cpu.weight"

python3 scripts/h3-workload.py >"$TRIAL_ROOT/control.log" 2>&1 & CONTROL_PID=$!
python3 scripts/h3-workload.py >"$TRIAL_ROOT/intervention.log" 2>&1 & INTERVENTION_PID=$!

for _ in {1..100}; do
  [[ -d "/proc/$CONTROL_PID" && -d "/proc/$INTERVENTION_PID" ]] && break
  sleep 0.01
done

[[ -d "/proc/$CONTROL_PID" && -d "/proc/$INTERVENTION_PID" ]] || { echo "process start failed" >&2; exit 1; }

echo "$CONTROL_PID" > "$CONTROL/cgroup.procs"
echo "$INTERVENTION_PID" > "$INTERVENTION/cgroup.procs"

taskset -pc "$CPU" "$CONTROL_PID" >/dev/null
taskset -pc "$CPU" "$INTERVENTION_PID" >/dev/null

actual_control_weight="$(cat "$CONTROL/cpu.weight")"
actual_intervention_weight="$(cat "$INTERVENTION/cpu.weight")"
actual_control_affinity="$(taskset -pc "$CONTROL_PID" 2>/dev/null | sed 's/.*: //')"
actual_intervention_affinity="$(taskset -pc "$INTERVENTION_PID" 2>/dev/null | sed 's/.*: //')"

[[ "$actual_control_weight" == "$CONTROL_WEIGHT" ]] || { echo "control weight mismatch" >&2; exit 1; }
[[ "$actual_intervention_weight" == "$INTERVENTION_WEIGHT" ]] || { echo "intervention weight mismatch" >&2; exit 1; }
[[ "$actual_control_affinity" == "$actual_intervention_affinity" ]] || { echo "CPU affinity mismatch" >&2; exit 1; }

before_control="$(awk '/usage_usec /{print $2}' "$CONTROL/cpu.stat")"
before_intervention="$(awk '/usage_usec /{print $2}' "$INTERVENTION/cpu.stat")"

kill -USR1 "$CONTROL_PID"
kill -USR1 "$INTERVENTION_PID"
sleep "$WINDOW"

after_control="$(awk '/usage_usec /{print $2}' "$CONTROL/cpu.stat")"
after_intervention="$(awk '/usage_usec /{print $2}' "$INTERVENTION/cpu.stat")"

control_delta=$((after_control - before_control))
intervention_delta=$((after_intervention - before_intervention))

kill -TERM "$CONTROL_PID" "$INTERVENTION_PID"
wait "$CONTROL_PID"; control_status=$?
wait "$INTERVENTION_PID"; intervention_status=$?

python3 - "$OUT" <<PY
import json
from pathlib import Path
control=$control_delta
intervention=$intervention_delta
obj={
  "trial_id": "$TRIAL_ID",
  "mapping_id": "$MAPPING_ID",
  "registration_commit": "$REGISTRATION_COMMIT",
  "efficiency": int("$EFFICIENCY"),
  "registered_control_cpu_weight": $CONTROL_WEIGHT,
  "registered_intervention_cpu_weight": $INTERVENTION_WEIGHT,
  "actual_control_cpu_weight": int("$actual_control_weight"),
  "actual_intervention_cpu_weight": int("$actual_intervention_weight"),
  "cpu_affinity": "$actual_control_affinity",
  "window_seconds": $WINDOW,
  "control_usage_usec_before": int("$before_control"),
  "control_usage_usec_after": int("$after_control"),
  "intervention_usage_usec_before": int("$before_intervention"),
  "intervention_usage_usec_after": int("$after_intervention"),
  "control_cpu_usec": control,
  "intervention_cpu_usec": intervention,
  "R": intervention / control if control > 0 else None,
  "control_exit_status": $control_status,
  "intervention_exit_status": $intervention_status,
  "valid": control > 0 and intervention > 0 and $control_status == 0 and $intervention_status == 0,
}
if not obj["valid"]:
    obj["invalidity_reason"]="nonzero_exit_or_zero_control_cpu"
Path("$OUT").write_text(json.dumps(obj, indent=2, sort_keys=True)+"\n")
PY

cat "$OUT"
