# LifeGraph Test Plan Revision Proposal 002

Status: **PROPOSED — NOT FROZEN**

## Purpose

Resolve the Governance Review 002 finding that LG-N05b has Contract authority but lacks an implementation-neutral observation surface capable of independently constructing its required counterstate.

This proposal does **not** modify Contract v1.0, C-LG, O-LG, or production code.

## Source basis

- Contract: `docs/spec/evolution-application-contract-v1.0.md` — **FROZEN**
- Semantic baseline: `docs/gates/lifegraph-semantic-baseline-v1.md` — **FROZEN**
- Test Plan: `docs/gates/lifegraph-test-plan-v1.md` — **FROZEN**
- Governance Review 002: `docs/gates/lifegraph-n05b-observation-gap-002-governance-review.md`

## Finding

LG-N05b requires an accepted-death state with:

- active LifeGraph node removed;
- historical representation present;
- required Contract-level death-metadata record absent.

The current observation surface exposes implementation-level metadata components, including `death_reason` and `death_cycle`, but Contract v1.0 does not define either field, their combination, or any `None`/sentinel encoding as the absence of the Contract-level death-metadata record.

Therefore the current LG-N05b fixture cannot construct its named semantic counterstate without introducing a new semantic predicate. Such a predicate would violate the frozen implementation-neutral observation boundary.

## Proposed disposition

**Remove LG-N05b from the Test Plan matrix. Do not replace it with an unrelated negative case.**

Rationale:

1. The negative case is semantically valid at the Contract level.
2. It is not independently observable through the currently frozen observation surface.
3. Re-encoding the case through `death_reason`, `death_cycle`, or a sentinel would add unsupported semantics.
4. Adding a different negative case merely to preserve matrix cardinality would weaken traceability and change the purpose of the LifeGraph negative suite.
5. The positive accepted-death case LG-P03 continues to test the Contract's required death metadata as part of a successful transition.
6. LG-N04 independently tests active-node removal.
7. LG-N05a independently tests historical representation.

Together, the remaining cases retain independently observable coverage of the other death obligations without pretending that the missing Contract-level metadata record has a supported negative observation.

## Exact proposed matrix change

Current:

- LG-N05a — Death missing historical representation
- LG-N05b — Death missing death metadata

Proposed:

- LG-N05a — Death missing historical representation
- **LG-N05b — removed from matrix**

No renumbering of unrelated cases is proposed.

## Explicit non-actions

This proposal does **not**:

- change Contract v1.0;
- change C-LG;
- change O-LG;
- define `death_reason` or `death_cycle` as Contract semantics;
- define `None` as semantic metadata absence;
- add a new Oracle predicate;
- modify production behavior;
- authorize harness execution;
- claim that the Contract requirement for death metadata is unnecessary.

## Consequence

After approval and freezing of this revision, the LifeGraph test matrix will no longer claim an independently executable negative test for absence of the Contract-level death-metadata record.

This is a **testability limitation**, not a Contract waiver and not evidence that the implementation satisfies the omitted negative state.

## Approval boundary

This document is a proposal only. It does not alter the frozen Test Plan. A separate governed approval must accept or reject this disposition before any frozen Test Plan update is made.
