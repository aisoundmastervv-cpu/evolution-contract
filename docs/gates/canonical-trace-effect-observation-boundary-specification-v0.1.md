# Canonical Trace/Effect Observation Boundary Specification v0.1

**Status:** DRAFT — PROPOSAL FOR ADVERSARIAL REVIEW

**Purpose:** define the minimum normative observation boundary required to test authority-effect identifiability. This document does **not** authorize Admission, execution, mutation, or any new semantic verdict.

**Precondition:** the existing canonical artifacts were reviewed for derivability. They establish trace/effect semantics and evidence/trace persistence, but do not uniquely derive a complete canonical observation projection. This specification is therefore a proposed missing normative layer, not an inferred fact.

## 1. Governing question

The boundary exists only to answer:

> Can the canonical evidence uniquely determine the complete authority-relevant effect set of an execution?

Let:

```text
U   = declared execution universe
τ   = complete execution trace
Eτ  = complete authority-relevant effect set
B   = canonical Trace/Effect Observation Boundary
O_B = canonical observation projection induced by B
```

The target property is:

```text
∀ τ1, τ2 ∈ U:
    O_B(τ1) = O_B(τ2)
        ⇒
    Eτ1 = Eτ2
```

This property is called **Authority-Effect Identifiability**.

Failure to establish it MUST prevent a negative authority claim such as `INDEPENDENT`.

## 2. Non-goals

This specification MUST NOT:

- define Admission;
- authorize execution transitions;
- redefine Capability;
- redefine Executor semantics;
- redefine the Effect Dependency Specification;
- claim that an effect is absent merely because it was not observed;
- treat operational telemetry as semantic evidence;
- require observation of every physical event, only every distinction relevant to authority-relevant effect sets.

## 3. Observation boundary

The canonical observation boundary is the normative projection from execution reality into admissible canonical evidence:

```text
Execution Reality
      ↓
Complete Trace τ
      ↓
Boundary B
      ↓
O_B(τ)
      ↓
Canonical Evidence
```

`O_B` MUST preserve every distinction that can change `Eτ` within the declared execution universe.

It MAY abstract distinctions that cannot change `Eτ`.

Therefore the requirement is not trace identity. It is authority-effect equivalence preservation.

## 4. Declared execution universe

The declared execution universe MUST state the classes of execution that the claim intends to cover, including where applicable:

```text
state transitions
sequential ordering
concurrency/interleavings
external actions
delegation / authority transfer
persistent writes
queued or deferred work
retries / repeated attempts
abort and failure paths
termination paths
post-attempt continuations that remain within scope
```

An execution class outside the declared universe MUST NOT silently be treated as covered.

A claim whose scope is broader than `U` is not established by this specification.

## 5. Observation completeness

For a fixed execution universe `U`, the boundary is **authority-effect complete** iff:

```text
O_B(τ1) = O_B(τ2)
⇒
Eτ1 = Eτ2
```

for every `τ1, τ2 ∈ U`.

Equivalently, any two executions that differ in an authority-relevant effect MUST be distinguishable by canonical observation.

The boundary need not expose irrelevant implementation details.

## 6. Authority-relevant effect coverage

The observation boundary MUST account for effect channels represented by the canonical Effect Dependency Specification, including at minimum:

```text
state mutation
persistent side effects
external actions
delegation / authority transfer
resource-affecting actions
intermediate-state authority effects
order-dependent effects
concurrent/interleaving effects
emergent sequential effects
```

The list is a coverage obligation, not a closed enumeration of possible effects. Any newly established authority-relevant effect class MUST be reflected in the boundary or make identifiability fail closed until coverage is restored.

## 7. Execution horizon

Observation completeness requires a defined execution horizon.

A trace MUST NOT be considered complete merely because:

```text
an observed process returned;
an immediate state was sampled;
a worker exited;
a local operation ended;
no further effect was observed yet.
```

The governing execution semantics MUST establish one of:

```text
COMPLETED
ABORTED WITH CLOSED EFFECT HORIZON
INCOMPLETE / UNKNOWN HORIZON
```

`INCOMPLETE / UNKNOWN HORIZON` MUST NOT support `INDEPENDENT`.

Deferred, asynchronous, queued, retried, or externally triggered continuations remain in scope when the declared execution universe includes them.

## 8. External effects

The boundary MUST preserve authority-relevant effects occurring outside the immediate local process when those effects are within `U`.

Examples include:

```text
filesystem / durable storage
network or service actions
credential or capability delegation
external actor requests
persistent records
cross-process effects
```

Operational telemetry MAY evidence that an execution occurred, but MUST NOT by itself be treated as proof that an external semantic effect did or did not occur.

## 9. Concurrency and ordering

The boundary MUST retain all ordering/interleaving information necessary to distinguish authority-relevant effects.

In particular:

```text
A ; B
```

MUST NOT be observationally collapsed with:

```text
B ; A
```

when their authority-relevant effect sets differ.

Likewise, concurrent executions MUST remain distinguishable whenever different interleavings can produce different authority-relevant effect sets.

## 10. Intermediate-state effects

Final-state equality is insufficient evidence of effect equality.

The boundary MUST preserve authority-relevant effects that occur in intermediate states, including effects that are irreversible even when later state converges.

Therefore:

```text
final_state(τ1) = final_state(τ2)
```

MUST NOT imply:

```text
Eτ1 = Eτ2
```

unless the boundary's completeness proof establishes that implication for the declared universe.

## 11. Observation failure semantics

If any authority-relevant portion of the declared execution cannot be observed or its observation completeness cannot be established, the result MUST be:

```text
UNDETERMINED
```

and MUST NOT be:

```text
INDEPENDENT
```

This rule applies to:

```text
missing trace segments
unknown execution horizon
unresolved external actions
unresolved deferred work
unresolved concurrency/interleaving
unresolved persistence
unresolved delegation
```

The semantic distinction is mandatory:

```text
not observed ≠ absent
incomplete ≠ empty
unknown ≠ independent
```

## 12. Observational equivalence

Define:

```text
τ1 ≈_B τ2
iff
O_B(τ1) = O_B(τ2)
```

Authority-effect identifiability requires:

```text
τ1 ≈_B τ2
⇒
Eτ1 = Eτ2
```

Thus `≈_B` is permitted to collapse only executions that are authority-effect equivalent.

## 13. Adversarial falsification tests

The draft MUST be reviewed against at least these constructions:

### A. Hidden-effect witness

Attempt to construct:

```text
O_B(τ1) = O_B(τ2)
Eτ1 ≠ Eτ2
```

If successful, the boundary is insufficient.

### B. Deferred-effect witness

Attempt to append a deferred continuation to an otherwise identical observed prefix.

The boundary MUST either observe the continuation or classify the observation as incomplete.

### C. Concurrency witness

Construct different interleavings with equal final state but different authority-relevant effects.

The boundary MUST distinguish them or fail closed.

### D. Intermediate irreversible effect

Construct two executions with equal final state where only one performs an irreversible authority-relevant action.

The boundary MUST distinguish them.

### E. Partial-observation witness

Remove an authority-relevant trace segment.

Expected result:

```text
UNDETERMINED
```

not `INDEPENDENT`.

## 14. Proof obligations

Approval of this specification MUST NOT be granted merely because the listed adversarial examples fail to produce a counterexample.

The review MUST distinguish:

```text
no counterexample found
```

from:

```text
identifiability established
```

The latter requires either:

1. a proof that `O_B` preserves all authority-relevant distinctions over `U`; or
2. an equivalent formal argument that observational equivalence implies authority-effect equivalence.

## 15. Scope of negative claims

A negative authority claim is permitted only when all of the following hold:

```text
execution ∈ U
AND
trace/effect observation is complete
AND
execution horizon is closed
AND
all declared authority-relevant effect channels are covered
AND
observational equivalence preserves effect-set equality
```

Otherwise:

```text
INDEPENDENT = NOT AUTHORIZED
```

and the appropriate epistemic result is `UNDETERMINED` unless another positive/negative verdict is independently justified.

## 16. Governance boundary

This artifact is a proposal only.

Until separately reviewed and approved:

```text
Observation Boundary       DRAFT
Identifiability             UNRESOLVED
INDEPENDENT                 NOT PROVABLE
ADMISSION                   BLOCKED
EXECUTION TRANSITION        BLOCKED
```

Approval of this document MUST NOT itself grant Admission authority.

## 17. Required review outcome

The adversarial reviewer MUST return exactly one primary disposition:

```text
IDENTIFIABILITY SUPPORTED
```

only if the completeness property is actually established;

```text
BOUNDARY COUNTEREXAMPLE
```

if an observationally indistinguishable pair with different authority-relevant effects is demonstrated; or

```text
UNDETERMINED
```

if the boundary or its completeness remains insufficiently established.

No fourth path through assumption or practical confidence is permitted.
