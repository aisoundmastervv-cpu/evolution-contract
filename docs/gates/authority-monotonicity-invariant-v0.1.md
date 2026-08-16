# Authority Monotonicity Invariant v0.1

Status: DRAFT / NOT ADMITTED
Parent design: `Authority Capability Model v0.1`

## Purpose

Define the invariant that authority-bearing layers may transmit or narrow authority, but may not create, extend, reinterpret, or amplify authority independently.

This document creates no authority and does not authorize token implementation or execution.

## 1. Core invariant

For every derived authorization artifact `C` and its parent canonical authority `P`:

```text
AUTHORIZED(C) =>
    VALID(P)
    AND CAPABILITY(C) subset_of CAPABILITY(P)
    AND OBJECT(C) = OBJECT(P)
    AND REVISION(C) = REVISION(P)
```

A child artifact may narrow a parent authority, but may not broaden it.

```text
child_authority <= parent_authority
```

No downstream mechanism may:

```text
create authority
extend authority
renew authority
replace authority
reinterpret authority
change the parent object/revision
```

Any broader authority requires a new canonical governance event at the appropriate authority domain.

## 2. Closed capability domain

`scope` alone is not sufficient to establish monotonicity. Authority MUST be evaluated over a closed capability domain appropriate to the authority type.

For execution authority, the capability vector is:

```text
CAPABILITY = {
    actor,
    action,
    object,
    revision,
    executor,
    environment,
    inputs,
    resources,
    side_effects,
    network,
    persistence,
    delegation,
    execution_count,
    temporal_bounds
}
```

Each dimension MUST have an explicitly defined value or an explicit bounded `NONE` / empty value. An unspecified dimension is not interpreted as unlimited.

The capability domain is closed for the relevant authority type: a new authority-bearing dimension cannot be introduced by a child artifact or token. Adding a new dimension requires a new canonical governance policy/design event.

## 3. Capability-vector monotonicity

For every dimension `d` in the closed capability domain:

```text
CAPABILITY(C)[d] <= CAPABILITY(P)[d]
```

where `<=` is the authority-specific containment relation defined for that dimension.

Examples:

```text
TOKEN_SCOPE         <= AUTHORIZED_SCOPE
TOKEN_TIME          <= AUTHORIZED_TIME
TOKEN_OBJECT        = AUTHORIZED_OBJECT
TOKEN_REVISION      = AUTHORIZED_REVISION
TOKEN_NETWORK       <= AUTHORIZED_NETWORK
TOKEN_RESOURCES     <= AUTHORIZED_RESOURCES
TOKEN_INPUTS        <= AUTHORIZED_INPUTS
TOKEN_SIDE_EFFECTS  <= AUTHORIZED_SIDE_EFFECTS
TOKEN_PERSISTENCE   <= AUTHORIZED_PERSISTENCE
TOKEN_DELEGATION    <= AUTHORIZED_DELEGATION
TOKEN_EXEC_COUNT    <= AUTHORIZED_EXEC_COUNT
```

A token/child that preserves identifiers but broadens any capability dimension is invalid.

## 4. Monotonic authority flow

The normative direction is:

```text
Canonical Authority Root
        |
        v
Governance Authority
        |
        v
Proposal Admission
        |
        v
Design Approval
        |
        v
Execution Authorization
        |
        v
AuthorizedExecutionToken
        |
        v
Executor Capability
        |
        v
Actual Execution
```

Authority flows downward only through explicitly bounded canonical transitions.

Evidence flows upward through append-only observation and independent evaluation.

Neither flow may cross the other's authority boundary.

## 5. Token non-amplification

A future `AuthorizedExecutionToken` is a proof/reference derived from an already valid canonical `EXECUTION_AUTHORIZED` event. It is not an independent source of authority.

The token MUST satisfy the complete capability-vector containment relation, not merely object/revision/scope/time equality:

```text
CAPABILITY(TOKEN) subset_of CAPABILITY(AUTHORIZATION)
```

The token MUST NOT introduce a new contract, plan, baseline, oracle, executor specification, environment, input, resource boundary, side effect, network permission, persistence permission, delegation right, or execution count outside the parent authorization.

A token that cannot demonstrate these bindings is invalid.

## 6. Non-escalation across layers

For every authority-bearing transition:

```text
child_capability subset_of parent_capability
```

In particular:

```text
Execution Subject
    cannot create Governance Authority

Authorization Artifact
    cannot create new Governance Authority

Executor
    cannot create Authorization

Evidence
    cannot create Authorization

Verdict
    cannot retroactively create Authorization
```

## 7. No implicit authority inheritance

Authority does not propagate merely because two artifacts share:

- an object ID;
- a branch;
- a repository;
- a workflow;
- a filename;
- a proposal ID;
- a human-readable status;
- an execution environment.

Propagation requires an explicit canonical parent relation and a valid bounded derivation.

## 8. Required executor rejection conditions

The executor MUST reject a derived authorization artifact if any of the following holds:

```text
parent invalid
capability dimension broadened
new capability dimension introduced
scope broadened
validity broadened
object changed
revision changed
authorization revoked
parent lineage incomplete
issuer authority unverified
token claims exceed parent claims
```

The rejection result is:

```text
NOT_AUTHORIZED
```

The executor must not infer authorization from operational success, artifact existence, or subject claims.

## 9. Governance test obligations

The following adversarial cases must remain false:

### AUTH-MONO-001 — scope amplification

Child scope is broader than parent scope.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-002 — temporal amplification

Child validity exceeds parent validity.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-003 — object substitution

Child refers to a different object or revision.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-004 — token-created authority

Token claims an authority not present in its parent authorization.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-005 — execution-to-authority feedback

Execution output is used as evidence that execution was authorized.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-006 — verdict-to-authority feedback

A verdict is used to retroactively establish execution authorization.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-007 — semantic capability amplification

Child preserves object, revision, and nominal scope but broadens any capability dimension, such as network, resources, inputs, side effects, persistence, delegation, or execution count.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-008 — implicit unlimited dimension

A capability dimension is omitted from the child and interpreted as unlimited or inherited without an explicit bounded value.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-009 — capability-domain expansion

A child introduces a capability dimension that is not part of the parent's closed capability domain.

Expected: `NOT_AUTHORIZED`.

## 10. Status boundary

```text
DESIGN STATUS: DRAFT
ADMISSION STATUS: NOT ADMITTED
TOKEN IMPLEMENTATION: NOT AUTHORIZED
ENFORCEMENT IMPLEMENTATION: NOT AUTHORIZED
EXECUTION AUTHORIZATION: NOT GRANTED
```

This invariant is therefore a review object, not an authority source.
