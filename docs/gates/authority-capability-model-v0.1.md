# Authority Capability Model v0.1 — Draft Design

**Status:** DRAFT / NOT ADMITTED

**Purpose:** define the authority boundary between an autonomous execution subject and the governance/evaluation layers before any `AuthorizedExecutionToken` or enforcement implementation is introduced.

## 1. Core principle

> **Autonomous agency without self-authorizing epistemic authority.**

An autonomous execution subject may act, mutate, propose, execute an already-authorized plan, and produce permitted observations. It must not possess the authority required to define, alter, authorize, or retroactively validate the conditions under which its own execution is considered evidence or success.

## 2. Authority domains

### 2.1 Execution subject

Permitted capabilities, subject to an already-authorized execution scope:

- `PROPOSE`
- `MUTATE_WITHIN_SCOPE`
- `EXECUTE_AUTHORIZED_PLAN`
- `OBSERVE_PERMITTED_SURFACES`
- `APPEND_EXECUTION_EVIDENCE`
- `REQUEST_NEW_AUTHORIZATION`

The subject may not mint, modify, or escalate authority.

### 2.2 Governance authority

Reserved capabilities:

- `ADMIT_PROPOSAL`
- `AUTHORIZE_EXECUTION`
- `FREEZE_BASELINE`
- `DEFINE_ORACLE`
- `DEFINE_ACCEPTANCE_CRITERIA`
- `DEFINE_EXECUTION_SCOPE`
- `REVOKE_AUTHORIZATION`
- `ISSUE_AUTHORIZATION_PROOF`

### 2.3 Evaluation/oracle authority

Reserved capabilities:

- `EVALUATE_EVIDENCE`
- `ISSUE_VERDICT`
- `DECLARE_UNDERDETERMINED`
- `DECLARE_OBSERVATION_GAP`

The execution subject cannot exercise these capabilities on its own execution lineage.

## 3. Capabilities that are forbidden to the autonomous subject

The following capabilities are authority-prohibited and must not be exposed through the subject's capability surface:

1. `AUTHORIZE_EXECUTION`
2. `MODIFY_CONTRACT`
3. `MODIFY_STATE_MODEL`
4. `MODIFY_EXECUTOR_SPEC`
5. `MODIFY_ORACLE`
6. `MODIFY_ACCEPTANCE_CRITERIA`
7. `MODIFY_FROZEN_BASELINE`
8. `DELETE_OR_REWRITE_EVIDENCE`
9. `ALTER_PROVENANCE`
10. `ISSUE_OR_ESCALATE_OWN_AUTHORITY`

These prohibitions are normative design requirements. They are not yet claimed to be machine-enforced by this document.

## 4. Capability separation invariant

For authority-changing operations:

```text
SubjectCapabilities ∩ GovernanceAuthorityCapabilities = ∅
```

The execution subject must not be able to construct an object that represents governance authority merely by invoking its own APIs.

## 5. Evidence boundary

The execution subject may append raw execution events and permitted observations. Evidence storage for an authorized lineage should be append-only from the subject's perspective.

The subject must not be able to:

- delete an observation;
- rewrite an observation;
- replace a failed execution with a successful one;
- alter event ancestry;
- alter the frozen baseline identity;
- alter the oracle or acceptance criteria after authorization.

Operational failure must remain distinguishable from semantic verdict.

## 6. Authorization proof boundary

Future execution authorization must be represented by a non-forgeable authorization proof, tentatively named:

```text
AuthorizedExecutionToken
```

The token is a future design object, not an implementation introduced by this draft.

The intended proof binding is:

```text
contract_hash
plan_hash
baseline_hash
oracle_hash
executor_spec_hash
execution_id
authorization_id
```

The autonomous subject must not be able to mint or modify this proof.

An executor accepting an execution request must require a valid authorization proof rather than treating the existence of an `ExecutionPlan` as sufficient authority.

## 7. Non-retroactivity invariant

No current execution result, observation, or subject-generated claim may acquire authority to rewrite historical evidence or its governance lineage.

In particular:

```text
current_result != authority_to_rewrite_baseline
current_observation != authority_to_define_oracle
execution != authorization
subject_claim != verdict
```

## 8. Escalation invariant

Authority escalation must not be reachable through the execution subject's capability graph.

The following path must be impossible:

```text
SUBJECT
  -> modify authorization
  -> gain governance capability
  -> authorize own execution
```

Authority-changing operations must originate from a distinct authority domain.

## 9. Enforcement layers — design target

Machine enforcement is expected to use defense in depth:

```text
Semantic Contract
        ↓
Capability-scoped API
        ↓
Typed authorization proof
        ↓
Reference Executor validation
        ↓
Sandbox / OS / container boundary
        ↓
Append-only evidence persistence
```

No single layer should be treated as the sole enforcement mechanism.

## 10. Proposed capability matrix

| Capability | Subject | Governance | Oracle/Evaluator | Executor |
|---|---:|---:|---:|---:|
| `PROPOSE` | YES | YES | NO | NO |
| `MUTATE_WITHIN_SCOPE` | YES | YES | NO | NO |
| `EXECUTE_AUTHORIZED_PLAN` | YES | YES | NO | YES |
| `OBSERVE_PERMITTED_SURFACES` | YES | YES | YES | YES |
| `APPEND_EXECUTION_EVIDENCE` | YES | YES | YES | YES |
| `ADMIT_PROPOSAL` | NO | YES | NO | NO |
| `AUTHORIZE_EXECUTION` | NO | YES | NO | NO |
| `FREEZE_BASELINE` | NO | YES | NO | NO |
| `DEFINE_ORACLE` | NO | YES | YES | NO |
| `DEFINE_ACCEPTANCE_CRITERIA` | NO | YES | YES | NO |
| `MODIFY_FROZEN_BASELINE` | NO | NO | NO | NO |
| `DELETE_OR_REWRITE_EVIDENCE` | NO | NO | NO | NO |
| `ALTER_PROVENANCE` | NO | NO | NO | NO |
| `ISSUE_OR_ESCALATE_OWN_AUTHORITY` | NO | YES* | NO | NO |
| `ISSUE_VERDICT` | NO | NO | YES | NO |

`*` Only an explicitly designated governance authority may issue authority; the execution subject cannot self-issue or escalate it.

## 11. Explicit non-goals of this draft

This draft does **not**:

- authorize any new execution;
- change H3 thresholds;
- change the frozen H3 evidence;
- introduce `AuthorizedExecutionToken` implementation;
- modify the existing H3 runner;
- grant the autonomous subject any new authority;
- establish a canonical execution authorization event.

## 12. Relationship to H3-CAP-001

`H3-CAP-001` remains:

```text
ADMITTED
```

with:

```text
execution authorization = NOT GRANTED
```

This draft is a design artifact for the next architectural stage. Its existence does not alter that boundary.

## 13. Required next governance step

Before implementing `AuthorizedExecutionToken` or enforcement code, this draft must undergo a separate governance/design review and receive its own disposition.

The intended sequence is:

```text
Authority Capability Model v0.1
        ↓
GOVERNANCE / DESIGN REVIEW
        ↓
ADMIT or REQUEST REVISION or REJECT
        ↓
AuthorizedExecutionToken design
        ↓
enforcement design
        ↓
implementation
```

Until that sequence is separately authorized, no new execution authority is created.
