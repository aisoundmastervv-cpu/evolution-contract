# LifeGraph Consistency — Adversarial Review

Status: **REVIEWED — NOT FROZEN**

## Scope

This review attacks the candidate C-LG / O-LG pair in `lifegraph-claim-oracle-analysis.md` against Contract v1.0. No tests, harness changes, or new normative requirements are authorized.

## Finding 1 — C-LG is substantively grounded, but its transition scope must be explicit

Contract v1.0 §6 states that the successful transition applies accepted deaths and accepted births together as one evolution transition and then defines the LifeGraph semantics. §9 names LifeGraph consistency as an independently testable invariant.

Therefore the core proposition — consistency after an accepted successful transition — is grounded in the Contract.

**Disposition:** PASS, with wording constraint: do not broaden the Claim to arbitrary internal states or failed/rejected attempts unless the Contract explicitly requires a LifeGraph condition there.

## Finding 2 — O-LG currently overstates bidirectionality

The candidate Oracle says:

- every active process has exactly one corresponding active life node;
- every active life node corresponds to an active process.

The Contract explicitly states that an active process has a corresponding life node, but does not separately state the converse as an independent normative rule, nor does it use the word `exactly`.

The converse may be a natural interpretation of "corresponding", but treating it as an independently testable biconditional would add semantic force not explicitly stated.

**Disposition:** FAIL AS WRITTEN. Narrow O-LG to the relation directly stated by §6: every active process has its required corresponding active life node. If the analysis needs a converse, it must first be sourced explicitly from the Contract rather than inferred.

## Finding 3 — Historical/genealogical observations are contract-grounded for death and birth

For death, §6 explicitly requires removal of the active life node, death metadata, and placement of the resulting historical node in fossil/history. For birth, §6 explicitly requires the child process, corresponding life node, parent/child relation, birth metadata, and scheduling eligibility.

**Disposition:** PASS. These are legitimate Oracle observations, provided the Oracle checks only these semantic requirements and does not infer extra graph properties.

## Finding 4 — "three projections" is an observation design, not a normative requirement

Active-process, active-LifeGraph, and historical/genealogical projections are implementation-neutral only if their contents are defined semantically. The names themselves must not become hidden requirements about data structures or serialization.

**Disposition:** PASS WITH CONSTRAINT. The projections may be used as observation interfaces, but their expected contents must be derived field-by-field from §6.

## Finding 5 — "accepted evolution transition" must not imply successful-only coverage of all LifeGraph behavior

The Contract's §6 semantics are attached to commit/successful transition. Build failure explicitly says no LifeGraph mutation occurs, but that is already a separate build-then-commit invariant. Therefore C-LG should not silently absorb the negative atomicity property into LifeGraph consistency.

**Disposition:** PASS WITH BOUNDARY. Keep C-LG about the post-success correspondence and birth/death semantics; leave failure atomicity to O1-C unless a later Contract-derived Claim explicitly separates it.

## Finding 6 — Oracle independence is achievable

The proposed Oracle can be expressed as semantic observations of process identity, active LifeGraph correspondence, and required historical metadata/relations. It does not require Rust types, containers, helper functions, control flow, or implementation symbols.

**Disposition:** PASS.

## Verdict

The candidate pair **does not survive unchanged**.

The core Claim is contract-grounded, but O-LG must be narrowed because its current converse/exactly-one formulation is stronger than the text explicitly establishes.

### Required correction before freezing

1. Remove the unsupported converse requirement: `every active life node corresponds to an active process`.
2. Remove the unsupported strength of `exactly one` unless an explicit Contract source is found.
3. Keep the Contract-derived death and birth observations.
4. Define semantic projections only as observation mechanisms, not additional normative requirements.

**No G12 authorization. No test-case authorization. No harness changes.**

Next permitted step: revise only O-LG to the narrowest Contract-supported form, then repeat a short independence review before freezing C-LG/O-LG.
