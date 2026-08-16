# LifeGraph P03 / N03b Metadata Observation Review 001

Status: **REVIEWED — OBSERVATION GAP CONFIRMED**

## Scope

This review examines only whether the frozen observation surface provides an implementation-neutral projection sufficient to observe **presence** of the Contract-level birth/death metadata required by C-LG/O-LG and Test Plan v1.1.

No Contract, C-LG, O-LG, Test Plan, or production-code changes are authorized or included by this review.

## Frozen sources

- Contract: `docs/spec/evolution-application-contract-v1.0.md`
- Semantic baseline: `docs/gates/lifegraph-semantic-baseline-v1.md`
- Test Plan: `docs/gates/lifegraph-test-plan-v1.1.md`
- Harness conformance review: `docs/gates/lifegraph-harness-conformance-review-v1.1.md`

## Contract-level requirement

Contract v1.0 §6 states that an accepted death records death metadata and that an accepted birth records the parent/child relation and birth metadata. §9 identifies associated birth/death metadata as part of genealogy preservation. The Contract deliberately does not define the concrete representation, field names, field types, or encoding of those metadata records.

Therefore the following implications are **not** Contract-authorized by themselves:

- `death_reason.is_some()` → death metadata record present;
- `death_cycle.is_some()` → death metadata record present;
- `birth_cycle` being present → birth metadata record present;
- any conjunction of implementation fields → Contract-level metadata record present.

## Current implementation observation surface

The harness projection exposes implementation fields including `birth_cycle`, `death_cycle`, and `death_reason` on active and historical nodes. The production `LifeNode` contains the same fields. Death commit currently populates `death_cycle` and `death_reason`; birth commit currently sets `birth_cycle` and leaves death fields empty.

These facts establish that the implementation contains metadata-like data, but they do not establish that any particular field or combination is the Contract-level metadata record. Promoting those fields into the semantic definition would add an implementation-derived predicate not stated by the frozen Contract.

## P03 — accepted death

Required semantic observation:

`accepted death + active node removed + required death metadata recorded + historical representation present`.

The active-node removal and historical representation are independently observable through the frozen projections.

The required death-metadata **presence**, however, is not independently observable without interpreting one or more implementation-specific fields as the Contract-level record.

**Verdict: P03 metadata component = UNSUPPORTED / OBSERVATION GAP.**

The current harness assertion that both `death_cycle` and `death_reason` are present is therefore not a valid implementation-neutral Oracle observation.

## N03b — birth missing birth metadata

Required counterstate:

`accepted birth + child process present + active child LifeGraph node present + parent/child relation present + required birth metadata absent`.

The child process, active node, and parent/child relation are independently observable. The Contract-level birth metadata record is not independently represented by the frozen observation surface.

`birth_cycle` is an implementation field. Treating its presence as proof of the Contract-level birth-metadata record, or its absence as proof of that record's absence, would introduce the same unsupported semantic promotion identified for N05b.

**Verdict: N03b metadata component = UNSUPPORTED / OBSERVATION GAP.**

## Independence result

The gap is symmetric across positive and negative metadata assertions:

```text
implementation field present
        ≠
Contract metadata record present

implementation field absent
        ≠
Contract metadata record absent
```

The problem is therefore not merely fixture construction. The current observation surface lacks an implementation-neutral semantic projection for the metadata record itself.

## Consequence for Test Plan v1.1

This review does **not** modify Test Plan v1.1. It establishes that the currently frozen P03 and N03b metadata assertions cannot be implemented faithfully from the existing observation surface without introducing a new semantic predicate or promoting implementation details into Contract semantics.

The appropriate next step is a separate governed Test Plan revision proposal addressing P03 and N03b. Until such a revision is approved and frozen:

- P03 remains present in v1.1 but is **blocked on metadata observability**;
- N03b remains present in v1.1 but is **blocked on metadata observability**;
- no metadata-related harness assertion is authorized;
- no Contract or C-LG/O-LG change is authorized;
- no production-code change is authorized for the purpose of satisfying these tests;
- no Gate execution may treat the current metadata assertions as valid evidence.

## Final verdict

```text
Contract authority for birth/death metadata       YES ✓
Implementation-neutral metadata projection         NO ✗
P03 metadata presence observation                   GAP ✗
N03b metadata absence observation                   GAP ✗
New semantic predicate required                     YES — if implemented as-is
Contract change                                     NO
C-LG/O-LG change                                    NO
Test Plan change                                    NOT MADE
Production code change                              NONE
Gate execution                                      BLOCKED for these metadata assertions
```

**Conclusion:** the observation gap is broader than N05b. Both positive death-metadata observation (P03) and negative birth-metadata observation (N03b) cross the same unsupported boundary. The correct response is governance of the Test Plan, not semantic invention in the harness.
