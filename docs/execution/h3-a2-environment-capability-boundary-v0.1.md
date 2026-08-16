# H3 A2 — Environment Capability Boundary v0.1

**Status:** observed execution boundary

## Decision

The first real H3 A2 execution candidate was materialized on the GitHub-hosted runner and reached the registered A2 preflight. The workload runner itself MUST NOT be modified in response to this result.

The checked execution surface is **not conformant for the registered A2 arm** because the required cgroup capability is unavailable:

```text
H3_PREFLIGHT: FAIL — cgroup root is not writable: /sys/fs/cgroup
exit_code=1
H3_STATUS=INCONCLUSIVE
```

Because preflight failed, no H3 workload trial was executed and no causal verdict was produced. This is therefore an **environment capability boundary**, not a H3 null result and not evidence against the registered causal criterion.

## Raw execution record

- Workflow: `H3 A2 Execution Candidate`
- Run: `31929006747`
- Execution commit: `1ae4e525fbb217caabcc162995171a8ae202e30e`
- Registered execution arm: `A2-cgroup-linear-v1`
- Registration commit: `d7f32d45c181082e01e38b4cfb529f9eed8da18a`
- Runner surface: GitHub-hosted `ubuntu-24.04`
- Job: `h3-a2-execution`
- Job conclusion: `success` (the workflow correctly handled the fail-closed preflight outcome)
- Evidence artifact: `h3-a2-execution-evidence-1ae4e525fbb217caabcc162995171a8ae202e30e`
- Artifact SHA-256: `2fdec634efaebf19ac49a0cc441107e62a05cfef7be7600925024f55f44c7a9d`

## Interpretation

The result establishes only:

> On the tested GitHub-hosted `ubuntu-24.04` execution surface, the registered A2 preflight cannot establish the cgroup capability required by `A2-cgroup-linear-v1`; therefore the execution is `INCONCLUSIVE` and must stop before workload trials.

This boundary is compatible with the fail-closed requirement of Environment Contract v0.1: a failed preflight MUST NOT be treated as a conformant execution environment.

## Governance invariants preserved

This observation does **not** authorize any of the following changes:

- no change to A2 registration;
- no change to the registered threshold;
- no change to the registered execution arm;
- no weakening of the preflight gate;
- no substitution of a different workload or measurement protocol;
- no retry solely to seek a different verdict on the same non-conformant execution surface.

## Evidence boundary

The result is sufficient to close H3 A2 preflight on this runner class as an environment-capability observation. It is **not** sufficient to claim:

- `CAUSAL SUPPORT`;
- `NULL-FALSIFICATION`;
- universal absence of the capability on all possible execution surfaces.

A future execution may proceed only on an execution surface that independently satisfies the existing A2 preflight without changing the registered arm or criterion.

## Evidence chain

`EEC-003` establishes environment-integrity enforcement. This execution establishes that the subsequent A2 arm is capability-gated on the tested runner surface. The two claims are distinct:

```text
EEC-003: environment identity drift -> mutation rejection
        |
        v
H3 A2 preflight: required cgroup capability unavailable
        |
        v
H3 workload: NOT MATERIALIZED
        |
        v
H3 verdict: INCONCLUSIVE (environment boundary)
```
