# Authority Capability Model v0.1

Status: DRAFT / NOT ADMITTED

## 1. Purpose

Define the authority boundaries between an autonomous execution subject, governance authority, executor, and evaluation/oracle authority.

This document is a design proposal only. It creates no authority, grants no execution permission, and does not authorize implementation of an `AuthorizedExecutionToken`.

## 2. Core invariant

> An autonomous execution subject may exercise agency within an authorized scope, but it cannot create, modify, extend, or retroactively validate the authority under which its own execution is judged.

The following distinctions are normative:

```text
execution != authorization
authority != capability
evidence != verdict
observation != semantic validation
artifact existence != approval
```

## 3. Authority domains

### 3.1 Execution Subject

May, when authorized:

- propose changes;
- execute an already authorized plan;
- perform actions within the authorized scope;
- emit append-only execution observations/events;
- request additional authorization.

May not:

- authorize its own execution;
- issue or escalate its own authority;
- modify the Contract, State Model, Executor Specification, Oracle, acceptance criteria, or frozen baseline applicable to its execution;
- rewrite or delete canonical evidence or provenance;
- issue a semantic verdict on its own execution.

### 3.2 Governance Authority

May create canonical governance events within its granted governance scope:

- register/admit a proposal;
- approve a design revision;
- grant or revoke execution authorization;
- define or approve authority policy;
- freeze a baseline;
- approve the authority of an evaluator/oracle.

Governance authority does not imply permission to execute the authorized subject action.

### 3.3 Executor

May validate an authorization proof and execute only within its encoded scope.

The executor must not infer authorization from:

- file existence;
- branch existence;
- workflow existence;
- implementation presence;
- proposal status alone;
- human-readable claims without a canonical event.

### 3.4 Evaluation / Oracle Authority

May evaluate evidence according to an already authorized evaluation contract and issue the corresponding evaluation/verdict event.

It may not retroactively alter the baseline or execution authorization that produced the evidence it evaluates.

## 4. Capability registry

The following capabilities are normative categories for review:

| Capability | Execution Subject | Governance | Executor | Oracle |
|---|---:|---:|---:|---:|
| PROPOSE | YES | YES | NO | NO |
| EXECUTE_AUTHORIZED | YES | YES | YES | NO |
| ADMIT_PROPOSAL | NO | YES | NO | NO |
| APPROVE_DESIGN | NO | YES | NO | NO |
| AUTHORIZE_EXECUTION | NO | YES | NO | NO |
| REVOKE_AUTHORIZATION | NO | YES | NO | NO |
| DEFINE_CONTRACT | NO | YES* | NO | NO |
| DEFINE_STATE_MODEL | NO | YES* | NO | NO |
| DEFINE_EXECUTOR_SPEC | NO | YES* | NO | NO |
| DEFINE_ORACLE | NO | YES* | NO | YES* |
| DEFINE_ACCEPTANCE_CRITERIA | NO | YES* | NO | YES* |
| FREEZE_BASELINE | NO | YES | NO | NO |
| APPEND_OBSERVATION | YES | YES | YES | YES |
| REWRITE_EVIDENCE | NO | NO | NO | NO |
| DELETE_CANONICAL_EVIDENCE | NO | NO | NO | NO |
| ALTER_PROVENANCE | NO | NO | NO | NO |
| ISSUE_OWN_AUTHORITY | NO | NO | NO | NO |
| DECLARE_VERDICT | NO | NO | NO | YES |

`*` Requires an independently scoped governance event and must not be performed unilaterally by an execution subject.

## 5. Canonical event taxonomy

Authority is materialized only by a canonical event. The minimum event classes are:

```text
PROPOSAL_REGISTERED
PROPOSAL_ADMITTED
DESIGN_APPROVED
EXECUTION_AUTHORIZED
EXECUTION_REVOKED
AUTHORITY_EXPIRED
BASELINE_FROZEN
EVIDENCE_APPENDED
EVIDENCE_EVALUATED
VERDICT_ISSUED
```

A file, branch, workflow, implementation, token-shaped value, or status string is not itself an authority event.

## 6. Canonical event requirements

Every authority-bearing canonical event MUST bind all of the following:

```text
actor_id
actor_authority_domain
event_id
event_type
object_id
object_revision
object_hash
scope
issued_at
valid_from
valid_until
parent_event_id
```

Where applicable it MUST also bind:

```text
contract_hash
plan_hash
baseline_hash
oracle_hash
executor_spec_hash
authorization_id
reason / disposition
```

### 6.1 Exact object identity

Authority is bound to an exact object revision, not a mutable name.

Therefore:

```text
object_id alone != sufficient identity
object_id + revision/hash = identity
```

A later revision does not inherit authority merely because it has the same object ID.

### 6.2 Scope

Every authority grant MUST explicitly state its scope. Scope may constrain, as applicable:

- object;
- revision;
- execution type;
- environment;
- executor identity/specification;
- dataset/artifact identity;
- resource boundary;
- maximum execution count;
- time window.

An unspecified scope MUST NOT be interpreted as unlimited scope.

### 6.3 Time validity

Every authority grant MUST have explicit temporal validity:

```text
valid_from
valid_until
```

Both fields are mandatory. A missing `valid_from` or `valid_until` makes the authority grant invalid. No higher-level contract, inherited policy, default, or implicit convention may supply a missing boundary. No authority is perpetual by default.

## 7. Authority state transitions

The normative progression is:

```text
DRAFT
  │
  ├── PROPOSAL_REGISTERED
  ▼
REGISTERED
  │
  ├── PROPOSAL_ADMITTED
  ▼
ADMITTED
  │
  ├── DESIGN_APPROVED
  ▼
DESIGN_APPROVED
  │
  ├── EXECUTION_AUTHORIZED
  ▼
EXECUTION_AUTHORIZED
  │
  ├── EXECUTION_REVOKED
  ├── AUTHORITY_EXPIRED
  └── EXECUTION_COMPLETED
```

The following transitions are forbidden:

```text
DRAFT → EXECUTION_AUTHORIZED
REGISTERED → EXECUTION_AUTHORIZED
ADMITTED → EXECUTION_AUTHORIZED
DESIGN_APPROVED → VERDICT
EXECUTION → AUTHORIZE_OWN_EXECUTION
OBSERVATION → VERDICT_BY_SUBJECT
```

Admission therefore never implies design approval, and design approval never implies execution authorization.

## 8. Authority non-escalation

An authority-bearing actor MUST NOT be able to create an event whose authority domain is greater than the authority domain granted to that actor.

In particular:

```text
execution_subject
    cannot create
        PROPOSAL_ADMITTED
        DESIGN_APPROVED
        EXECUTION_AUTHORIZED
        EXECUTION_REVOKED
```

unless an independent governance contract explicitly delegates that exact authority. Such delegation itself must be represented by a canonical, scoped, time-bounded event.

Delegation MUST NOT permit self-authorization or authority escalation.

## 9. Provenance and lineage

Every downstream authority event MUST reference its parent event.

Example:

```text
PROPOSAL_REGISTERED
        │
        ▼
PROPOSAL_ADMITTED
        │
        ▼
DESIGN_APPROVED
        │
        ▼
EXECUTION_AUTHORIZED
        │
        ▼
EXECUTION
        │
        ▼
EVIDENCE_EVALUATED
        │
        ▼
VERDICT_ISSUED
```

An event with no valid parent lineage MUST NOT be accepted as authority-bearing.

A later event cannot repair a missing earlier authority event retroactively.

### 9.1 Canonical authority root

The authority graph MUST terminate at a separately governed canonical root. The root is the trust anchor for the authority graph and is not created by the actor whose authority it establishes.

A root MUST itself have:

```text
root_id
root_authority_domain
root_canonical_event_id
root_object_or_policy_identity
root_scope
root_valid_from
root_valid_until
root_provenance
```

The root's provenance MUST be anchored outside the authority claim being established. A root claim that is only self-attested by its subject is not a valid authority root.

For every non-root authority event:

```text
actor_authority(actor, domain)
    requires
valid_canonical_lineage(actor, domain)
    terminating at
valid_authority_root(domain)
```

No authority-bearing event may bootstrap the authority of its own issuer, and no lineage is valid merely because its first event declares itself to be a root.

## 10. Non-forgeability requirement for future authorization proofs

This model intentionally does NOT define the implementation of `AuthorizedExecutionToken`.

It does, however, define its required semantic inputs. Any future authorization proof MUST be derived from a valid canonical `EXECUTION_AUTHORIZED` event and MUST be bound to the exact:

```text
contract_hash
plan_hash
baseline_hash
oracle_hash
executor_spec_hash
execution_id
authorization_id
scope
valid_from
valid_until
```

The execution subject MUST NOT possess the authority/capability required to mint or alter such a proof.

## 11. Evidence boundary

Execution subjects may append observations/events but may not mutate canonical evidence after commitment.

Operational failure MUST NOT be converted by the subject into a semantic verdict.

Examples:

```text
OBSERVATION_UNAVAILABLE != FAIL
EXECUTION_ABORTED != FAIL
MISSING_EVIDENCE != PASS
```

The resulting state must remain within the Validation Machine semantics.

## 12. Governance review criteria

This model is ready for admission only if governance can verify all of the following:

1. Every authority type has a named holder/domain.
2. Every authority-bearing transition has a canonical event type.
3. Every event binds an exact object revision/hash.
4. Every grant has explicit scope.
5. Every grant has explicit temporal validity.
6. Every downstream authority event has valid parent lineage terminating at a valid authority root.
7. Admission, design approval, and execution authorization are distinct events.
8. The execution subject cannot issue or escalate its own authority.
9. The executor can reject execution without a valid canonical authorization event.
10. Evidence and verdict authority are separate from execution authority.
11. Revocation and expiry invalidate downstream execution capability.
12. Artifact existence cannot be interpreted as approval.
13. An authority claim cannot bootstrap or self-attest its own trust root.

## 13. Current status

```text
DESIGN STATUS: DRAFT
ADMISSION STATUS: NOT ADMITTED
IMPLEMENTATION AUTHORIZATION: NOT GRANTED
AUTHORIZED EXECUTION TOKEN: NOT DEFINED / NOT IMPLEMENTED
ENFORCEMENT IMPLEMENTATION: NOT AUTHORIZED
```

This document is therefore a design candidate, not a source of authority.
