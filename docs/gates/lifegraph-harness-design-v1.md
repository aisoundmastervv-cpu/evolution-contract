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

**Required observation:** observable active LifeGraph node records and their Contract-defined identity-bearing fields needed for O-LG to evaluate process-to-node correspondence.

**Adapter:** implementation-specific adapter that exposes raw semantic node records/fields only. The adapter MUST NOT compute, assert, or emit a precomputed `correspondence` predicate.

**Oracle input:** active LifeGraph semantic projection from the observed node records/fields; O-LG computes the required correspondence itself.

**Evidence:** deterministic serialized semantic projection of the observed active LifeGraph node records/fields. Evidence must not contain an adapter-generated pass/fail correspondence as the source of truth.

### OP-3 Historical/genealogical projection

**Required observation:** fossil/history representation and the parent/child plus birth/death metadata required by O-LG.

**Adapter:** implementation-specific adapter exposing only those semantic fields.

**Oracle input:** historical/genealogical semantic projection.

**Evidence:** deterministic serialized projection of the relevant history and metadata.

### OP-4 Semantic transition witness

**Required observation:** an independently specified test scenario identifying that the exercised operation is intended to be the accepted birth/death transition required by the frozen test case.

**Adapter:** test fixture/scenario definition supplies the transition preconditions and intended semantic scenario class (`birth` or `death`) before execution. The harness MUST NOT infer the scenario class from the implementation's return value, helper result, internal control flow, or post-state LifeGraph result.

**Oracle input:** the frozen scenario class plus the post-transition semantic projections required by C-LG/O-LG. The scenario class selects which already-frozen birth/death relations are evaluated; it does not add a new semantic predicate.

**Evidence:** explicit fixture/scenario record showing the preconditions and intended scenario class, plus the resulting semantic projections and Oracle verdict. The scenario record is evidence of test setup, not evidence that the implementation successfully achieved the expected outcome.

## Case mapping

| Case | Required observation | Adapter | Oracle input | Expected evidence |
|---|---|---|---|---|
| LG-P01 | OP-1 + OP-2 | active-state adapter | active process + observed node records/fields | PASS record showing O-LG correspondence |
| LG-P02 | OP-1 + OP-2 + OP-3 + OP-4 | semantic scenario + projection adapters | birth scenario class + child/node/relation/metadata/scheduling observations | PASS record containing each required birth relation |
| LG-P03 | OP-1 + OP-2 + OP-3 + OP-4 | semantic scenario + projection adapters | death scenario class + former process/node/history/metadata observations | PASS record containing each required death relation |
| LG-N01 | OP-1 + OP-2 | controlled semantic fixture/adapter | active process + observed node records/fields showing missing required node | Oracle rejection attributable to missing correspondence |
| LG-N02 | OP-1 + OP-2 + OP-4 | controlled birth fixture/adapter | birth scenario class + child without required node | Oracle rejection attributable to missing child node |
| LG-N03a | OP-1 + OP-2 + OP-3 + OP-4 | controlled birth fixture/adapter | birth scenario class + child/node present, parent/child relation absent | Oracle rejection attributable to missing relation |
| LG-N03b | OP-1 + OP-2 + OP-3 + OP-4 | controlled birth fixture/adapter | birth scenario class + child/node/relation present, birth metadata absent | Oracle rejection attributable to missing metadata |
| LG-N04 | OP-1 + OP-2 + OP-3 + OP-4 | controlled death fixture/adapter | death scenario class + formerly active node remains active | Oracle rejection attributable to active node |
| LG-N05a | OP-1 + OP-2 + OP-3 + OP-4 | controlled death fixture/adapter | death scenario class + node removed, death metadata present, history absent | Oracle rejection attributable to missing history |
| LG-N05b | OP-1 + OP-2 + OP-3 + OP-4 | controlled death fixture/adapter | death scenario class + node removed, history present, death metadata absent | Oracle rejection attributable to missing metadata |

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
2. test scenario/witness record where applicable;
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
