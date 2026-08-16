# LifeGraph Harness Design v1

Status: **DESIGN REVIEW — IMPLEMENTATION NOT YET STARTED**

## Frozen inputs

- Contract: `docs/spec/evolution-application-contract-v1.0.md`
- Semantic baseline: `docs/gates/lifegraph-semantic-baseline-v1.md`
- Test plan: `docs/gates/lifegraph-test-plan-v1.md`

This design is derived strictly from the frozen LifeGraph Test Plan v1. It does not modify C-LG/O-LG or add semantic requirements.

## Design rule

For every case:

`case → required observation → adapter → Oracle input → expected evidence`

The harness is an observation mechanism, not a second specification. Adapters may translate implementation state into semantic projections, but the Oracle receives only Contract-supported observations.

## Observation model

### OP-1 Active-process projection

**Required observation:** identities of processes that are active for the transition under test.

**Adapter:** implementation-specific adapter that exposes only the process identity set needed by O-LG.

**Oracle input:** active-process identities.

**Evidence:** deterministic serialized semantic projection containing the observed active-process identities.

### OP-2 Active-LifeGraph projection

**Required observation:** active LifeGraph node information needed to establish the required process-to-node correspondence.

**Adapter:** implementation-specific adapter that exposes node identity/correspondence information without requiring the Oracle to inspect graph containers or Rust types.

**Oracle input:** active LifeGraph semantic projection.

**Evidence:** deterministic serialized semantic projection of active LifeGraph correspondence data.

### OP-3 Historical/genealogical projection

**Required observation:** fossil/history representation and the parent/child plus birth/death metadata required by O-LG.

**Adapter:** implementation-specific adapter exposing only those semantic fields.

**Oracle input:** historical/genealogical semantic projection.

**Evidence:** deterministic serialized projection of the relevant history and metadata.

### OP-4 Semantic transition outcome

**Required observation:** whether the exercised transition has the semantic outcome required to classify it as the accepted birth/death transition specified by the test case.

**Adapter:** a semantic test fixture/transition boundary. It must not infer acceptance from a Rust return type, helper result, or internal control-flow signal alone.

**Oracle input:** transition classification required by the frozen test case.

**Evidence:** explicit transition-outcome record sufficient to justify why P02/P03/N02/N03/N04/N05 are evaluated under the relevant birth/death semantics.

## Case mapping

| Case | Required observation | Adapter | Oracle input | Expected evidence |
|---|---|---|---|---|
| LG-P01 | OP-1 + OP-2 | active-state adapter | active process + required node correspondence | PASS record showing required correspondence |
| LG-P02 | OP-1 + OP-2 + OP-3 + OP-4 | semantic transition + projection adapters | child, node, parent/child relation, birth metadata, scheduling eligibility | PASS record containing each required birth relation |
| LG-P03 | OP-1 + OP-2 + OP-3 + OP-4 | semantic transition + projection adapters | former process inactive in LifeGraph, death metadata, history | PASS record containing each required death relation |
| LG-N01 | OP-1 + OP-2 | controlled semantic fixture/adapter | active process without required node | Oracle rejection attributable to missing correspondence |
| LG-N02 | OP-1 + OP-2 + OP-4 | controlled birth fixture/adapter | child without required node | Oracle rejection attributable to missing child node |
| LG-N03a | OP-1 + OP-2 + OP-3 + OP-4 | controlled birth fixture/adapter | child/node present, parent/child relation absent | Oracle rejection attributable to missing relation |
| LG-N03b | OP-1 + OP-2 + OP-3 + OP-4 | controlled birth fixture/adapter | child/node/relation present, birth metadata absent | Oracle rejection attributable to missing metadata |
| LG-N04 | OP-1 + OP-2 + OP-3 + OP-4 | controlled death fixture/adapter | formerly active node remains active | Oracle rejection attributable to active node |
| LG-N05a | OP-1 + OP-2 + OP-3 + OP-4 | controlled death fixture/adapter | node removed, death metadata present, history absent | Oracle rejection attributable to missing history |
| LG-N05b | OP-1 + OP-2 + OP-3 + OP-4 | controlled death fixture/adapter | node removed, history present, death metadata absent | Oracle rejection attributable to missing metadata |

## Observation-gap rules

An observation gap is raised if any case requires an observation that cannot be obtained without:

- changing C-LG/O-LG;
- introducing a semantic predicate not present in the frozen baseline;
- inspecting implementation internals as an Oracle criterion;
- inferring a semantic result solely from an implementation-specific return/control-flow detail.

An observation gap is **not** resolved by weakening or strengthening the Oracle. It must be recorded for review.

## Evidence requirements

Each harness execution must be capable of producing:

1. case identifier;
2. semantic transition outcome where applicable;
3. the relevant semantic projections;
4. Oracle input derived from those projections;
5. Oracle verdict;
6. failure attribution for negative cases;
7. deterministic machine-readable evidence suitable for later Gate recording.

The design does not prescribe a particular serialization format or implementation language.

## Non-goals

The harness will not test:

- converse LifeGraph-to-process correspondence;
- `exactly one` cardinality;
- graph ordering/traversal;
- Rust container/type structure;
- serialization semantics;
- internal helper/control flow;
- unregistered scheduler properties;
- broader genealogy preservation outside frozen C-LG/O-LG.

## Authorization boundary

This document registers the harness design and observation contract only.

Implementation may begin only after design review confirms that every required observation is obtainable without changing frozen semantics.

If implementation reveals an observation gap, implementation stops for that case and the gap is recorded. No C-LG/O-LG change is permitted as a workaround.
