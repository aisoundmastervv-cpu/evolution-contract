# Authority Capability Model v0.1 — Governance Review

**Status:** REQUEST REVISION  
**Reviewed object:** `Authority Capability Model v0.1`  
**Proposal surface:** PR #21  
**Reviewed revision:** `eec076d54cff2889de5282b4b9d0318bf07073d7`  
**Base:** `main` at `e8070f115437c0ba470cf4ee239724a4727e7e21`  
**Review disposition:** `REQUEST REVISION`  

## 1. Scope of this review

This record reviews the draft Authority Capability Model as a governance/design object only.

This review does **not** grant:

- design approval;
- implementation authorization;
- `AuthorizedExecutionToken` issuance authority;
- enforcement implementation authority;
- H3 execution authorization;
- threshold modification authority;
- any new execution authority.

The reviewed artifact remains `DRAFT / NOT ADMITTED`.

## 2. Positive findings

The draft establishes the correct architectural principle:

> **Autonomous agency without self-authorizing epistemic authority.**

It correctly separates execution subject capabilities from governance capabilities, prohibits self-authorization, prohibits retrospective evidence rewriting, and identifies a future non-forgeable authorization proof as a separate design object.

The draft also correctly preserves the existing H3 boundary:

```text
H3-CAP-001 = ADMITTED
Execution authorization = NOT GRANTED
```

## 3. Governance findings requiring revision

### Finding A — Canonical event taxonomy is incomplete

The model must explicitly define the canonical event corresponding to each authority transition, including at minimum:

```text
proposal registration
proposal admission
execution/design approval
execution authorization
authorization revocation
observation/evidence evaluation
authoritative verdict
```

Each event must bind at minimum:

```text
actor/domain
object identity
exact revision/hash
scope
parent event
timestamp
resulting authority state
```

### Finding B — Design approval and execution authorization are not sufficiently separated

`ADMIT_PROPOSAL` is not equivalent to design approval, and design approval is not equivalent to execution authorization.

The model must define distinct capabilities and canonical events for these boundaries.

Required invariant:

```text
proposal admission
    != design approval
    != execution authorization
```

### Finding C — Oracle and acceptance-criteria authority is ambiguous

`DEFINE_ORACLE` and `DEFINE_ACCEPTANCE_CRITERIA` appear across Governance and Oracle/Evaluator domains.

The revised model must specify:

- who may create the oracle/criteria;
- who may apply them;
- who may change them;
- when they become immutable for an execution lineage;
- how a subject is prevented from influencing criteria used to evaluate its own execution.

### Finding D — Authorization proof requires stronger binding semantics

The proposed `AuthorizedExecutionToken` fields are directionally correct but insufficient as a complete non-forgeability contract.

The revised design must define:

- exact object/revision binding;
- authorization scope;
- issuer identity/domain;
- issuance event identity;
- proof authenticity/non-forgeability mechanism;
- replay prevention;
- expiry/revocation semantics;
- executor validation failure semantics.

### Finding E — Authority-state transition semantics are incomplete

The model must explicitly define the legal state transitions, including:

```text
DRAFT
  -> ADMITTED
  -> DESIGN_APPROVED
  -> EXECUTION_AUTHORIZED
```

and the corresponding rejection, revision, revocation, expiry, and invalidation paths.

It must also state explicitly:

> Existence of a file, branch, implementation, workflow, machine-readable projection, or executable capability does not itself constitute authority.

### Finding F — Evidence materialization needs a sharper boundary

`APPEND_EXECUTION_EVIDENCE` currently risks conflating subject-generated observations with canonical evidence.

The revised model must distinguish:

```text
subject observation
    -> collected evidence
    -> oracle evaluation
    -> canonical verdict
```

and specify which domain is permitted to materialize each event.

### Finding G — Scope and lineage invariants must be explicit

An authorization must not be replayable across:

- another proposal;
- another revision;
- another contract;
- another baseline;
- another oracle;
- another execution;
- another scope.

The revised model must define the identity and lineage constraints that make such substitution invalid.

## 4. Disposition

```text
REQUEST REVISION
```

The draft is **not rejected** because its architectural direction is sound. It is not admitted because the authority model is not yet sufficiently canonical and machine-checkable at the event/state-transition level.

## 5. Explicit downstream prohibition

Until a revised Authority Capability Model receives a separate governance disposition:

```text
AuthorizedExecutionToken implementation = NOT AUTHORIZED
Enforcement implementation = NOT AUTHORIZED
H3 execution = NOT AUTHORIZED
H3 threshold modification = NOT AUTHORIZED
New execution authorization = NOT GRANTED
```

## 6. Required next step

Revise `Authority Capability Model v0.1` to address Findings A–G, create a new exact revision, and submit that revision for a fresh governance review.

No revision may retroactively alter frozen evidence or the canonical `H3-CAP-001` admission boundary.
