# Post-G11 Boundary Review — Oracle Adequacy Source Audit

Status: **AUDITED — NO G12 AUTHORIZATION**

## Question

Can the frozen G11 Oracle O1 be treated as an adequate semantic discriminator for the full normative scope of Evolution Application Contract v1.0?

## Sources audited

- `docs/spec/evolution-application-contract-v1.0.md` — Contract v1.0, FROZEN.
- `docs/G11.1-semantic-baseline.md` — G11.1, FROZEN.

## Finding

**No — not for the full Contract v1.0 semantic surface.**

This is not a defect in G11. It is an explicit scope boundary of G11.1.

Contract v1.0 §9 defines seven independently testable normative properties:

1. Capability safety
2. Structural atomicity
3. Build-then-commit atomicity
4. Temporal tolerance
5. Lazy invalidation safety
6. LifeGraph consistency
7. Genealogy preservation

G11.1 C1 and O1 intentionally select only the first four properties. The frozen G11.1 baseline explicitly states that LifeGraph consistency, genealogy preservation, runnable-queue semantics, fossil/death metadata semantics, audit-state semantics, and exact generation semantics are outside the current Gate.

Therefore O1 is adequate only for the frozen **C1 claim surface**. It is not evidence that O1 is a complete discriminator for the full Contract v1.0 semantic surface.

## Consequence

The post-G11 Oracle adequacy candidate is therefore a legitimate new research boundary, but it cannot be promoted directly to G12 without first defining exactly what "adequate" means independently of the current implementation.

In particular, the following distinction must remain explicit:

- **G11 adequacy:** O1 is sufficient to evaluate C1. — established by the frozen G11.1 design and execution.
- **Full-contract Oracle adequacy:** O1 distinguishes all semantically significant behaviors required by Contract v1.0. — **not established**.

## Required next specification work

Before any G12 tests or harness changes are authorized:

1. Define a minimal Claim for Oracle adequacy that is sourced from Contract v1.0 and does not silently expand it.
2. Define an independent Oracle for that Claim.
3. Establish an adequacy criterion that is not derived from the current implementation or existing G11 tests.
4. Identify the contract predicates currently outside O1 and determine whether they are semantically independent dimensions or merely refinements of existing O1 predicates.
5. Freeze the new Claim/Oracle only after adversarial review.

## Explicit non-decision

This audit does **not**:

- declare G12;
- declare an Oracle-adequacy PASS or FAIL;
- modify Contract v1.0;
- modify C1 or O1;
- authorize new tests;
- authorize harness implementation.

> **Current conclusion: Oracle adequacy beyond the G11 C1 surface is UNDETERMINED.**
