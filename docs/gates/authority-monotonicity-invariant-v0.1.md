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
    AND VALID_EXECUTOR_SEMANTICS(C)
    AND EFFECTS(C) subset_of EFFECTS(P)
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
change the executor semantics under which effects are evaluated
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

## 4. Canonical Executor Semantics / Effect Model

`EFFECTS(X)` is not determined by the capability vector alone. The same vector can have different effective authority under different executor semantics.

Therefore every authority-bearing execution context MUST bind to an exact canonical Executor Semantics / Effect Model revision:

```text
executor_semantics_id
executor_semantics_revision
executor_semantics_hash
effect_model_id
effect_model_revision
effect_model_hash
```

These identities MUST be part of the canonical authorization lineage and MUST be immutable for the authorized execution.

The executor MUST evaluate effects only under the exact semantics/effect-model revision bound by the authorization. A runtime or executor MAY NOT silently substitute a different semantics revision.

### 4.1 Counterexample closed by this requirement

Consider the same capability vector:

```text
network = LOCAL_ONLY
inputs = D1
persistence = NONE
```

Under executor semantics `E1`, `LOCAL_ONLY` may permit access only to an unprivileged local service.

Under executor semantics `E2`, the same `LOCAL_ONLY` primitive may resolve to a privileged host control socket or another security-sensitive local endpoint.

The capability vector is identical, but:

```text
EFFECTS(vector, E1) != EFFECTS(vector, E2)
```

Therefore a capability vector without an exact executor-semantics binding cannot establish a unique effect set and MUST NOT be treated as sufficient authorization evidence.

### 4.2 Semantic determinacy invariant

For a fixed canonical semantics revision `S`:

```text
CAPABILITY = C
EXECUTOR_SEMANTICS = S
        =>
EFFECTS(C,S) is uniquely defined by the normative effect model
```

If two executions can legitimately produce different effect sets while claiming the same `(CAPABILITY, EXECUTOR_SEMANTICS)` pair, the effect model is under-specified and the authority model is not closed.

A semantics change that can alter authority-relevant effects requires a new canonical executor-semantics/effect-model revision and a new applicable governance event.

## 5. Semantic effects and composition closure

Dimension-wise containment is necessary but not sufficient. A combination of individually permitted dimensions MUST NOT create an effect that is absent from the parent authority.

`EFFECTS(X,S)` is the closed, machine-evaluable set of security-relevant and authority-relevant effects that execution of `X` can cause or obtain under exact executor semantics `S`.

For every valid child:

```text
EFFECTS(C,S_C) subset_of EFFECTS(P,S_P)
```

and, unless a new canonical governance event explicitly authorizes a changed semantics boundary:

```text
S_C = S_P
```

The effect relation MUST account for composition across dimensions. In particular, the evaluator MUST consider interactions such as:

```text
network + inputs
network + persistence
inputs + side_effects
resources + execution_count
executor + delegation
environment + network
```

A child is invalid if the combination of individually contained dimensions enables a new effective operation, resource, data flow, persistence path, delegation path, or side effect not contained in the parent.

Therefore, checking each dimension independently is not sufficient to establish monotonicity.

## 6. Semantic aliasing closure

Different representations MUST be normalized to the same semantic capability before containment is evaluated.

Aliases, defaults, derived values, indirect references, and equivalent executor primitives MUST NOT allow a child to evade the capability comparison.

Examples of aliasing that MUST resolve to their semantic effect include:

```text
network = localhost
network = loopback
network = socket-to-local-service
```

when they produce the same effective network capability;

and:

```text
resource = /tmp/file
persistence = filesystem-write
```

when the former necessarily grants the latter under the executor semantics.

The normalization function is part of the closed authority domain. A child cannot choose a representation whose semantics are stronger than the represented capability.

## 7. Monotonic authority flow

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

## 8. Token non-amplification

A future `AuthorizedExecutionToken` is a proof/reference derived from an already valid canonical `EXECUTION_AUTHORIZED` event. It is not an independent source of authority.

The token MUST satisfy both:

```text
CAPABILITY(TOKEN) subset_of CAPABILITY(AUTHORIZATION)
EFFECTS(TOKEN,S_TOKEN) subset_of EFFECTS(AUTHORIZATION,S_AUTHORIZATION)
S_TOKEN = S_AUTHORIZATION
```

unless a separately authorized semantics transition explicitly governs the change.

The token MUST NOT introduce a new contract, plan, baseline, oracle, executor specification, executor semantics, effect model, environment, input, resource boundary, side effect, network permission, persistence permission, delegation right, or execution count outside the parent authorization.

A token that cannot demonstrate these bindings is invalid.

## 9. Non-escalation across layers

For every authority-bearing transition:

```text
child_capability subset_of parent_capability
child_effects   subset_of parent_effects
child_semantics = parent_semantics
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

## 10. No implicit authority inheritance

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

## 11. Required executor rejection conditions

The executor MUST reject a derived authorization artifact if any of the following holds:

```text
parent invalid
capability dimension broadened
new capability dimension introduced
semantic effect broadened
capability composition creates a new effect
semantic aliasing hides a broader capability
executor semantics missing
executor semantics mismatch
executor semantics revision substituted
missing effect model
semantic effect model under-specified
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

## 12. Governance test obligations

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

### AUTH-MONO-010 — compositional amplification

Every individual child capability dimension is contained in the corresponding parent dimension, but their combination enables an effect not contained in the parent.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-011 — semantic aliasing

A child uses a representation, alias, default, indirect reference, or executor primitive that is syntactically different but semantically broader than the parent capability.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-012 — effect-set mismatch

A child passes all identity and dimension checks but its derived semantic effect set exceeds the parent's effect set.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-013 — executor-semantics substitution

The child preserves the same capability vector but is evaluated under a different executor-semantics/effect-model revision whose effective effects are broader.

Expected: `NOT_AUTHORIZED`.

### AUTH-MONO-014 — semantic nondeterminacy

Two executions claim the same capability vector and exact semantics revision but the normative effect model permits different authority-relevant effect sets.

Expected: `NOT_AUTHORIZED` until the effect model is made deterministic and canonical.

## 13. Status boundary

```text
DESIGN STATUS: DRAFT
ADMISSION STATUS: NOT ADMITTED
TOKEN IMPLEMENTATION: NOT AUTHORIZED
ENFORCEMENT IMPLEMENTATION: NOT AUTHORIZED
EXECUTION AUTHORIZATION: NOT GRANTED
```

This invariant is therefore a review object, not an authority source.
