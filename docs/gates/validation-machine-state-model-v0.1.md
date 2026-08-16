# Validation Machine State Model v0.1

**Status:** DRAFT — PENDING REVIEW / APPROVAL  
**Scope:** validation governance only  
**Contract:** unchanged  
**C-LG / O-LG:** unchanged  
**Test Plan v1.1:** unchanged  
**Harness:** unchanged  
**Production code:** unchanged  
**Cloud execution layer:** out of scope

## 1. Purpose

This document defines the minimal state-transition model for the Validation Machine. It formalizes execution discipline between frozen governance artifacts, observation, evidence, oracle evaluation, and verdicts.

It does **not** define or extend Contract semantics. It does **not** promote implementation fields into Contract concepts. It does **not** authorize changes to frozen artifacts.

The central property is:

> The Validation Machine must be able to stop at an epistemic boundary without converting insufficient evidence into a semantic claim.

## 2. State set

### Normal execution states

- `FROZEN_INPUT` — frozen governance inputs have been identified and are read-only for execution.
- `PLAN_AUTHORIZED` — the applicable Test Plan scope is authorized for execution.
- `OBSERVATION_REQUIRED` — a test claim has identified an observation that must be obtained.
- `OBSERVATION_EXECUTED` — the authorized observation procedure has executed and produced raw observation output.
- `EVIDENCE_COLLECTED` — admissible evidence has been assembled from observations without adding semantic meaning not supplied by the governing artifacts.
- `ORACLE_EVALUATED` — the evidence has been evaluated against the authorized claim/oracle rules.
- `VERDICT` — a permitted verdict has been established.

### Epistemic / governance states

- `OBSERVATION_UNAVAILABLE` — a required observation cannot be obtained from the authorized observation surface.
- `OBSERVATION_GAP` — the inability to establish a required observation has been formally classified.
- `UNDERDETERMINED` — available evidence does not uniquely support a permitted semantic conclusion.
- `FORBIDDEN_TRANSITION` — an attempted transition or action is not authorized by this model or by an approved refinement.
- `NOT_AUTHORIZED` — the rejected action/transition has reached the machine's authorization boundary.
- `STOP` — terminal execution state.

`UNTESTED` is a verdict class, not a machine state. It may be established only where the governing Test Plan or another approved governance artifact explicitly classifies the applicable claim/scope as untested; it is not inferred from an execution failure or observation gap.

## 3. Core transitions

```text
FROZEN_INPUT
    -> PLAN_AUTHORIZED

PLAN_AUTHORIZED
    -> OBSERVATION_REQUIRED
    -> VERDICT [only when the governing artifacts explicitly classify the applicable scope as UNTESTED]

OBSERVATION_REQUIRED
    -> OBSERVATION_EXECUTED
    -> OBSERVATION_UNAVAILABLE

OBSERVATION_EXECUTED
    -> EVIDENCE_COLLECTED

EVIDENCE_COLLECTED
    -> ORACLE_EVALUATED
    -> UNDERDETERMINED

ORACLE_EVALUATED
    -> VERDICT
    -> UNDERDETERMINED

OBSERVATION_UNAVAILABLE
    -> OBSERVATION_GAP

OBSERVATION_GAP
    -> STOP

UNDERDETERMINED
    -> STOP

FORBIDDEN_TRANSITION
    -> NOT_AUTHORIZED

NOT_AUTHORIZED
    -> STOP

VERDICT
    -> STOP
```

A transition not explicitly authorized by this model or by a later approved refinement is forbidden. An attempted forbidden transition is represented by `FORBIDDEN_TRANSITION` and must not execute its requested action.

## 4. Transition contract

Every implemented transition must have, at minimum:

1. **Preconditions** — what must already be true.
2. **Input** — the artifacts or observations consumed.
3. **Allowed action** — the transformation the executor may perform.
4. **Produced evidence** — what new evidence is created or preserved.
5. **Next state** — the only state(s) to which the transition may advance.
6. **Forbidden alternatives** — semantic shortcuts that the transition must not perform.

No executor, agent, or cloud component may invent a transition by implication.

## 5. Verdict space

The machine recognizes only explicitly authorized verdict classes. The minimal v0.1 space is:

- `PASS`
- `FAIL`
- `UNTESTED`
- `OBSERVATION_GAP`
- `UNDERDETERMINED`
- `NOT_AUTHORIZED`

`OBSERVATION_GAP`, `UNDERDETERMINED`, and `NOT_AUTHORIZED` are not substitutes for `PASS` or `FAIL`.

`UNTESTED` is a verdict-only classification. It requires explicit authorization by the governing artifacts and is never inferred merely because execution did not occur or because evidence is missing.

In particular:

```text
missing observation != PASS
missing observation != FAIL
untested != PASS
untested != FAIL
```

## 6. Epistemic boundary invariant

If a required observation cannot be independently produced from the authorized observation surface:

```text
required observation unavailable
        -> OBSERVATION_GAP
        -> STOP
```

The machine MUST NOT:

- infer missing semantics;
- introduce a new semantic predicate solely to complete the test;
- promote an implementation-level field into Contract-level meaning;
- treat `None`, `Some`, absence, or presence of an implementation field as Contract semantics unless that correspondence is independently authorized by the governing artifacts;
- convert the observation gap into `PASS` or `FAIL`.

## 7. Frozen-artifact invariant

Execution cannot silently modify:

- Contract semantics;
- C-LG;
- O-LG;
- an approved Test Plan baseline;
- evidence admissibility rules;
- verdict semantics.

A requested mutation of a frozen artifact is a forbidden transition and must resolve to:

```text
FORBIDDEN_TRANSITION
    -> NOT_AUTHORIZED
    -> STOP
```

The rejected action itself is not executed.

## 8. Agent neutrality

The agent/executor is not the semantic authority.

```text
Agent may:
    inspect
    execute authorized actions
    collect observations
    assemble evidence
    propose or perform authorized transitions

Agent may not:
    redefine Contract semantics
    redefine Test Plan semantics
    redefine observation semantics
    redefine evidence admissibility
    redefine verdict semantics
    bypass an epistemic boundary
```

A more capable agent does not acquire additional epistemic authority merely by being more capable.

## 9. Reference safety properties

The following properties are normative for v0.1:

**I1 — Frozen inputs are read-only for execution.**  
Execution cannot silently mutate frozen governance artifacts.

**I2 — Observation does not imply interpretation.**  
Raw implementation observations do not acquire Contract semantics by proximity.

**I3 — Missing evidence cannot produce a semantic verdict.**  
A required but unavailable observation leads to `OBSERVATION_GAP`, not `PASS` or `FAIL`.

**I4 — Agent output cannot alter semantic authority.**  
Agent reasoning is subordinate to the State Model and governing artifacts.

**I5 — Every PASS/FAIL requires admissible evidence.**  
A verdict is not justified merely because execution completed.

**I6 — Epistemic boundaries are terminal.**  
When the machine cannot establish the required observation or semantic conclusion, it may terminate without forcing a PASS/FAIL result.

**I7 — Authorization boundaries are terminal.**  
`NOT_AUTHORIZED` cannot silently transition into execution.

## 10. Explicit non-goals

This v0.1 does not:

- modify the Evolution Application Contract;
- modify C-LG or O-LG;
- modify Test Plan v1.1;
- modify or remove any harness case;
- define new Contract metadata fields;
- define new semantic predicates for LifeGraph metadata;
- implement an executor;
- define cloud infrastructure;
- define an agent protocol or provider API.

## 11. Current status

This document is a **proposal for review and approval**. It is not yet an authorized implementation specification.

Until separately approved:

```text
State Model v0.1      PENDING REVIEW / APPROVAL
Harness                UNCHANGED
Production code        UNCHANGED
Contract               UNCHANGED
C-LG / O-LG            UNCHANGED
Test Plan v1.1         UNCHANGED
Cloud layer            NOT AUTHORIZED
```

Approval of this document, if granted, should be recorded separately from the document itself.
