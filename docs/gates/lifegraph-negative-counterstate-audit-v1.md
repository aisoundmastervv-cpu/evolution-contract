# LifeGraph Negative Counterstate Audit v1

Status: **REVIEWED — ONE CASE UNSUPPORTED**

## Scope

Audit every LifeGraph negative case in frozen Test Plan v1 for one question:

> Does the negative case correspond to a Contract-supported semantic counterstate, or does it merely describe an implementation-level corruption invented by the test?

Frozen inputs:

- Contract v1.0
- C-LG/O-LG
- LifeGraph Test Plan v1

No frozen semantic baseline is changed by this audit.

## Criteria

A negative case is **Contract-supported** only if:

1. the positive property being violated is explicitly normative in Contract v1.0 §6 / frozen O-LG;
2. the counterstate can be described entirely using Contract-supported semantic observations;
3. rejection does not depend on a sentinel, Rust representation, helper result, container property, or other implementation-specific convention;
4. the expected failure identifies the violated frozen relation rather than an invented encoding of "missing".

## Results

| Case | Frozen positive property | Semantic counterstate | Independent observation | Verdict |
|---|---|---|---|---|
| LG-N01 | active process has required active LifeGraph node | active process present; required node absent | yes | **SUPPORTED** |
| LG-N02 | accepted birth creates corresponding active LifeGraph node | accepted birth child present; corresponding node absent | yes | **SUPPORTED** |
| LG-N03a | accepted birth records parent/child relation | child/node present; required relation absent | yes | **SUPPORTED** |
| LG-N03b | accepted birth records birth metadata | child/node/relation present; birth metadata absent | no Contract-defined absence state | **UNSUPPORTED** |
| LG-N04 | death removes active LifeGraph node | accepted death; formerly active node remains active | yes | **SUPPORTED** |
| LG-N05a | death places historical representation in fossil/history | active node removed; required history representation absent | yes | **SUPPORTED** |
| LG-N05b | death records death metadata | historical representation present; required death metadata absent | yes | **SUPPORTED** |

## Case findings

### LG-N01 — SUPPORTED

The Contract explicitly requires an active process to have a corresponding active life node. The counterstate "active process exists while required node is absent" is directly expressible through the frozen active-process and active-LifeGraph projections. No implementation convention is needed.

### LG-N02 — SUPPORTED

The Contract explicitly requires an accepted birth to create the child process and corresponding active life node. A child without that node is therefore a direct semantic counterstate. The test does not need to define how absence is encoded internally; absence is evaluated at the semantic projection level.

### LG-N03a — SUPPORTED

The Contract explicitly requires the parent/child relation to be recorded. A birth state in which the child/node exist but the required relation is absent is a direct semantic counterstate. The Oracle observes relation presence rather than a Rust representation.

### LG-N03b — UNSUPPORTED

The Contract requires birth metadata to be recorded, but does not define an independently observable semantic state representing "birth metadata absent". The current implementation's `birth_cycle: u64` therefore cannot be assigned a sentinel meaning without adding a new predicate. The frozen O-LG likewise does not define an absence encoding.

This case must not be implemented by treating `0`, a default value, an empty record, or another implementation convention as "missing metadata".

### LG-N04 — SUPPORTED

The Contract explicitly requires a death to remove the active life node. Retaining that node after an accepted death is a direct semantic violation observable through the active-LifeGraph projection.

### LG-N05a — SUPPORTED

The Contract explicitly requires the resulting historical representation to be placed in fossil/history. A death state with the active node removed but the required historical representation absent is a direct counterstate at the semantic projection level.

### LG-N05b — SUPPORTED

The Contract explicitly requires death metadata to be recorded. The semantic audit must distinguish metadata presence from its encoding; the negative fixture removes the required semantic field from the projection rather than assigning an implementation sentinel. If the current implementation cannot produce such a semantic projection, that is a harness observation limitation, not a new Contract rule. The case remains Contract-supported in principle.

## Overall finding

Six negative cases are directly grounded in Contract-supported semantic counterstates. **LG-N03b is the sole unsupported negative case under the current frozen specification.**

This audit therefore does not justify broadening or weakening C-LG/O-LG. The correct governance action is to remove or replace N03b through a separately reviewed test-plan amendment if full matrix coverage is required.

## Governance boundary

This audit does not modify:

- Contract v1.0;
- C-LG;
- O-LG;
- frozen Test Plan v1.

It records an analysis result only. Any change to the test plan requires a new review and a new explicit freeze before harness implementation/execution may claim full coverage.
