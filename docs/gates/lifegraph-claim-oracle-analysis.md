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

This Claim is intentionally limited to the semantics explicitly stated in §6 and does not assert any implementation-specific graph representation, ordering, container type, or serialization.

## 3. Candidate independent Oracle

**O-LG:** Observe the post-transition semantic state through three implementation-neutral projections:

1. **Active-process projection** — the set of processes considered active by the application state.
2. **Active-LifeGraph projection** — the set of active life nodes and their identity correspondence to active processes.
3. **Historical/Genealogical projection** — fossil/history entries and parent/child plus birth/death metadata required by §6.

The Oracle evaluates only the relations explicitly required by Contract v1.0:

- every active process has exactly one corresponding active life node;
- every active life node corresponds to an active process;
- accepted death removes the active correspondence and preserves the required historical representation and death metadata;
- accepted birth creates the child process and corresponding active life node and preserves the required parent/child and birth metadata.

The Oracle does not inspect Rust types, collection internals, helper functions, control flow, or current implementation-specific symbols.

## 4. Independence assessment

The Claim and Oracle are derivable from Contract v1.0 itself rather than from the current implementation. The contract explicitly defines the semantic relations to be observed and explicitly declares LifeGraph consistency independently testable.

Therefore an independent semantic Claim + Oracle **can be constructed** for the LifeGraph dimension without adding a new normative requirement.

However, independence is not yet sufficient for Gate authorization. The exact observation mechanism and expected outcomes must still be specified without importing assumptions from the current implementation, and the Claim/Oracle pair must undergo adversarial review before freezing.

## 5. Boundary

This analysis does **not** establish that the current implementation satisfies C-LG.

It does **not** establish that the proposed Oracle is complete for every possible interpretation of the LifeGraph contract.

It establishes only the narrower methodological result:

> Contract v1.0 contains enough explicit semantic material to define a candidate LifeGraph Claim and an implementation-independent Oracle without silently extending the contract.

## 6. Next permitted step

Review the candidate C-LG / O-LG pair adversarially. If it survives, freeze the pair as a baseline before designing any positive or negative cases.

No LifeGraph test cases or harness changes are authorized by this document.
