# LifeGraph Consistency — Claim / Oracle Analysis

Status: **ANALYSIS ONLY — NOT FROZEN**

This document evaluates whether the LifeGraph consistency dimension can support an independent semantic Claim and Oracle directly from Contract v1.0. It does not authorize tests, harness changes, or a new Gate.

## 1. Contract source

Contract v1.0 §6 defines the LifeGraph semantics:

- an active process has a corresponding life node;
- death removes the active life node, records death metadata, and places the resulting historical node in fossil/history;
- birth creates the child process and corresponding life node, records the parent/child relation and birth metadata, and makes the child eligible for scheduling.

Contract v1.0 §9 independently names **LifeGraph consistency** as a normative, independently testable invariant.

## 2. Candidate semantic Claim

**C-LG:** After each accepted evolution transition, the observable active-process set and LifeGraph active representation are mutually consistent according to the LifeGraph semantics stated in Contract v1.0 §6.

For a death, the formerly active process has no active life node and its historical representation is present with the required death metadata.

For a birth, the accepted child has a corresponding active life node, and the parent/child relation and birth metadata are represented as required by the contract.

This Claim is intentionally limited to the semantics explicitly stated in §6 and does not assert any implementation-specific graph representation, ordering, container type, serialization, or converse mapping beyond what the Contract explicitly requires.

## 3. Revised candidate independent Oracle

**O-LG:** Observe the post-transition semantic state through implementation-neutral projections whose contents are defined only by Contract v1.0 §6:

1. **Active-process projection** — the set of processes considered active by the application state, limited to the identities needed to evaluate the Contract's LifeGraph correspondence.
2. **Active-LifeGraph projection** — the active life-node representation needed to determine whether each active process has its required corresponding life node.
3. **Historical/Genealogical projection** — fossil/history entries and the parent/child plus birth/death metadata explicitly required by §6.

The Oracle evaluates only these Contract-supported relations:

- for each active process, the required corresponding active life node is present;
- for an accepted death, the active life node is removed, the required death metadata is recorded, and the resulting historical representation is placed in fossil/history;
- for an accepted birth, the child process and corresponding active life node are present, the required parent/child relation and birth metadata are recorded, and the child is eligible for scheduling.

The Oracle does **not** require a converse relation from every active life node to an active process, does not impose an `exactly one` cardinality requirement unless separately sourced from the Contract, and does not inspect Rust types, collection internals, helper functions, control flow, or current implementation-specific symbols.

## 4. Independence assessment

The revised Claim and Oracle are derived from Contract v1.0 §6 and §9 rather than from the current implementation. The Contract explicitly defines the semantic relations to be observed and explicitly declares LifeGraph consistency independently testable.

The previous adversarial review identified that the earlier Oracle formulation had silently strengthened the Contract by adding a converse and exact-cardinality requirement. Those additions have now been removed. The revised Oracle is limited to observations and expected relations directly stated by the Contract.

Therefore an independent semantic Claim + Oracle **can be constructed** for the LifeGraph dimension without silently extending the Contract.

## 5. Boundary

This analysis does **not** establish that the current implementation satisfies C-LG.

It does **not** establish that the proposed Oracle is complete for every possible interpretation of the LifeGraph contract.

It establishes only the narrower methodological result:

> Contract v1.0 contains enough explicit semantic material to define a candidate LifeGraph Claim and an implementation-independent Oracle without silently extending the contract.

## 6. Next permitted step

A short repeat independence review is required before freezing C-LG/O-LG. No LifeGraph test cases or harness changes are authorized by this document.
