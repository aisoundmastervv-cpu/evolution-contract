# LifeGraph Test Plan v1.1

Status: **FROZEN**

## Revision basis

This revision is governed by:

- `docs/gates/lifegraph-test-plan-v1.md` — **FROZEN historical baseline**
- `docs/gates/lifegraph-test-plan-revision-proposal-002.md` — approved disposition
- `docs/gates/lifegraph-n05b-observation-gap-002-governance-review.md`

The revision removes LG-N05b because its Contract-level counterstate is not independently observable through the frozen observation surface. No Contract, C-LG, O-LG, or production semantics are changed.

## Frozen inputs

- Contract: `docs/spec/evolution-application-contract-v1.0.md`
- Frozen semantic baseline: `docs/gates/lifegraph-semantic-baseline-v1.md`
- Claim: **C-LG**
- Oracle: **O-LG**
- Prior Test Plan: `docs/gates/lifegraph-test-plan-v1.md`
- Revision Proposal: `docs/gates/lifegraph-test-plan-revision-proposal-002.md`

This plan remains derived only from the frozen C-LG/O-LG baseline. It does not modify, reinterpret, or extend either one.

## 1. Test objective

Determine whether the implementation satisfies the frozen LifeGraph Claim under observable accepted evolution transitions, using only the frozen O-LG relations.

A passing result must never be interpreted as proof of properties outside C-LG/O-LG.

## 2. Observation surface

Each test case must produce implementation-neutral evidence sufficient to construct these projections:

- active-process projection;
- active-LifeGraph projection;
- historical/genealogical projection.

The harness may use adapters or test fixtures to expose these projections, but the Oracle must evaluate only the semantic relations defined in the frozen baseline.

## 3. Positive cases

### LG-P01 — Stable active correspondence

Setup: establish a valid active process with its required LifeGraph node.

Action: perform an accepted evolution transition that does not create or kill the process.

Expected: the active process retains its required corresponding active LifeGraph node.

Oracle relation: active-process → required active-LifeGraph correspondence.

### LG-P02 — Accepted birth

Setup: establish a valid parent process and a transition whose **semantic outcome is an accepted birth under Contract v1.0 §6**.

Action: execute that birth transition.

Expected: child process exists; corresponding active LifeGraph node exists; required parent/child relation and birth metadata are present; child is eligible for scheduling.

The harness must identify the accepted birth by the semantic transition outcome, not by a particular Rust return type, helper result, or internal control-flow signal.

Oracle relations: birth semantics in C-LG/O-LG.

### LG-P03 — Accepted death

Setup: establish an active process and a transition whose **semantic outcome is an accepted death under Contract v1.0 §6**.

Action: execute that death transition.

Expected: the formerly active process has no active LifeGraph node; required death metadata is recorded; historical representation is present in fossil/history.

The harness must identify the accepted death by the semantic transition outcome, not by a particular Rust return type, helper result, or internal control-flow signal.

Oracle relations: death semantics in C-LG/O-LG.

## 4. Negative cases

### LG-N01 — Missing active LifeGraph correspondence

Construct an observable post-transition state in which an active process lacks its required active LifeGraph node.

Expected: Oracle rejects the state.

### LG-N02 — Birth missing child LifeGraph node

Construct an accepted-birth outcome in which the child process exists but the required active LifeGraph node is absent.

Expected: Oracle rejects the state.

### LG-N03a — Birth missing parent/child relation

Construct an accepted-birth outcome in which the child process and required active LifeGraph node exist, but the required parent/child relation is absent.

Expected: Oracle rejects the state for the missing relation.

### LG-N03b — Birth missing birth metadata

Construct an accepted-birth outcome in which the child process, required active LifeGraph node, and parent/child relation exist, but required birth metadata is absent.

Expected: Oracle rejects the state for the missing metadata.

Each subcase must isolate the named missing semantic relation/metadata item so that the failure attribution is unambiguous.

### LG-N04 — Death leaves active LifeGraph node

Construct an accepted-death outcome in which the formerly active LifeGraph node remains active.

Expected: Oracle rejects the state.

### LG-N05a — Death missing historical representation

Construct an accepted-death outcome in which the active node is removed and required death metadata is present, but the required historical representation is absent from fossil/history.

Expected: Oracle rejects the state for the missing historical representation.

**LG-N05b — Death missing death metadata is removed from this revision.**

Reason: the Contract-level counterstate is valid semantically but cannot be independently constructed through the frozen implementation-neutral observation surface without introducing an unsupported semantic predicate. See `docs/gates/lifegraph-n05b-observation-gap-002-governance-review.md` and `docs/gates/lifegraph-test-plan-revision-proposal-002.md`.

This removal is a testability limitation only. It is not a waiver, reinterpretation, or satisfaction claim for the Contract-level death-metadata requirement.

## 5. Deliberate non-tests

The following are explicitly excluded because they are not frozen C-LG/O-LG requirements:

- converse requirement that every active LifeGraph node must correspond to an active process;
- `exactly one` cardinality assertions;
- graph ordering or traversal properties;
- Rust container/type requirements;
- serialization format;
- internal helper/control-flow behavior;
- scheduler semantics beyond the explicit birth requirement that the child is eligible for scheduling;
- compositional properties not stated in C-LG/O-LG;
- genealogy preservation beyond the birth/death metadata and parent/child or historical relations explicitly included in the frozen Oracle;
- any negative assertion that requires an implementation-specific encoding of Contract-level metadata absence.

## 6. Matrix review criteria

The matrix is reviewed for:

1. direct traceability to a frozen C-LG/O-LG clause;
2. an observable expected outcome;
3. a falsifiable failure condition;
4. absence of implementation-specific assumptions;
5. no dependence on another case's hidden mutable state;
6. no modification of C-LG/O-LG to accommodate a case.

The N05b revision was separately reviewed under Governance Review 002 and Proposal 002. The remaining matrix retains only cases with supported observation surfaces.

## 7. Authorization boundary

This frozen test plan authorizes **design and implementation of the LifeGraph test harness strictly from this matrix**.

It does not authorize:

- changes to C-LG/O-LG;
- changes to Contract v1.0;
- changes to production semantics for the purpose of passing tests;
- execution of the harness as a Gate verdict;
- a G12 verdict without a separately registered execution/evidence record.

Any future test-case change requires a new review and an explicit update to this frozen baseline before implementation or execution.

## 8. Revision record

**v1.1** — LG-N05b removed following Governance Review 002 and approved Test Plan Revision Proposal 002. No Contract, C-LG, O-LG, or production-code changes are included in this revision.
