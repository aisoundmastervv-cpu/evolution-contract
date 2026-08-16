# LifeGraph Test Plan v1

Status: **REVIEW DRAFT — TEST EXECUTION NOT AUTHORIZED**

## Frozen inputs

- Contract: `docs/spec/evolution-application-contract-v1.0.md`
- Frozen semantic baseline: `docs/gates/lifegraph-semantic-baseline-v1.md`
- Claim: **C-LG**
- Oracle: **O-LG**

This plan is derived only from the frozen C-LG/O-LG baseline. It does not modify, reinterpret, or extend either one.

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

Setup: establish a valid parent process and a transition that produces an accepted child.

Action: execute the accepted birth transition.

Expected: child process exists; corresponding active LifeGraph node exists; required parent/child relation and birth metadata are present; child is eligible for scheduling.

Oracle relations: birth semantics in C-LG/O-LG.

### LG-P03 — Accepted death

Setup: establish an active process and a transition that produces an accepted death.

Action: execute the accepted death transition.

Expected: the formerly active process has no active LifeGraph node; required death metadata is recorded; historical representation is present in fossil/history.

Oracle relations: death semantics in C-LG/O-LG.

## 4. Negative cases

### LG-N01 — Missing active LifeGraph correspondence

Construct an observable post-transition state in which an active process lacks its required active LifeGraph node.

Expected: Oracle rejects the state.

### LG-N02 — Birth missing child LifeGraph node

Construct an accepted-birth outcome in which the child process exists but the required active LifeGraph node is absent.

Expected: Oracle rejects the state.

### LG-N03 — Birth missing parent/child relation or birth metadata

Construct an accepted-birth outcome in which the child and active node exist but a required parent/child relation or birth metadata is absent.

Expected: Oracle rejects the state.

### LG-N04 — Death leaves active LifeGraph node

Construct an accepted-death outcome in which the formerly active LifeGraph node remains active.

Expected: Oracle rejects the state.

### LG-N05 — Death missing historical representation or required death metadata

Construct an accepted-death outcome in which the active node is removed but the required historical representation or death metadata is absent.

Expected: Oracle rejects the state.

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
- genealogy preservation beyond the birth/death metadata and parent/child or historical relations explicitly included in the frozen Oracle.

## 6. Matrix review criteria

Before implementation is authorized, each case must be reviewed for:

1. direct traceability to a frozen C-LG/O-LG clause;
2. an observable expected outcome;
3. a falsifiable failure condition;
4. absence of implementation-specific assumptions;
5. no dependence on another case's hidden mutable state;
6. no modification of C-LG/O-LG to accommodate the case.

## 7. Authorization boundary

This document is a test-plan draft. It authorizes **review of the matrix only**.

It does not authorize:

- test harness implementation;
- changes to production code;
- changes to C-LG/O-LG;
- G12 workflow execution;
- a G12 verdict.

Implementation may begin only after the matrix passes review and this plan is explicitly accepted/frozen as the test baseline.
