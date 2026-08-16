# G11 — Test Plan

Status: **DRAFT — TEST DESIGN ONLY**

## Purpose

Define the positive, negative, and boundary cases for G11 strictly from the frozen semantic baseline. This document does not change the Claim or Oracle and does not authorize implementation changes or test execution.

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
| O1-A | Capability safety | An ineligible/unauthorized target is admitted as an authorized evolution operation |
| O1-B | Structural atomicity | A structurally invalid plan causes any evolution-state mutation |
| O1-C | Build-then-commit atomicity | A failed child build causes committed evolution mutation, including death, child, generation, or LifeGraph mutation |
| O1-D | Temporal tolerance | A stale request becomes a plan-wide failure or invalidates an unrelated valid request |

## Positive cases

### TC-P01 — Authorized evolution request

**Exercises:** O1-A

**Purpose:** Verify that an eligible target can pass the contract's capability gate when all other required conditions are satisfied.

**Expected observable outcome:** The request is not rejected solely because the target lacks the required capability.

**Evidence:** machine-readable application result plus post-state sufficient to establish the capability outcome.

### TC-P02 — Structurally valid plan with successful build

**Exercises:** O1-B, O1-C

**Purpose:** Establish the normal admissible path needed as the control case for rejection/failed-build cases.

**Expected observable outcome:** No O1-B or O1-C failure condition is observed.

**Scope note:** This case does not assert exact genealogy, generation, queue, fossil, or audit semantics beyond the frozen contract requirements.

### TC-P03 — Mixed application containing stale and valid requests

**Exercises:** O1-D

**Purpose:** Verify individual temporal tolerance without invalidating an unrelated valid request.

**Expected observable outcome:** The stale request is skipped/ignored according to the contract and the unrelated valid request is not invalidated solely because of the stale request.

## Negative cases

### TC-N01 — Ineligible target presented as evolution-capable

**Exercises:** O1-A

**Expected observable outcome:** The target is not admitted as an authorized evolution operation. Any admission is an O1-A failure.

### TC-N02 — Structurally invalid plan

**Exercises:** O1-B

**Expected observable outcome:** Evolution state before and after application is observationally identical with respect to the contract's evolution state. Any partial death, birth, genealogy, generation, or equivalent mutation is an O1-B failure.

### TC-N03 — Child build failure after otherwise admissible validation

**Exercises:** O1-C

**Expected observable outcome:** No committed evolution mutation: specifically no deaths, no children, no generation bump, and no LifeGraph mutation. Failure/audit recording is allowed only as specified by the contract.

### TC-N04 — Stale request without unrelated valid work

**Exercises:** O1-D

**Expected observable outcome:** The stale request is individually tolerated/skipped rather than being treated as a plan-wide invalidation.

## Boundary / adversarial cases

### TC-B01 — Invalid plan with multiple mutation opportunities

A structurally invalid plan must not partially apply an earlier valid-looking operation before discovering the invalid condition.

**Expected:** O1-B remains satisfied: zero evolution-state mutation.

### TC-B02 — Build failure after earlier death/birth opportunities

Arrange the request so that a naive implementation could commit earlier effects before child construction fails.

**Expected:** O1-C remains satisfied: no committed evolution mutation.

### TC-B03 — Stale request adjacent to valid request

Place the stale request immediately before and immediately after an unrelated valid request in separate executions/fixtures.

**Expected:** O1-D remains satisfied in either ordering; the stale request does not invalidate unrelated valid work.

### TC-B04 — Capability failure combined with otherwise valid structure

Make the plan structurally valid but target an ineligible entity.

**Expected:** O1-A rejects admission; structural validity must not bypass capability safety.

## Evidence requirements

Each executed case must produce machine-readable evidence containing at least:

- test case ID;
- frozen Contract v1.0 reference;
- frozen G11.1 reference;
- oracle predicate exercised;
- pre-state observation relevant to that predicate;
- request/fixture identifier;
- post-state observation relevant to that predicate;
- outcome (`PASS` / `FAIL`);
- failure classification if `FAIL`.

Evidence must be generated by the harness and retained as an immutable artifact. Human narrative is supplementary and cannot replace raw machine evidence.

## Failure rule

A single falsified frozen oracle predicate is sufficient for **G11 = FAIL**.

A harness failure, missing evidence, or inability to evaluate a predicate is **NOT PASS** and must be classified separately from a semantic failure. It must not be silently converted into a passing result.

## Explicit prohibitions

Before this plan is reviewed and accepted:

- no G11 production changes;
- no G11 test implementation;
- no G11 execution;
- no modification of C1/O1;
- no addition of semantic requirements derived from current implementation behavior.

## Review gate

This test plan itself must be reviewed against frozen Contract v1.0 and frozen G11.1 before a test harness is implemented.
