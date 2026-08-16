# LifeGraph Harness Conformance Review v1.1

Status: **REVIEW COMPLETE — NO CODE CHANGES AUTHORIZED**

## Purpose

Determine whether the current LifeGraph harness conforms to frozen Test Plan v1.1 without modifying production code, C-LG/O-LG, or the frozen Test Plan.

## Frozen basis

- Contract v1.0 — **FROZEN**
- LifeGraph Semantic Baseline v1 — **FROZEN**
- LifeGraph Test Plan v1.1 — **FROZEN**
- N05b Observation Gap 002 Governance Review — **RESOLVED**

## Review rule

A harness case is conformant only if:

1. it corresponds directly to a v1.1 matrix case;
2. its expected observation is supported by the frozen semantic surface;
3. it does not derive Contract semantics from implementation-specific fields/types;
4. it does not introduce a new semantic predicate;
5. it does not execute or mutate production behavior merely to make the oracle pass.

No test execution is performed by this review.

## Case-by-case findings

### LG-P01 — Stable active correspondence

**Status: SUPPORTED.**

The harness observes active processes and active LifeGraph nodes and checks the required active-process → active-LifeGraph correspondence. This matches the frozen C-LG/O-LG relation and does not depend on implementation-specific metadata.

### LG-P02 — Accepted birth

**Status: PARTIALLY SUPPORTED / FIXTURE SCOPE GAP.**

The harness checks child presence, active LifeGraph correspondence, parent/child relation, and scheduling eligibility. However, Test Plan v1.1 also requires required birth metadata to be present. The current evaluator does not independently assert a Contract-level birth-metadata record.

This is not a justification to infer a specific implementation field as the Contract definition. A follow-up review is required before claiming full P02 conformance.

### LG-P03 — Accepted death

**Status: NOT CONFORMANT.**

The harness checks active-node removal and historical representation correctly at the semantic level. It then treats `fossil.death_cycle.is_some()` and `fossil.death_reason.is_some()` as proof of the Contract-level required death metadata.

Contract v1.0 does not define either implementation field, their conjunction, or their `Option` encoding as the Contract-level death-metadata record. The N05b governance review explicitly rejected this interpretation for absence; the same implementation-neutrality boundary applies to presence.

Therefore the current P03 oracle cannot claim Contract-level metadata presence from these fields.

### LG-N01 — Missing active LifeGraph correspondence

**Status: SUPPORTED.**

The harness constructs an observable state with an active process lacking its required active LifeGraph node and the oracle rejects the state using the frozen correspondence relation.

### LG-N02 — Birth missing child node

**Status: SUPPORTED.**

The harness removes the child's active LifeGraph node after an accepted-birth setup and the oracle rejects the missing correspondence.

### LG-N03a — Birth missing parent/child relation

**Status: SUPPORTED.**

The harness removes the parent's child relation and the oracle rejects the missing required relation.

### LG-N03b — Birth missing birth metadata

**Status: NOT IMPLEMENTED.**

Test Plan v1.1 retains LG-N03b, but the current harness contains no corresponding test. No unsupported implementation-specific metadata predicate is introduced, so this is an implementation coverage gap rather than a semantic reinterpretation.

### LG-N04 — Death leaves active LifeGraph node

**Status: FIXTURE/ORACLE MISMATCH.**

The Test Plan requires an accepted-death outcome in which the former active LifeGraph node remains active. The current fixture removes the process from `scheduler.processes` but does not construct that accepted-death outcome; it then relies on `evaluate_death()` to fail because the node remains active.

The failure is caused by the implementation state being inconsistent, but the fixture does not explicitly establish an accepted-death semantic outcome while preserving the active node. This requires a separate fixture-design review before execution can be claimed conformant.

### LG-N05a — Death missing historical representation

**Status: PARTIALLY SUPPORTED / METADATA DEPENDENCY.**

The harness performs an accepted death, clears fossil history, and evaluates death. The expected failure is the missing historical representation, but `evaluate_death()` also requires implementation-level `death_cycle` and `death_reason` fields for success. Because the fossils are cleared first, the historical lookup fails before those checks, so the current N05a negative path remains semantically aligned with the named missing relation.

No code change is authorized by this review.

### LG-N05b — Death missing death metadata

**Status: REMOVED / STALE HARNESS CASE.**

Test Plan v1.1 explicitly removes LG-N05b. The current harness still contains the old `lg_n05b_death_missing_death_metadata_is_rejected` test and therefore has drifted from the frozen matrix.

This test must not be treated as an authorized v1.1 test. Its removal is a future implementation action only after the present review is accepted.

## Summary matrix

| Case | Conformance |
|---|---|
| LG-P01 | SUPPORTED |
| LG-P02 | PARTIALLY SUPPORTED / FIXTURE SCOPE GAP |
| LG-P03 | NOT CONFORMANT |
| LG-N01 | SUPPORTED |
| LG-N02 | SUPPORTED |
| LG-N03a | SUPPORTED |
| LG-N03b | NOT IMPLEMENTED |
| LG-N04 | FIXTURE/ORACLE MISMATCH |
| LG-N05a | PARTIALLY SUPPORTED / METADATA DEPENDENCY |
| LG-N05b | REMOVED / STALE HARNESS CASE |

## Critical finding

The N05b investigation exposed a symmetric issue: if `death_reason`/`death_cycle` cannot be used as the Contract definition of metadata absence, they also cannot be used as the Contract definition of metadata presence in P03.

Therefore P03 currently has Contract authority for the required semantic outcome, but its implementation-neutral observation is insufficient to independently verify the metadata component.

This does **not** imply a Contract defect and does **not** authorize a Contract change. It establishes a new observation/conformance boundary for the harness.

## Governance consequences

1. Contract v1.0 remains **FROZEN**.
2. C-LG/O-LG remain **FROZEN**.
3. Test Plan v1.1 remains **FROZEN**.
4. No production code is changed by this review.
5. No new semantic predicate is introduced.
6. No G12 execution verdict is authorized from the current harness.
7. The stale N05b test must not be counted as v1.1 coverage.
8. P03 requires a separate governed observation review before it can be considered fully conformant.
9. N03b requires implementation of a test only if an implementation-neutral birth-metadata observation is available; otherwise it requires its own observation-gap review rather than an invented field-level predicate.

## Verdict

> **Current harness is NOT CONFORMANT to Test Plan v1.1.**

The primary blocking issue is not the removed N05b. It is the broader observation-boundary mismatch revealed by P03: the harness currently promotes implementation-level metadata fields into Contract-level semantics.

**No code changes are authorized by this review.**
