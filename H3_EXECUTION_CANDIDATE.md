# H3 A2 Execution Candidate

**Status:** EXECUTION CANDIDATE — implementation only; no H3 outcome established.

**Registered arm:** `A2-cgroup-linear-v1`

**Registration commit:** `d7f32d45c181082e01e38b4cfb529f9eed8da18a`

This candidate implements the already-registered A2 protocol without changing its mapping, endpoint, trial count, validity rules, or acceptance threshold.

## Frozen protocol

- `efficiency=0` → `cpu.weight=100`
- `efficiency=255` → `cpu.weight=10000`
- five trials at each registered gene value
- identical frozen workload for control and intervention
- same CPU affinity
- fixed 5-second measurement window
- primary endpoint: cgroup `cpu.stat` `usage_usec`
- `R = intervention_cpu_usec / control_cpu_usec`

## Acceptance

The evaluator uses the registered criterion only:

- median `R(e=0)` must be within `[0.5, 2.0]`
- median `R(e=255)` must be at least `2.0 × median R(e=0)`
- all ten trials must be valid

Outcome is exactly one of `CAUSAL SUPPORT`, `NULL-FALSIFICATION`, or `INCONCLUSIVE`.

The runner does not silently fall back to the `A1-nice-v1` arm. If the A2 cgroup capability cannot be established, the execution is `INCONCLUSIVE`.
