# LifeGraph Semantic Baseline v1

Status: **FROZEN**

Freeze basis:
- Contract: `docs/spec/evolution-application-contract-v1.0.md`
- Claim/Oracle analysis: `docs/gates/lifegraph-claim-oracle-analysis.md`
- Adversarial review: `docs/gates/lifegraph-claim-oracle-adversarial-review.md`
- Independence review: `docs/gates/lifegraph-claim-oracle-independence-review.md`

## Frozen Claim — C-LG

After each accepted evolution transition, the observable active-process set and LifeGraph active representation are mutually consistent according to the LifeGraph semantics stated in Contract v1.0 §6.

For a death, the formerly active process has no active life node and its historical representation is present with the required death metadata.

For a birth, the accepted child has a corresponding active life node, and the parent/child relation and birth metadata are represented as required by the Contract.

The Claim is limited to semantics explicitly stated in §6. It does not assert implementation-specific graph representation, ordering, container type, serialization, or any converse mapping not explicitly required by the Contract.

## Frozen Oracle — O-LG

Observe the post-transition semantic state through implementation-neutral projections whose contents are defined only by Contract v1.0 §6:

1. **Active-process projection** — active process identities needed to evaluate the Contract's LifeGraph correspondence.
2. **Active-LifeGraph projection** — active life-node representation needed to determine whether each active process has its required corresponding life node.
3. **Historical/Genealogical projection** — fossil/history entries and the parent/child plus birth/death metadata explicitly required by §6.

The Oracle evaluates only these Contract-supported relations:

- for each active process, the required corresponding active life node is present;
- for an accepted death, the active life node is removed, the required death metadata is recorded, and the resulting historical representation is placed in fossil/history;
- for an accepted birth, the child process and corresponding active life node are present, the required parent/child relation and birth metadata are recorded, and the child is eligible for scheduling.

The Oracle does **not** require a converse relation from every active life node to an active process and does **not** assert `exactly one` cardinality unless separately sourced from the Contract.

The Oracle does not inspect Rust types, collection internals, helper functions, control flow, serialization format, or current implementation-specific symbols.

## Independence / Freeze Rationale

The prior adversarial review identified and removed an unsupported converse and exact-cardinality strengthening. The subsequent independence review found the revised Claim and Oracle contract-grounded, implementation-independent, and limited to semantic observations directly stated by §6.

Therefore:

> **C-LG/O-LG = FROZEN**

## Authorization Boundary

This baseline freezes only the semantic Claim and Oracle. It does **not** authorize test cases, harness changes, implementation changes, or a G12 execution. Any such work requires a separate test-plan review derived from this frozen baseline.
