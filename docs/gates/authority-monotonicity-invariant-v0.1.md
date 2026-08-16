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
    AND SCOPE(C) subset_of SCOPE(P)
    AND TIME(C) subset_of TIME(P)
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

## 2. Monotonic authority flow

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

## 3. Token non-amplification

A future `AuthorizedExecutionToken` is a proof/reference derived from an already valid canonical `EXECUTION_AUTHORIZED` event. It is not an independent source of authority.

The token MUST satisfy:

```text
TOKEN_SCOPE subset_of AUTHORIZED_SCOPE
TOKEN_TIME subset_of AUTHORIZED_TIME
TOKEN_OBJECT = AUTHORIZED_OBJECT
TOKEN_REVISION = AUTHORIZED_REVISION
```

The token MUST NOT introduce a new contract, plan, baseline, oracle, executor specification, environment, resource boundary, or execution count outside the parent authorization.

A token that cannot demonstrate these bindings is invalid.

## 4. Non-escalation across layers

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

## 5. No implicit authority inheritance

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

## 6. Required executor rejection conditions

The executor MUST reject a derived authorization artifact if any of the following holds:

```text
parent invalid
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

## 7. Governance test obligations

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

## 8. Status boundary

```text
DESIGN STATUS: DRAFT
ADMISSION STATUS: NOT ADMITTED
TOKEN IMPLEMENTATION: NOT AUTHORIZED
ENFORCEMENT IMPLEMENTATION: NOT AUTHORIZED
EXECUTION AUTHORIZATION: NOT GRANTED
```

This invariant is therefore a review object, not an authority source.
