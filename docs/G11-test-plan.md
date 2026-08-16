# G11 — Test Plan

Status: **ACCEPTED — HARNESS MAY BE IMPLEMENTED**

## Purpose

Define the positive, negative, and boundary cases for G11 strictly from the frozen semantic baseline. This document does not change the Claim or Oracle.

## Frozen inputs

- Normative contract: `docs/spec/evolution-application-contract-v1.0.md`
- Contract status: **FROZEN**
- G11 semantic baseline: `docs/G11.1-semantic-baseline.md`
- G11.1 status: **FROZEN**

## Test-design immutability rule

C1 and O1 are frozen. No test result, implementation behavior, fixture limitation, or harness convenience may modify their meaning or expected outcomes.

Tests may operationalize the frozen predicates. They may not add new semantic requirements.

## Oracle predicates

| ID | Frozen predicate | Required observable failure |
|---|---|---|
| O1-A | Capability safety | An `Immune` or `Protected` process is admitted as an authorized evolution target through the normal capability boundary |
| O1-B | Structural atomicity | A structurally invalid plan causes any evolution-state mutation |
| O1-C | Build-then-commit atomicity | A failed child build causes committed evolution mutation, including deaths, children, generation advance, or LifeGraph mutation |
| O1-D | Temporal tolerance | A stale request becomes a plan-wide failure or invalidates an unrelated valid request |

## Traceability matrix

Every semantic test case maps to an already-frozen oracle predicate and an already-frozen Contract v1.0 rule. The matrix is closed: adding a case requires a documented reason that it is needed to operationalize an existing predicate; it may not introduce a new semantic requirement.

| Test | Role | Oracle | Contract v1.0 source | Frozen expected observation |
|---|---|---|---|---|
| TC-P01 | Control | O1-A | §1 Protection and capability safety | Eligible target can be represented at the capability boundary; no O1-A failure |
| TC-P02 | Control | O1-C | §5 Build phase | Valid fixture reaches build-success path; no O1-C failure |
| TC-P03 | Positive | O1-D | §4 Temporal tolerance | Stale request is individually tolerated and unrelated valid request remains eligible |
| TC-N01 | Negative | O1-A | §1 Protection and capability safety | `Immune`/`Protected` target is not admitted |
| TC-N02 | Negative | O1-B | §3 Structural validation | Structurally invalid plan produces zero evolution-state mutation |
| TC-N03 | Negative | O1-C | §5 Build phase | Build failure produces no committed deaths, children, generation advance, or LifeGraph mutation |
| TC-N04 | Negative | O1-D | §4 Temporal tolerance | Stale request does not cause plan-wide failure or invalidate unrelated valid request |
| TC-B01 | Boundary | O1-B | §3 Structural validation | Invalid plan remains zero-mutation despite earlier mutation opportunities |
| TC-B02 | Boundary | O1-C | §5 Build phase | Build failure remains zero committed evolution despite earlier mutation opportunities |
| TC-B03 | Boundary | O1-D | §4 Temporal tolerance | Stale request does not invalidate unrelated valid work in either ordering |
| TC-B04 | Boundary | O1-A | §1 Protection and capability safety | Structural validity cannot bypass capability safety |

## Positive / control cases

These cases are controls and operational coverage. They do **not** create a new semantic success oracle beyond O1.

### TC-P01 — Eligible target capability control

**Exercises:** O1-A control

**Purpose:** Confirm the harness can construct an otherwise eligible evolution target and observe the capability boundary without confusing eligibility with unauthorized admission.

**Expected observable outcome:** The eligible target is representable as an evolution-capable request target. No O1-A failure is present.

**Evidence:** capability/result observation sufficient to show the fixture is suitable for O1-A testing.

### TC-P02 — Valid-plan build control

**Exercises:** O1-C control

**Purpose:** Establish a valid plan/build fixture that can be reused to construct the corresponding forced-build-failure case.

**Expected observable outcome:** The fixture reaches the build-success path without an O1-C failure condition.

**Scope note:** This is a harness control, not an assertion of exact successful transition semantics.

### TC-P03 — Mixed stale + valid control

**Exercises:** O1-D

**Purpose:** Establish the application shape required to observe individual stale tolerance and preservation of an unrelated valid request.

**Expected observable outcome:** The stale request is individually skipped/tolerated and the unrelated valid request remains eligible, exactly as required by O1-D.

## Negative cases

### TC-N01 — Ineligible target presented as evolution-capable

**Exercises:** O1-A

**Expected observable outcome:** An `Immune` or `Protected` process is not admitted as an authorized evolution target through the normal capability boundary. Any such admission is an O1-A failure.

### TC-N02 — Structurally invalid plan

**Exercises:** O1-B

**Expected observable outcome:** Evolution state before and after application is observationally identical with respect to the contract's evolution state. Any partial evolution-state mutation is an O1-B failure.

### TC-N03 — Child build failure after otherwise admissible validation

**Exercises:** O1-C

**Expected observable outcome:** No committed evolution mutation: specifically no deaths, no children, no generation advance, and no LifeGraph mutation. Failure/audit recording is allowed only as specified by Contract v1.0.

### TC-N04 — Stale request paired with unrelated valid request

**Exercises:** O1-D

**Expected observable outcome:** The stale request is individually skipped/tolerated, and the unrelated valid request remains eligible for application. Treating the stale request as a plan-wide failure or invalidating the unrelated valid request is an O1-D failure.

## Boundary / adversarial cases

### TC-B01 — Invalid plan with multiple mutation opportunities

A structurally invalid plan must not partially apply an earlier valid-looking operation before discovering the invalid condition.

**Expected:** O1-B remains satisfied: zero evolution-state mutation.

### TC-B02 — Build failure after earlier death/birth opportunities

Arrange the request so that a naive implementation could commit earlier effects before child construction fails.

**Expected:** O1-C remains satisfied: no committed evolution mutation.

### TC-B03 — Stale request ordering

Run the same stale + unrelated-valid fixture with the stale request before and after the unrelated valid request.

**Expected:** O1-D remains satisfied in either ordering; the stale request does not invalidate unrelated valid work.

### TC-B04 — Capability failure combined with otherwise valid structure

Make the plan structurally valid but target an `Immune` or `Protected` entity.

**Expected:** O1-A rejects admission; structural validity must not bypass capability safety.

## Evidence requirements

Each executed semantic case must produce machine-readable evidence containing at least:

- test case ID;
- frozen Contract v1.0 reference;
- frozen G11.1 reference;
- oracle predicate exercised;
- pre-state observation relevant to that predicate;
- request/fixture identifier;
- post-state observation relevant to that predicate;
- outcome (`PASS` / `FAIL`);
- failure classification if `FAIL`.

For O1-D, evidence must explicitly identify both the stale request and the unrelated valid request and show that the latter remains eligible.

Evidence must be generated by the harness and retained as an immutable artifact. Human narrative is supplementary and cannot replace raw machine evidence.

## Failure rule

A single falsified frozen oracle predicate is sufficient for **G11 = FAIL**.

A harness failure, missing evidence, or inability to evaluate a predicate is **NOT PASS** and must be classified separately from a semantic failure. It must not be silently converted into a passing result.

## Explicit prohibitions

- No G11 production changes are authorized by this document.
- No G11 execution is authorized by this document.
- C1/O1 remain immutable.
- No semantic requirements may be added from current implementation behavior.

## Review gate

**Review result: ACCEPTED.**

The matrix has been reviewed against frozen Contract v1.0 and frozen G11.1. Every semantic case maps to an existing oracle predicate and contract rule; no case adds a new semantic requirement.

**Authorization:** implementation of the G11 test harness may now begin. Harness implementation must remain mechanically derived from this accepted matrix and may not alter C1/O1.
