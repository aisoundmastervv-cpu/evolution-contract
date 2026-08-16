# Reference Executor Specification v0.1

**Status:** DRAFT — PENDING REVIEW / APPROVAL  
**Normative dependency:** Validation Machine State Model v0.1  
**Approved State Model:** `a3cdeaf009e2e1afff136b6883cb33840a742b1f`  
**Approval record:** `c51d850185554adc676200ca34d04a31bcc56f10`  
**Contract:** unchanged  
**Test Plan v1.1:** unchanged  
**Harness:** unchanged  
**Production code:** unchanged  
**Cloud:** out of scope

## 1. Purpose

This document specifies the minimal reference executor that implements the approved Validation Machine State Model. It is an implementation specification, not a source of Contract semantics or validation methodology.

The executor MUST implement the approved State Model and MUST NOT extend it by implication.

> **Machine defines the rules; executor performs the rules.**

## 2. Normative authority

The authority order is:

```text
Contract / approved Test Plan
        ↓
Validation Machine State Model v0.1
        ↓
Reference Executor Specification v0.1
        ↓
Reference Executor implementation
```

If executor behavior conflicts with the State Model, the executor is non-conformant. The conflict MUST NOT be resolved by silently changing the State Model.

## 3. Input contract

The reference executor consumes only explicitly supplied inputs:

- frozen governance artifacts required by the authorized validation scope;
- current machine state;
- an authorized transition request;
- raw observation output, where applicable;
- admissible evidence produced by authorized observation procedures;
- oracle/claim rules already authorized by governing artifacts.

The executor MUST NOT infer additional semantic inputs from implementation details.

## 4. Machine-state representation

The executor MUST represent the State Model states explicitly, including:

```text
FROZEN_INPUT
PLAN_AUTHORIZED
OBSERVATION_REQUIRED
OBSERVATION_EXECUTED
EVIDENCE_COLLECTED
ORACLE_EVALUATED
VERDICT
OBSERVATION_UNAVAILABLE
OBSERVATION_GAP
UNDERDETERMINED
FORBIDDEN_TRANSITION
NOT_AUTHORIZED
STOP
```

`UNTESTED` is a verdict classification, not a machine state.

The executor MUST distinguish machine state from verdict classification.

## 5. Transition dispatcher

For a current state `S` and requested transition `T`, the dispatcher MUST first determine whether `T` is explicitly authorized by the State Model or by a later approved refinement.

```text
if T is authorized from S:
    execute T
else:
    do not execute T
    S -> FORBIDDEN_TRANSITION
       -> NOT_AUTHORIZED
       -> STOP
```

No implementation-specific fallback transition is permitted.

## 6. Global unauthorized-transition guard

The guard is mandatory and MUST execute before the requested action.

```text
ANY STATE S
    + unauthorized transition request
    -> FORBIDDEN_TRANSITION
    -> NOT_AUTHORIZED
    -> STOP
```

The requested unauthorized action MUST NOT execute as a side effect of detecting the violation.

`FORBIDDEN_TRANSITION` is a machine state. The guard itself is a transition rule, not an additional state.

## 7. Epistemic-boundary handling

When a required observation cannot be independently produced from the authorized observation surface:

```text
OBSERVATION_REQUIRED
    -> OBSERVATION_UNAVAILABLE
    -> OBSERVATION_GAP
    -> STOP
```

The executor MUST NOT:

- synthesize the missing observation;
- introduce a semantic predicate solely to complete the claim;
- promote implementation fields into Contract concepts;
- reinterpret absence/presence of an implementation value as Contract semantics without explicit governing authorization;
- emit `PASS` or `FAIL` for the missing observation.

This is an execution rule implementing the approved epistemic boundary; it does not define what the Contract means.

## 8. Evidence handling

The executor MUST preserve raw observations separately from derived evidence.

For every transition that produces evidence, the executor SHOULD retain, at minimum:

```text
source
observation identity
input identity
transition identity
machine state before
machine state after
evidence payload or reference
timestamp / execution identity
```

The executor MUST NOT alter raw observation semantics while packaging evidence.

Evidence admissibility remains governed by the approved governance artifacts. The executor cannot enlarge admissibility rules.

## 9. Oracle boundary

The executor may invoke an authorized oracle/evaluation rule, but it MUST NOT rewrite the oracle's semantic criteria.

The executor may route an oracle result to:

```text
ORACLE_EVALUATED
    -> VERDICT
```

or, where the State Model permits:

```text
ORACLE_EVALUATED
    -> UNDERDETERMINED
    -> STOP
```

The executor MUST NOT manufacture a positive or negative semantic result from an oracle failure, missing evidence, or an implementation-specific convention.

## 10. Verdict emission

The executor recognizes only the State Model verdict space:

```text
PASS
FAIL
UNTESTED
OBSERVATION_GAP
UNDERDETERMINED
NOT_AUTHORIZED
```

A `PASS` or `FAIL` MUST be backed by admissible evidence and an authorized oracle/claim evaluation.

`UNTESTED` may be emitted only when explicitly authorized by governing artifacts. It MUST NOT be inferred from execution failure or observation absence.

After a verdict is established:

```text
VERDICT -> STOP
```

## 11. Frozen-artifact protection

The reference executor MUST treat the following as read-only during validation execution:

- Contract semantics;
- C-LG;
- O-LG;
- approved Test Plan baseline;
- evidence admissibility rules;
- verdict semantics.

A requested mutation of a frozen artifact is handled as:

```text
FORBIDDEN_TRANSITION
    -> NOT_AUTHORIZED
    -> STOP
```

The mutation MUST NOT be applied.

## 12. Determinism

For identical:

```text
governing inputs
+ current machine state
+ authorized transition request
+ observation/evidence inputs
+ oracle result
```

the reference executor MUST produce the same machine transition and verdict classification.

Nondeterminism introduced by an agent, provider, network, or cloud environment MUST NOT alter the semantic result of the state machine. Environment-specific execution metadata may differ without changing the transition semantics.

## 13. Agent neutrality

The reference executor MUST NOT depend on a particular AI model or agent provider.

An agent may request or perform an authorized executor action, but:

```text
Agent ≠ State Model
Agent ≠ semantic authority
Agent ≠ verdict authority
```

A more capable agent MUST NOT gain additional transition or semantic authority.

## 14. STOP semantics and execution abort

`STOP` is a normal terminal **machine state**. It is distinct from an operational execution abort.

The executor MUST terminate the current validation path when reaching:

```text
OBSERVATION_GAP -> STOP
UNDERDETERMINED -> STOP
NOT_AUTHORIZED -> STOP
VERDICT -> STOP
```

The executor MUST NOT retry a terminal machine state by inventing a new transition.

A subsequent validation run requires a separately authorized execution context or governance action.

An operational failure that is not mapped by an already authorized governance rule to a machine transition MUST NOT be represented as `STOP`, `OBSERVATION_GAP`, `UNDERDETERMINED`, `NOT_AUTHORIZED`, `PASS`, or `FAIL` merely by convention.

Instead, the executor records a separate non-semantic **execution outcome**:

```text
EXECUTION_ABORTED
```

`EXECUTION_ABORTED` is NOT a State Model state and NOT a verdict class. It is an operational outcome indicating that computation could not complete. It MUST NOT be interpreted as evidence for or against the validation claim.

```text
machine state: OBSERVATION_REQUIRED
operational failure
        -> execution outcome: EXECUTION_ABORTED
        -> no semantic verdict
        -> no implicit machine-state transition
```

If an approved governance rule explicitly maps a particular operational condition to a State Model transition, that authorized transition applies. Otherwise, the executor MUST preserve the current machine state and terminate the execution attempt with `EXECUTION_ABORTED`.

## 15. Error boundary

Operational failures must not automatically acquire semantic meaning.

Examples:

```text
runner crashed
network unavailable
process timed out
artifact missing
oracle unavailable
```

These are execution conditions. The executor MUST classify them according to an already authorized rule, or terminate the execution attempt with `EXECUTION_ABORTED` without manufacturing a semantic verdict.

An operational error MUST NOT silently become `PASS` or `FAIL`.

## 16. Explicit non-goals

This specification does not:

- modify Contract semantics;
- modify C-LG or O-LG;
- modify Test Plan v1.1;
- modify or remove harness cases;
- define new Contract metadata fields;
- define new semantic predicates;
- define new State Model states or transitions;
- authorize production-code changes;
- define cloud infrastructure;
- define an agent provider protocol;
- replace the State Model as normative authority.

`EXECUTION_ABORTED` is an operational outcome defined by this executor specification, not a State Model extension and not a semantic verdict.

## 17. Conformance criteria

An implementation conforms to this specification only if it demonstrates that:

1. every authorized State Model transition is represented;
2. every unauthorized transition is blocked before execution;
3. unauthorized transitions terminate through `FORBIDDEN_TRANSITION -> NOT_AUTHORIZED -> STOP`;
4. unavailable required observations terminate through `OBSERVATION_GAP -> STOP`;
5. `PASS`/`FAIL` require admissible evidence and authorized evaluation;
6. `UNTESTED` is never inferred from missing evidence or failed execution;
7. frozen governance artifacts cannot be mutated by executor actions;
8. machine state and verdict classification remain distinct;
9. deterministic inputs produce deterministic transition/verdict semantics;
10. operational failures without an authorized semantic mapping produce `EXECUTION_ABORTED` and no semantic verdict;
11. the executor does not introduce semantic predicates or State Model extensions.

These are conformance requirements for the executor implementation, not new Contract requirements.

## 18. Current status

This document is a **proposal for review and approval**.

Until separately approved:

```text
Reference Executor Specification v0.1    PENDING REVIEW / APPROVAL
Reference Executor implementation         NOT AUTHORIZED
Executor conformance tests                NOT AUTHORIZED
Harness                                   UNCHANGED
Production code                           UNCHANGED
Contract                                  UNCHANGED
Test Plan v1.1                            UNCHANGED
Cloud execution layer                     NOT AUTHORIZED
```

Approval must be recorded separately from this specification.
