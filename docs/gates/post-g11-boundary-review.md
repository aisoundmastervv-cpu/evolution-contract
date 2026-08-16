# Post-G11 Boundary Review

Status: **REGISTERED — NO NEW GATE AUTHORIZED YET**

## Purpose

G11 established a machine-backed PASS for the frozen semantic predicates O1-A through O1-D. This review records what remains unproven by Contract v1.0 and prevents the next Gate from being selected merely by momentum or by the capabilities of the existing harness.

## Candidate boundary 1 — Compositionality

**Question:** Does the semantic correspondence established by G11 remain valid across a sequence of individually admissible evolution operations?

Potential property:

> If each operation satisfies the frozen Contract individually, do the Contract-required invariants remain satisfied after every step of a valid sequence?

**Current status:** NOT AUTHORIZED.

Reason: Contract v1.0 must first be audited for an explicit source of a compositional requirement. If the Contract does not define such a property, treating compositionality as a G12 requirement would silently extend the specification.

## Candidate boundary 2 — Oracle adequacy

**Question:** Is the frozen Oracle sufficiently discriminating to distinguish the semantic states and behaviors that Contract v1.0 requires us to distinguish?

This is a meta-validation question. G11 demonstrated implementation → Oracle correspondence. It did not independently establish Oracle → Contract adequacy.

**Current status:** CANDIDATE FOR NEXT BOUNDARY REVIEW.

The key risk is circularity: a stronger harness can produce stronger evidence for whatever the Oracle can observe while still failing to detect a semantic distinction omitted by the Oracle itself.

## Methodological constraint

No G12 test, harness, implementation change, or new semantic predicate may be introduced from this review alone.

Before authorizing a next Gate:

1. Audit Contract v1.0 for the source of the candidate property.
2. Define the candidate Claim independently of the implementation.
3. Define an independent Oracle.
4. Test Oracle adequacy against the Contract rather than against the current implementation.
5. Freeze the result before designing positive/negative cases.

## Provisional recommendation

Prioritize **Oracle adequacy** over compositionality for the next boundary review because it tests the proof mechanism itself. Compositionality should remain a separate candidate until its contractual source is established.

## Non-Gate status

This document does **not** declare G12, G12.1, or any other future Gate. It records only the post-G11 boundary question and the decision discipline required before the next Gate can be authorized.
