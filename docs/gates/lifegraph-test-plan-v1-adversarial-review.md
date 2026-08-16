# LifeGraph Test Plan v1 — Adversarial Review

Status: **REVIEWED — REVISIONS REQUIRED**

## Scope

Review the eight proposed LifeGraph cases against frozen C-LG/O-LG for:

1. direct traceability;
2. falsifiable expected outcome;
3. absence of hidden implementation assumptions;
4. absence of semantic expansion;
5. independence between cases.

No harness or production implementation is authorized by this review.

## Case-by-case findings

| Case | Traceability | Falsifiability | Implementation independence | Verdict |
|---|---|---|---|---|
| LG-P01 | PASS | PASS | PASS | ACCEPT |
| LG-P02 | PASS | PASS | PASS | ACCEPT WITH WORDING CLARIFICATION |
| LG-P03 | PASS | PASS | PASS | ACCEPT WITH WORDING CLARIFICATION |
| LG-N01 | PASS | PASS | PASS | ACCEPT |
| LG-N02 | PASS | PASS | PASS | ACCEPT |
| LG-N03 | PASS | PASS | PASS | ACCEPT |
| LG-N04 | PASS | PASS | PASS | ACCEPT |
| LG-N05 | PASS | PASS | PASS | ACCEPT |

## Findings requiring correction

### 1. LG-P01

The case is directly grounded in the active-process correspondence requirement. It is falsifiable: removing the required active LifeGraph node must cause Oracle failure. No implementation-specific representation is required.

**Verdict: ACCEPT.**

### 2. LG-P02

The birth semantics are directly grounded in C-LG/O-LG. However, the phrase “accepted child” should be tied explicitly to an accepted birth transition rather than to an implementation-specific notion of acceptance. The test harness must obtain the semantic outcome from the transition contract, not infer it from a Rust return type.

**Required action:** wording clarification only; no semantic change.

### 3. LG-P03

The death semantics are directly grounded in C-LG/O-LG. As with P02, “accepted death transition” must refer to the semantic transition outcome, not a particular implementation return value.

**Required action:** wording clarification only; no semantic change.

### 4. LG-N01

Direct falsification of active-process → required active-LifeGraph correspondence. The negative state must be an observable semantic state, not merely a corrupted internal data structure.

**Verdict: ACCEPT.**

### 5. LG-N02

Direct falsification of the birth requirement that the child has a corresponding active LifeGraph node. No converse relation is introduced.

**Verdict: ACCEPT.**

### 6. LG-N03

Directly targets parent/child relation and birth metadata. The case must be parameterized so that exactly one required semantic relation/metadata item is absent at a time; otherwise a failure could be ambiguous.

**Required action:** test construction clarification only; no semantic change.

### 7. LG-N04

Direct falsification of the death requirement that the formerly active process has no active LifeGraph node. It must not assert anything beyond the frozen death semantics.

**Verdict: ACCEPT.**

### 8. LG-N05

Directly falsifies historical representation or required death metadata. To preserve falsifiability, missing historical representation and missing metadata should be represented as distinct subcases or independently attributable failure observations rather than one opaque combined corruption.

**Required action:** test construction clarification only; no semantic change.

## Cross-matrix findings

### Independence

The cases can be executed from isolated fixtures. The plan already requires no hidden mutable state between cases. This is sufficient as a governance constraint, but the future harness must enforce fresh semantic fixtures rather than rely on test ordering.

**PASS, with harness requirement.**

### Oracle boundary

The matrix does not require the rejected converse mapping or `exactly one` cardinality. It does not introduce graph ordering, container types, serialization, or internal control-flow assertions.

**PASS.**

### Falsifiability

Every case has a concrete semantic condition whose violation can produce an Oracle rejection. P02/P03/N03/N05 need the wording/construction clarifications above to avoid ambiguous implementation-derived failures.

**PASS after narrow clarifications.**

## Verdict

The eight-case matrix is **not yet accepted/frozen**.

It is substantively sound, but four narrow test-plan clarifications are required:

1. P02: define “accepted birth” semantically, not by implementation return type.
2. P03: define “accepted death” semantically, not by implementation return type.
3. N03: isolate missing parent/child relation vs missing birth metadata for attributable failure evidence.
4. N05: isolate missing historical representation vs missing death metadata for attributable failure evidence.

These are test-plan wording/construction changes only. **C-LG and O-LG remain frozen and must not be changed.**

After these corrections, a second short review may authorize freezing the test plan. Until then:

- harness implementation: NOT AUTHORIZED;
- production changes: NOT AUTHORIZED;
- workflow execution: NOT AUTHORIZED;
- G12 verdict: NOT AUTHORIZED.
