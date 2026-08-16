# LifeGraph Observation Gap 001

Status: **OPEN — IMPLEMENTATION BLOCKED FOR LG-N03b**

## Discovery point

Discovered during implementation of `LifeGraph Harness Design v1`.

## Frozen requirement involved

`LG-N03b — Birth missing birth metadata` requires an accepted birth outcome in which the child process, active LifeGraph node, and parent/child relation remain present while required birth metadata is absent.

## Observed implementation surface

`LifeNode.birth_cycle` is a non-optional `u64` field. The current semantic projection can observe a value for `birth_cycle`, but it has no representation of **absence** of birth metadata.

Changing `0` to another value would only change the value; it would not represent missing metadata. Treating a sentinel value as “missing” would introduce a new semantic predicate not contained in frozen C-LG/O-LG.

## Consequence

The harness can implement and evaluate the other registered LifeGraph cases without inventing a rule for missing birth metadata, but it cannot honestly implement `LG-N03b` as specified by the frozen test plan with the current observation surface.

The attempted implementation of N03b was therefore removed rather than weakened or replaced by a sentinel-based assertion.

## Governance rule applied

Per `lifegraph-harness-design-v1.md`, this is an observation gap, not permission to modify C-LG/O-LG or reinterpret the test case.

## Current harness scope

Implemented semantic harness cases:

- LG-P01
- LG-P02
- LG-P03
- LG-N01
- LG-N02
- LG-N03a
- LG-N04
- LG-N05a
- LG-N05b

Blocked:

- **LG-N03b** — birth missing birth metadata

## Next decision

No production semantic change is authorized by this record.

A separate governance review must determine whether the Contract/test-plan requirement can be observed through an already-supported semantic surface, or whether this is a genuine specification/observation mismatch requiring a new baseline decision.

Until then, full LifeGraph harness validation is blocked and no Gate verdict may claim complete coverage of the frozen 8-case matrix.
