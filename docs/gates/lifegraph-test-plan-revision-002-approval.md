# LifeGraph Test Plan Revision 002 — Approval Record

Status: **APPROVED / FROZEN**

## Decision

Test Plan Revision Proposal 002 is approved.

Disposition:

> **Remove LG-N05b from the LifeGraph Test Plan matrix. Do not replace it with an unrelated negative case.**

## Basis

Governance Review 002 established that LG-N05b has Contract authority but lacks an implementation-neutral observation surface capable of independently constructing its required Contract-level counterstate.

The approved disposition therefore treats N05b as a testability limitation rather than:

- a Contract defect;
- a Contract waiver;
- an implementation failure;
- a reason to introduce a new semantic predicate.

## Scope of approved change

Allowed:

- create and freeze `docs/gates/lifegraph-test-plan-v1.1.md`;
- remove LG-N05b from the active Test Plan matrix;
- retain N05b's observation-gap record and proposal as provenance.

Not allowed by this approval:

- modify Contract v1.0;
- modify C-LG;
- modify O-LG;
- define `death_reason`, `death_cycle`, `None`, or a sentinel as Contract semantics;
- modify production semantics;
- claim that the death-metadata requirement is satisfied merely because N05b is removed;
- execute the harness or issue a G12 verdict without a separate execution/evidence record.

## Resulting state

```text
Contract v1.0                 FROZEN ✓
C-LG / O-LG                   FROZEN ✓
Test Plan v1                  FROZEN historical baseline ✓
Test Plan v1.1                FROZEN active revision ✓
LG-N05b                       REMOVED FROM ACTIVE MATRIX ✓
N05b observation gap          PRESERVED AS GOVERNANCE RECORD ✓
Production code               UNCHANGED ✓
New semantic predicate        NONE ✓
```

## Traceability

- `docs/gates/lifegraph-n05b-observation-gap-002-governance-review.md`
- `docs/gates/lifegraph-test-plan-revision-proposal-002.md`
- `docs/gates/lifegraph-test-plan-v1.md`
- `docs/gates/lifegraph-test-plan-v1.1.md`
- `docs/gates/lifegraph-semantic-baseline-v1.md`
- `docs/spec/evolution-application-contract-v1.0.md`
