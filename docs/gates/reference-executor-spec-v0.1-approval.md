# Reference Executor Specification v0.1 — Approval Record

**Status:** APPROVED / AUTHORIZED

## Approved artifact

`docs/gates/reference-executor-spec-v0.1.md`

**Approved revision:** `64a77553d9f259a902ffe1bc82b575c820afb7de`

## Normative dependency

`Validation Machine State Model v0.1`

**Approved State Model revision:** `a3cdeaf009e2e1afff136b6883cb33840a742b1f`

**State Model approval record:** `c51d850185554adc676200ca34d04a31bcc56f10`

## Approval basis

The specification was reviewed after revision to separate terminal machine states from operational execution outcomes.

The approved specification establishes that:

- the Validation Machine State Model is normative authority;
- the executor implements authorized transitions and does not extend the State Model;
- unauthorized transitions are blocked before execution and resolve through `FORBIDDEN_TRANSITION -> NOT_AUTHORIZED -> STOP`;
- unavailable required observations resolve through `OBSERVATION_GAP -> STOP`;
- `PASS` and `FAIL` require admissible evidence and authorized evaluation;
- `UNTESTED` is never inferred from execution failure or missing evidence;
- frozen governance artifacts are read-only during execution;
- machine state and verdict classification remain distinct;
- operational failure without an authorized semantic mapping produces `EXECUTION_ABORTED` rather than an implicit machine `STOP` or semantic verdict;
- `EXECUTION_ABORTED` is an operational outcome, not a State Model state and not a verdict class;
- deterministic inputs must produce deterministic transition and verdict semantics;
- executor implementation must remain agent- and provider-neutral.

## Authorization

This approval authorizes implementation of the **Reference Executor** according to the approved specification.

It does **not** authorize:

- modification of Contract semantics;
- modification of C-LG/O-LG;
- modification of Test Plan v1.1;
- modification or removal of existing harness cases;
- introduction of new semantic predicates;
- extension of the State Model;
- cloud infrastructure implementation;
- agent/provider protocol changes.

Executor conformance tests are a separate implementation/conformance step and must verify the approved specification; they do not become a source of new semantics.

## Boundary

```text
Validation Machine State Model v0.1    APPROVED / AUTHORIZED
Reference Executor Spec v0.1           APPROVED / AUTHORIZED
Reference Executor implementation       AUTHORIZED
Executor conformance tests              NEXT GOVERNED STEP
Contract                               UNCHANGED
C-LG / O-LG                            UNCHANGED
Test Plan v1.1                         UNCHANGED
Harness                                UNCHANGED
Cloud execution layer                  NOT YET AUTHORIZED
```

Approval is recorded separately from the specification, preserving the distinction between review approval and artifact authorization.
