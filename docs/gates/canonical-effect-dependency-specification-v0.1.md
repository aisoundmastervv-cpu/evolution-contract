# Canonical Effect Dependency Specification v0.1

Status: DRAFT / NOT ADMITTED
Parent design: `Authority Capability Model v0.1`
Related invariant: `Authority Monotonicity Invariant v0.1`

## Purpose

Define the canonical semantic boundary used to determine whether execution dependencies are either:

1. causally accounted for in authority-relevant effects; or
2. proven noninterfering with respect to the canonical effect semantics.

This document is a governance/design object only. It creates no authority and does not authorize executor, token, enforcement, or execution implementation.

## 1. Normative boundary

Authorization is valid only if every authority-relevant effect is causally accounted for by the canonical dependency closure, and every dependency outside that closure is proven noninterfering with respect to the canonical effect semantics.

```text
AUTHORIZATION_VALID =>
    EFFECT_DEPENDENCIES ⊆ CANONICAL_CLOSURE
    AND
    ∀ d ∉ CANONICAL_CLOSURE:
        PROVEN_NONINTERFERING(d)
```

An unresolved dependency is not treated as irrelevant.

```text
UNRESOLVED => NOT_AUTHORIZED
```

## 2. Canonical effect semantics

Authority-relevant effects are defined relative to a canonical semantic function:

```text
E = F(C, S, D)
```

where:

- `C` = capability vector;
- `S` = exact executor/effect semantics;
- `D` = execution dependency context;
- `E` = complete authority-relevant effect set.

The specification MUST define the effect universe sufficiently to distinguish authority-relevant changes.

For any executor whose authority-relevant behavior depends on execution history, the state-based representation MUST be lifted to canonical trace semantics:

```text
τ = (s0, a1, s1, a2, s2, ..., an, sn)
Eτ = G(C, S, D, τ)
```

where `τ` is the canonical execution trace and `G` determines authority-relevant effects over the trace.

A single-step projection MUST NOT be assumed sufficient merely because every individual step is within the declared effect boundary.

## 3. Causal relevance criterion

A dependency `d` is causally relevant iff there exist two admissible values `d1` and `d2`, with all other authorization inputs held equal, such that:

```text
F(C, S, D[d:=d1]) != F(C, S, D[d:=d2])
```

For trace-based semantics, the corresponding criterion is:

```text
G(C,S,D[d:=d1],τ) != G(C,S,D[d:=d2],τ)
```

or the dependency changes the set of admissible traces:

```text
TRACES(C,S,D[d:=d1]) != TRACES(C,S,D[d:=d2])
```

If either condition holds, `d` MUST be included in the canonical dependency closure and bound by the authorization lineage.

The test is semantic, not syntactic. A dependency cannot be excluded merely because it is absent from a configuration file, interface, or declaration.

## 4. Independence criterion

A dependency may be classified `INDEPENDENT` only when the system has a valid justification that, over its allowed domain, it cannot alter authority-relevant effects.

For state-based semantics:

```text
∀ d1,d2:
    F(C, S, D[d:=d1]) = F(C, S, D[d:=d2])
```

For trace-based semantics, independence MUST additionally establish:

```text
∀ d1,d2, τ1 ∈ TRACES(D[d:=d1]), τ2 ∈ TRACES(D[d:=d2]):
    EFFECTS(τ1) = EFFECTS(τ2)
```

or an equivalent stronger refinement/noninterference relation that preserves the complete authority-relevant effect set and admissible authority-relevant traces.

It is insufficient to compare only final states if intermediate effects, irreversible side effects, delegation, persistence, or authorization-relevant observations occur during execution.

This is a positive noninterference claim.

Empirical observation alone is not sufficient to establish universal independence unless the tested domain is itself the complete normative domain and the test provides exhaustive coverage of that domain.

Independence is a property of the dependency **in context**, not a permanent intrinsic property of a dependency class. A dependency proven independent for one `C`, `S`, effect universe, or domain is not thereby independent for another.

## 5. Dependency states

Every dependency considered by the specification MUST have exactly one of these states:

```text
BOUND
INDEPENDENT
UNRESOLVED
```

### BOUND

The dependency can affect authority-relevant effects and is included in the canonical execution context through an explicit identity, hash, policy, constraint, or equivalent canonical binding.

### INDEPENDENT

The dependency is outside the canonical closure because noninterference with authority-relevant effects has been positively established under the canonical effect semantics and allowed domain.

### UNRESOLVED

The system cannot establish either causal binding or valid noninterference.

```text
UNRESOLVED => NOT_AUTHORIZED
```

## 6. Canonical dependency universe

The specification MUST define the normative universe of possible execution dependencies for the relevant executor class. At minimum, candidates include:

```text
executor artifact
executor semantics
runtime
libraries/dependencies
configuration
filesystem/mount policy
network policy
credentials/identity context
kernel/host security policy
sandbox policy
hardware capability
clock/time source
randomness source
DNS/name resolution
external services
resource enforcement
dynamic plugins/extensions
execution history / trace state
temporal ordering / concurrency semantics
```

This is a dependency universe, not a claim that every item is always authority-relevant.

Each candidate MUST be classified `BOUND`, `INDEPENDENT`, or `UNRESOLVED`.

The universe itself is canonical input to authorization. A child artifact, token, executor, or runtime MUST NOT silently shrink, reinterpret, or replace the universe or its allowed domains.

## 7. Dependency record

Each canonical dependency record SHOULD contain:

```text
DependencyRecord {
    dependency_id
    dependency_class
    canonical_identity
    semantic_role
    allowed_domain
    authority_relevance
    binding_method
    binding_identity
    independence_claim
    verification_method
    status
}
```

The record MUST make it possible to determine why a dependency is in the closure, why it is proven independent, or why authorization is blocked.

For an `INDEPENDENT` dependency, `allowed_domain` MUST itself be canonical, bounded, and bound to the same authorization lineage. It MUST NOT be a mutable or executor-selected domain.

## 8. Closure completeness

For every authority-relevant effect `e`:

```text
DEPENDENCIES(e) ⊆ CANONICAL_DEPENDENCY_CLOSURE
```

For every dependency excluded from the closure:

```text
d ∉ CANONICAL_DEPENDENCY_CLOSURE
    =>
PROVEN_NONINTERFERING(d)
```

The closure is therefore complete by construction: an effect dependency is either bound or independently proven irrelevant.

There is no fourth state such as `probably irrelevant`, `ambient but trusted`, or `implementation detail`.

### 8.1 Joint noninterference

Independence MUST be established for the relevant dependency set, not merely one dependency at a time.

It is insufficient to prove:

```text
D1 independent in isolation
D2 independent in isolation
```

if their composition can alter effects:

```text
F(C,S,D1=x1,D2=y1) != F(C,S,D1=x2,D2=y2)
```

when each individual substitution, holding the other dependency fixed at its reference value, appears noninterfering.

Therefore, an `INDEPENDENT` classification is valid only when the independence claim covers the dependency's allowed joint context, including relevant interactions with other excluded dependencies.

If interaction cannot be ruled out, the dependency set is `UNRESOLVED` or the interacting dependencies MUST be moved into the canonical closure.

### 8.2 Temporal and trace closure

A dependency cannot be classified `INDEPENDENT` merely because it does not change the effect of each individual execution step.

A dependency is temporally relevant if it can change any authority-relevant property of a trace, including:

```text
step ordering
admissible next actions
intermediate observations
irreversible side effects
persistence state
credential/delegation state
resource exhaustion state
concurrency/interleaving outcomes
termination / continuation
final authority-relevant state
```

The following counterexample is explicitly invalidating:

```text
step A: individually allowed
step B: individually allowed

A ; B
    ↓
new authority-relevant effect
```

If `A` and `B` are each safe under a one-step effect model but their sequence creates an effect outside the parent authority, the model MUST treat the sequence as a trace-level effect and reject the authorization unless that trace is explicitly within the canonical effect boundary.

Likewise, a dependency classified `INDEPENDENT` MUST be noninterfering with respect to relevant traces, not merely isolated steps.

## 9. Ambient dependency rule

A dependency remains authority-relevant even if it is:

- outside the executor's explicit API;
- outside the configuration format;
- supplied by the operating environment;
- normally stable;
- inconvenient to hash;
- treated as implementation detail.

If changing it can change authority-relevant effects, it belongs to the canonical closure.

Examples include time, DNS, ambient credentials, host policy, external service responses, or dynamic loading when those factors influence effects.

## 10. Semantic normalization

Different representations of the same authority-relevant dependency MUST be normalized before relevance is evaluated.

Examples include:

```text
localhost / loopback / local socket
hostname / resolved endpoint
alias / canonical resource identity
multiple credential representations / same effective identity
```

Normalization MUST preserve authority-relevant distinctions. An alias cannot be used to hide a semantic change.

## 11. Fail-closed rule

The executor MUST NOT infer independence from absence of evidence.

```text
no causal proof
AND
no independence proof
        ↓
UNRESOLVED
        ↓
NOT_AUTHORIZED
```

Operational success, artifact existence, historical stability, or a human assertion cannot convert `UNRESOLVED` into `INDEPENDENT`.

## 12. Canonical binding requirement

Every `BOUND` dependency MUST be linked to the authorization lineage through a canonical identity appropriate to its semantics.

A hash is an integrity binding for the bytes or canonical representation it covers. It is not, by itself, proof that the dependency boundary is complete.

Therefore:

```text
binding integrity
    !=
boundary completeness
```

Both properties are required.

## 13. Domain-closure requirement

The allowed domain used by any independence claim MUST itself be canonical and closed for the relevant semantic question.

The following is insufficient:

```text
"D is independent for values currently observed"
```

The specification MUST instead define the admissible domain `Dom(D)` and establish the claim over that domain.

If the domain can be expanded by the executor, environment, token, or downstream artifact without a new canonical governance event, the independence claim is invalid.

A newly admitted domain value that can alter authority-relevant effects invalidates the previous independence claim and requires reclassification/rebinding.

## 14. Adversarial obligations

The specification MUST survive at least these counterexamples:

### CED-001 — omitted ambient dependency

A dependency outside the declared interface changes an authority-relevant effect.

Expected: dependency becomes `BOUND`; prior authorization is invalid until rebinding occurs.

### CED-002 — false independence

A dependency is labeled `INDEPENDENT`, but two allowed values produce different authority-relevant effects.

Expected: `INDEPENDENT` classification fails; dependency becomes `BOUND` or `UNRESOLVED`.

### CED-003 — incomplete universe

A real dependency is absent from the candidate dependency universe.

Expected: authorization cannot be considered valid until the universe is extended and the dependency classified.

### CED-004 — semantic alias

Two syntactically different values represent materially different effective authority.

Expected: normalization preserves the distinction; effect difference remains visible.

### CED-005 — hidden external effect

An external service, DNS result, credential, or host policy changes effective authority without changing the declared executor artifact.

Expected: dependency is treated as authority-relevant and must be bound or independently proven noninterfering.

### CED-006 — empirical false negative

Many tests observe no effect from a dependency, but the allowed domain contains an untested value that changes effects.

Expected: testing alone does not establish universal `INDEPENDENT` status.

### CED-007 — compositional independence failure

`D1` and `D2` are each classified `INDEPENDENT` under one-at-a-time substitutions, but a joint assignment changes authority-relevant effects.

Expected: individual independence claims fail for the joint context; the interacting dependency set becomes `BOUND` or `UNRESOLVED`.

### CED-008 — mutable-domain expansion

A dependency is `INDEPENDENT` over `Dom(D)`, but the executor or environment silently expands `Dom(D)` to include a value that changes effects.

Expected: expansion is rejected unless introduced by a new canonical governance event; otherwise authorization is `NOT_AUTHORIZED`.

### CED-009 — context transfer

A dependency is proven independent under `(C1,S1)` but the same classification is reused under `(C2,S2)` where it changes effects.

Expected: independence does not transfer across semantic context; the new context requires a fresh proof or binding.

### CED-010 — hidden interaction through excluded set

Multiple dependencies excluded from the closure interact through an unmodeled relation, even though each pairwise single-variable test appears independent.

Expected: the excluded dependency set is not `INDEPENDENT` until joint noninterference is established; otherwise `UNRESOLVED`.

### CED-011 — emergent sequential effect

Each action `a_i` is individually within the declared effect boundary, but a valid sequence `a1 ; a2 ; ... ; an` creates an authority-relevant effect that no individual action creates.

Expected: the effect model MUST evaluate the relevant trace; stepwise independence is insufficient. Authorization is invalid unless the emergent trace effect is explicitly accounted for.

### CED-012 — order sensitivity

The same set of actions produces different authority-relevant effects under different orderings:

```text
A ; B != B ; A
```

Expected: execution ordering is part of canonical trace semantics whenever it can affect authority-relevant effects.

### CED-013 — intermediate-state authority

The final state is identical across two traces, but one trace produces an irreversible intermediate side effect, delegation, observation, persistence change, or external action.

Expected: final-state equality does not establish noninterference; trace-level effects MUST distinguish the executions.

### CED-014 — concurrency/interleaving emergence

Each action is individually safe and each pairwise sequential trace is safe, but concurrent/interleaved execution creates an authority-relevant effect.

Expected: if concurrency is admissible, the canonical trace semantics MUST include the relevant interleavings or provide a stronger proof that all admissible interleavings are noninterfering.

## 15. Governance boundary

This specification defines a criterion and evidence model. It does not itself establish any execution authority.

```text
DESIGN STATUS: DRAFT
ADMISSION STATUS: NOT ADMITTED
TOKEN IMPLEMENTATION: NOT AUTHORIZED
ENFORCEMENT IMPLEMENTATION: NOT AUTHORIZED
EXECUTION AUTHORIZATION: NOT GRANTED
```

A future implementation MUST consume an admitted version of this specification rather than silently extending its semantic boundary.
