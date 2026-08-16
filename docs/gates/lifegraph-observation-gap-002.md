# LifeGraph Observation Gap 002 — N05b Death Metadata

Status: **OPEN — IMPLEMENTATION BLOCKED FOR LG-N05b**

## Discovery point

Discovered during the N05b Contract source audit and subsequent observation-surface audit at commit `fbd903a3ec8e85af2fdd7a8c050b3fa1d385921c`.

## Frozen requirement involved

`LG-N05b — Death missing death metadata` requires an accepted-death outcome in which:

- the formerly active LifeGraph node is removed;
- the required historical representation is present; and
- the required Contract-level death-metadata record is absent.

The requirement is traceable to Contract v1.0 §6 and remains Contract-authorized.

## Source finding

Contract v1.0 requires that death metadata be recorded, but does not define a field-level decomposition of that metadata and does not define an absence encoding for a component.

In particular, Contract does not make `death_reason`, `death_cycle`, `None`, a sentinel, or any other implementation convention individually normative.

Therefore `death_reason = None` cannot by itself be interpreted as the Contract-level counterstate "required death metadata record absent".

## Current observation surface

The current semantic projection exposes historical nodes containing:

- `death_cycle: Option<u64>`;
- `death_reason: Option<DeathReason>`.

The current N05b fixture mutates only `death_reason` to `None` and relies on the generic failure `dead process ... has no death metadata`.

This establishes an implementation-level field mutation, but does not establish that the mutation corresponds to absence of the whole Contract-level death-metadata record.

## Verdict

**OBSERVATION GAP.**

The frozen semantic requirement is valid. The current observation surface does not independently expose the absence of the whole Contract-level death-metadata record without introducing an additional semantic convention.

This is not evidence of a Contract violation and is not permission to change Contract, C-LG/O-LG, or production semantics.

## Governance boundary

The following remain frozen:

- Contract v1.0;
- C-LG/O-LG;
- LifeGraph Test Plan v1.

No sentinel, inferred field-level absence rule, or production-code modification is authorized by this record.

The existing N05b fixture is not authorized as a Gate-valid implementation of the frozen negative case.

## Next decision

A separate governance review must determine whether an already-supported semantic observation can expose the composite absence without extending the Contract. If not, the test-plan treatment of N05b must be revised through the normal review/re-freeze process.

Until that decision, N05b remains blocked and no complete LifeGraph Gate verdict may claim N05b coverage.
