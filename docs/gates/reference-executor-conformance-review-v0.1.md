# Reference Executor Conformance Review v0.1

**Reviewed implementation:** `src/validation_executor.rs`  
**Implementation commit:** `d1a80445138d54e9bd114cfb9255b957cc90b28d`  
**Normative specification:** `docs/gates/reference-executor-spec-v0.1.md`  
**Approved specification:** `64a77553d9f259a902ffe1bc82b575c820afb7de`  
**State Model:** `Validation Machine State Model v0.1`  
**State Model approval:** `c51d850185554adc676200ca34d04a31bcc56f10`

## Review type

Static source conformance review against the approved Reference Executor Specification.

No Contract, Test Plan, harness, or production semantic changes were authorized by this review. No cloud execution was performed. This review does not claim a passing runtime test suite; runtime execution remains a separate evidence step.

## Verdict

**CHANGES REQUESTED**

The implementation correctly establishes the core transition guard, terminal-state behavior, epistemic-gap path, verdict separation, and `EXECUTION_ABORTED` distinction. However, one conformance gap prevents approval.

## Finding ECR-001 — transition-chain observability is not preserved

The approved State Model requires unauthorized requests to resolve through:

```text
FORBIDDEN_TRANSITION
    -> NOT_AUTHORIZED
    -> STOP
```

The implementation performs these assignments internally, but `ExecutionOutcome` exposes only:

```text
Transitioned(MachineState)
```

and therefore returns only the final `STOP` state for an unauthorized request. The intermediate governance states are not observable in the executor result or an execution trace.

This creates an evidence gap: a downstream conformance observer cannot independently distinguish:

```text
unauthorized request
    -> guard fired
    -> FORBIDDEN_TRANSITION
    -> NOT_AUTHORIZED
    -> STOP
```

from an implementation that simply maps an invalid request directly to `STOP`.

The issue is not that the internal assignments are semantically wrong. The issue is that the executor does not preserve sufficient execution evidence to establish that the approved transition chain was actually enforced.

## Required disposition

Revise the executor implementation minimally so that an unauthorized transition produces an auditable execution result containing the required transition chain, without adding a new State Model state or changing the approved semantics.

A suitable implementation-neutral shape is a transition trace/result such as:

```text
FORBIDDEN_TRANSITION
-> NOT_AUTHORIZED
-> STOP
```

The exact data structure may vary, but the implementation must make the chain independently inspectable by conformance tests.

No new semantic predicate is required.

## Conformance observations that already pass source review

- Authorized core transitions are explicitly enumerated.
- Unauthorized transitions are rejected before the requested action executes.
- Observation gaps reach `OBSERVATION_GAP -> STOP`.
- `UNDERDETERMINED` reaches `STOP`.
- Verdict classification is distinct from machine state.
- `UNTESTED` is not inferred from execution abort.
- `EXECUTION_ABORTED` preserves the current machine state and produces no semantic verdict.
- Terminal `STOP` does not permit implicit retry.
- The implementation contains no Contract-level metadata semantics and no new semantic predicates.

## Frozen-layer status

```text
Contract                 UNCHANGED
C-LG / O-LG              UNCHANGED
Test Plan v1.1           UNCHANGED
Harness                  UNCHANGED
State Model v0.1         UNCHANGED
Executor Specification   UNCHANGED
Cloud                    NOT AUTHORIZED
```

## Governance disposition

```text
Reference Executor implementation    CHANGES REQUESTED
Conformance approval                 NOT AUTHORIZED
Next action                          Minimal implementation revision
```

The implementation must not be declared conformant until ECR-001 is resolved and a subsequent conformance review is performed.
