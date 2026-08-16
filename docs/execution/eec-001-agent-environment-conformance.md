# EEC-001 — Agent Environment Conformance

**Status:** implementation candidate; not yet a governance approval.

## Claim

Any mutation-capable agent execution MUST begin by establishing canonical repository identity and a conformant execution environment. If either identity verification or environment preflight fails, mutation MUST be blocked.

## Required sequence

```text
canonical repository identity
        ↓
exact commit / lineage
        ↓
Environment Preflight
        ↓
AGENT_ENVIRONMENT: READY
        ↓
mutation may begin
```

A failed or unavailable preflight is fail-closed.

## Reference enforcement surface

The reference implementation is `scripts/agent-environment-gate.sh`. An agent execution harness that performs repository mutation MUST invoke this gate before the first mutation.

The gate verifies:

1. the repository is `aisoundmastervv-cpu/evolution-contract`;
2. `.project/IDENTITY.md` is present and identifies the canonical repository;
3. the requested exact commit is `HEAD`;
4. the workspace is clean before mutation;
5. the Environment Contract is present;
6. the Environment Preflight returns `ENVIRONMENT: READY`.

## Mutation boundary

The gate does not make the filesystem immutable. Its purpose is to establish a clean, verified boundary **before** mutation. After mutation begins, a repeated preflight is expected to report the workspace as non-clean; that is a post-gate state, not evidence that the original gate was invalid.

## Non-goals

EEC-001 does not claim that GitHub can enforce behavior of an external agent runtime that never invokes the repository gate. Such enforcement belongs to the agent execution harness. GitHub Actions can provide conformance evidence for the reference gate, but cannot by itself control an unrelated ChatGPT/Codex runtime.

## Evidence requirement

A passing EEC-001 implementation must include a real CI execution showing:

- clean verified workspace;
- `AGENT_ENVIRONMENT: READY` before the mutation sentinel;
- mutation sentinel created only after the gate passes;
- a post-mutation gate rejection due to the expected dirty workspace;
- preserved logs as evidence.
