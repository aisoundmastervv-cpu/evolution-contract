# Post-G11 Semantic Dimensions Review

Status: **ANALYSIS ONLY — NO G12 AUTHORIZATION**

## Purpose

Determine which of the seven normative Contract v1.0 invariants are already distinguished by the frozen G11 Oracle O1, and which require a materially different oracle. This review does not change Contract v1.0, G11.1, O1, the test plan, or the implementation.

## Source boundary

Normative source: `docs/spec/evolution-application-contract-v1.0.md`, frozen at commit `118dd043d6d5d208bc197cda7583c1ea05f0bf47`.

G11 Oracle source: `docs/G11.1-semantic-baseline.md`.

The current implementation is not used to define semantic dimensions.

## Semantic dimensions

| Contract invariant | Semantic dimension | What must be distinguished | O1 status | Assessment |
|---|---|---|---|---|
| Capability safety | **Authorization / protection boundary** | Whether an evolution capability can be legitimately obtained for a protected identity | O1-A | **Directly distinguished** |
| Structural atomicity | **Pre-commit rejection atomicity** | Whether structural invalidity causes zero observable evolution-state mutation | O1-B | **Directly distinguished** |
| Build-then-commit atomicity | **Transactional build/commit boundary** | Whether build failure leaves committed evolution state unchanged | O1-C | **Directly distinguished** |
| Temporal tolerance | **Request-level stale independence** | Whether one stale plan request can be skipped without invalidating unrelated valid requests | O1-D | **Directly distinguished, but narrowly** |
| Lazy invalidation safety | **Scheduler-reference tolerance** | Whether a stale scheduler/queue reference can be skipped without becoming a fatal evolution error | O1-D | **Not distinguished** |
| LifeGraph consistency | **Cross-representation state consistency** | Whether active process state and LifeGraph representation remain mutually consistent across accepted transitions | O1-B/O1-C | **Not distinguished** |
| Genealogy preservation | **Historical/provenance relation preservation** | Whether accepted births preserve parent/child relations and associated birth/death metadata | O1-C | **Not distinguished** |

## Detailed findings

### 1. Capability safety — O1-A

The semantic dimension is authorization across a protection boundary. O1-A directly asks whether an `Immune` or `Protected` process can be admitted through the normal capability boundary. This matches the contract's invariant without relying on a Rust-specific representation.

**Conclusion:** O1-A is an adequate oracle for this selected dimension.

### 2. Structural atomicity — O1-B

The semantic dimension is atomic rejection of a structurally invalid plan. O1-B observes evolution state immediately before and after application and requires zero mutation. This directly distinguishes the contract's required boundary.

**Conclusion:** O1-B is an adequate oracle for this selected dimension.

### 3. Build-then-commit atomicity — O1-C

The semantic dimension is transactional separation between pending construction and committed evolution. O1-C observes pre/post state for a build failure and explicitly requires no deaths, children, generation advance, or LifeGraph mutation.

**Conclusion:** O1-C is an adequate failure-boundary oracle for the selected build-failure property. It does not by itself establish correctness of successful LifeGraph or genealogy transitions.

### 4. Temporal tolerance — O1-D

The semantic dimension is independence between a stale request and unrelated valid requests in the same application. O1-D directly distinguishes plan-level stale request tolerance.

The contract also contains a separate lazy-invalidation rule for stale scheduler references. O1-D does not observe that scheduler dimension.

**Conclusion:** O1-D is adequate for request-level temporal tolerance, but must not be generalized to scheduler-level lazy invalidation safety.

### 5. Lazy invalidation safety — requires a new oracle dimension

The contract explicitly says a scheduler may encounter a reference to a process that has already disappeared and skip it without treating the reference as fatal. This is semantically different from a stale request inside an evolution plan.

O1-D observes plan request handling; it does not observe scheduler traversal, stale queue references, or scheduler error classification.

**Conclusion:** not distinguishable by current O1. A future oracle must expose scheduler-reference handling as an independent observable dimension.

### 6. LifeGraph consistency — requires a new oracle dimension

The contract requires a bidirectional semantic relationship between active processes and LifeGraph representation across accepted transitions: active process ↔ active life node, death ↔ removal plus fossil/history record, birth ↔ child node plus parent/child relation and birth metadata.

O1-B and O1-C can detect unwanted LifeGraph mutation on rejected/failed applications because those predicates require no partial mutation. They do not distinguish whether a successful accepted transition produces the correct graph representation.

**Conclusion:** current O1 provides only negative protection against partial LifeGraph mutation, not a positive consistency oracle. A new oracle must compare accepted post-state relationships, not merely absence of mutation on failure.

### 7. Genealogy preservation — requires a new oracle dimension

The contract requires accepted births to preserve parent/child relationships and associated birth/death metadata. This is a relational and historical property, not merely a mutation-count property.

O1-C can detect absence of genealogy mutation on build failure, but cannot distinguish correct from incorrect genealogy on successful commit.

**Conclusion:** current O1 does not distinguish genealogy preservation. A future oracle must inspect provenance relations and required metadata across an accepted transition.

## Coverage classes

### Class A — already distinguished by O1

- Capability safety
- Structural atomicity
- Build-then-commit atomicity (failure boundary)
- Temporal tolerance (plan-request scope)

### Class B — partially touched by O1 but not semantically distinguished

- LifeGraph consistency — O1-C observes the absence of LifeGraph mutation on failure, but not correctness after success.
- Genealogy preservation — O1-C observes the absence of genealogy-affecting mutation on failure, but not preservation after success.

### Class C — requires a distinct observation surface

- Lazy invalidation safety — requires scheduler/queue observation.
- LifeGraph consistency — requires accepted-transition graph consistency observation.
- Genealogy preservation — requires accepted-transition provenance/history observation.

## Important non-inference

The fact that O1 can observe `LifeGraph` or genealogy-related state as part of a failure snapshot does not make O1 a LifeGraph or genealogy oracle. Observability of a state component is not equivalent to semantic adequacy for the invariant governing that component.

Likewise, the fact that O1-D covers stale plan requests does not establish lazy invalidation safety for stale scheduler references.

## Boundary result

The seven invariants collapse into at least **five semantically distinct dimensions** for oracle-design purposes:

1. Authorization/protection boundary.
2. Pre-commit rejection atomicity.
3. Build/commit transactional atomicity.
4. Temporal/request independence.
5. Cross-state/relational consistency and scheduler/history dimensions, which further require distinct observation surfaces for scheduler tolerance, LifeGraph consistency, and genealogy preservation.

For practical Gate design, the last group must be split rather than treated as one generic "state correctness" oracle, because the contract specifies different observables and failure meanings.

## Governance conclusion

**No G12 is authorized by this document.**

This analysis identifies where new oracle capability is needed; it does not formulate new claims, expected outcomes, test cases, or implementation requirements.

The next specification-level step, if desired, is to choose one uncovered semantic dimension and perform a source audit for a minimal new Claim and independent Oracle. No test harness work is permitted before that claim/oracle pair is frozen.
