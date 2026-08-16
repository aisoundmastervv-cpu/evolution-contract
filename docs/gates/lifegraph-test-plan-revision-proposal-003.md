# LifeGraph Test Plan Revision Proposal 003

Status: **PROPOSAL — NOT APPROVED**

## Scope

This proposal addresses the observation findings for **LG-P03** and **LG-N03b** identified by the metadata observation review.

It does **not** modify:

- Contract v1.0;
- C-LG;
- O-LG;
- production semantics;
- the current harness;
- frozen Test Plan v1.1.

The purpose of this proposal is to determine whether P03 and N03b can remain in the test plan without introducing an unsupported semantic interpretation of implementation fields.

## 1. Finding

The current implementation exposes metadata-related fields such as `birth_cycle`, `death_cycle`, and `death_reason` on `LifeNode`. These fields are implementation-level observations. The Contract, however, requires birth/death metadata at Contract level without defining those Rust fields, their exact composition, or an encoding for metadata presence/absence.

Therefore neither of the following implications is independently justified by the frozen semantic layer:

```text
implementation field present
    -> Contract-level metadata record present
```

or:

```text
implementation field absent
    -> Contract-level metadata record absent
```

Using either implication as an Oracle predicate would introduce a new semantic predicate.

## 2. P03 — Accepted death

### 2.1 Preserve

The following observable Contract-supported components remain valid:

- accepted death semantic outcome;
- formerly active process has no active LifeGraph node;
- historical representation is present in fossil/history.

### 2.2 Remove from P03 assertion

The assertion that the current implementation's `death_cycle` and/or `death_reason` fields prove the presence of the Contract-level death-metadata record must be removed.

### 2.3 Result

P03 may remain as an **accepted-death structural/history observation**, but its metadata component is explicitly **UNTESTED — OBSERVATION GAP**.

This is not a waiver of the Contract requirement and must not be reported as a metadata pass.

## 3. N03b — Birth missing birth metadata

### 3.1 Preserve

The following observable components remain valid:

- accepted birth semantic outcome;
- child process exists;
- corresponding active LifeGraph node exists;
- required parent/child relation exists.

### 3.2 Remove

The negative fixture requiring the Oracle to prove that Contract-level birth metadata is absent must be removed unless an implementation-neutral semantic projection is established independently of this proposal.

In particular, `birth_cycle` must not be promoted to the definition of Contract-level birth metadata merely to preserve N03b.

### 3.3 Result

N03b is proposed for **REMOVAL** from the active test matrix because its named negative state is not independently observable through the frozen observation surface.

The Contract-level birth-metadata requirement remains **UNTESTED — OBSERVATION GAP**.

## 4. No semantic invention

This proposal explicitly rejects the following substitutions:

- `birth_cycle` == Contract birth metadata;
- `death_cycle` == Contract death metadata;
- `death_reason` == Contract death metadata;
- `None` == Contract metadata absent;
- `Some(...)` == Contract metadata present;
- any conjunction/disjunction of those implementation fields as a newly defined semantic record.

No new semantic predicate is authorized by this proposal.

## 5. Proposed v1.2 disposition

If approved, Test Plan v1.2 should:

1. retain P01 and P02 unchanged;
2. retain the observable death components of P03;
3. retain the observable birth components of P02 and existing relation checks;
4. remove N03b as a metadata-absence negative case;
5. remove metadata-presence from P03 as a gate assertion;
6. preserve explicit traceability that Contract-level birth/death metadata requirements are **UNTESTED due to observation gap**;
7. preserve N01, N02, N03a, N04, and N05a subject to their independent conformance review;
8. leave Contract v1.0, C-LG, O-LG, and production semantics unchanged.

## 6. Governance condition

This proposal is **not** itself a Test Plan revision and does not authorize harness changes.

A separate approval record is required before creating a new frozen Test Plan baseline.

Until approval:

- Test Plan v1.1 remains the active frozen baseline;
- the harness remains unchanged;
- P03/N03b metadata assertions remain blocked;
- no Gate verdict may treat Contract-level metadata as passed.

## 7. Traceability

This proposal follows:

- Governance Review 002 for the original N05b observation gap;
- the metadata observation review for P03/N03b;
- Test Plan v1.1's requirement that Oracle evaluation use only frozen semantic relations and implementation-neutral evidence.

## 8. Decision requested

**Approve / Reject Test Plan Revision Proposal 003.**

Requested disposition: **APPROVE**, followed by creation of a new frozen Test Plan baseline only after the approval is separately recorded.
