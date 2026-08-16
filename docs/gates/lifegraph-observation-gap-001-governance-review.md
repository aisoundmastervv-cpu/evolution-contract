# LifeGraph Observation Gap 001 — Governance Review

Status: **RESOLVED AS OBSERVATION/SPECIFICATION BOUNDARY — NO SEMANTIC CHANGE AUTHORIZED**

## Question

Does Contract v1.0 require a separately observable state in which birth metadata is *absent*, or does the Contract merely require preservation of birth metadata as part of an accepted birth?

## Sources reviewed

- `docs/spec/evolution-application-contract-v1.0.md` — Contract v1.0, §6 Commit phase and §9 Observability and invariants.
- `docs/gates/lifegraph-semantic-baseline-v1.md` — frozen C-LG/O-LG.
- `docs/gates/lifegraph-test-plan-v1.md` — frozen test matrix.
- `docs/gates/lifegraph-harness-design-v1.md` — frozen observation boundary for harness design.
- `docs/gates/lifegraph-observation-gap-001.md` — implementation discovery record.

## Finding 1 — What Contract v1.0 actually requires

Contract v1.0 §6 states that an accepted birth:

- creates the child process and corresponding life node;
- records the parent/child relation;
- records birth metadata;
- makes the child eligible for scheduling.

Contract §9 identifies LifeGraph consistency and genealogy preservation as normative invariants and explicitly says these are independently testable.

The Contract does **not** define a separate normative representation for "birth metadata absent". It does not specify an optional field, sentinel value, invalid/null state, or other representation of metadata absence.

## Finding 2 — Is absence itself a Contract semantic dimension?

**No, not from the frozen Contract text.**

The normative semantic requirement is that an accepted birth **records birth metadata**. The Contract does not separately prescribe the representation or observability of a malformed state in which that metadata is absent.

Therefore a test that defines `birth_cycle == 0`, another sentinel, a missing map entry, a null value, or any implementation-specific encoding as "missing birth metadata" would introduce a new semantic predicate not supported by Contract v1.0.

## Finding 3 — Consequence for LG-N03b

The frozen test case `LG-N03b — Birth missing birth metadata` is stronger than the Contract's independently specified observation surface. Its intent is traceable to the requirement to record birth metadata, but its required negative state — metadata absence — has no Contract-defined observable representation.

Therefore LG-N03b is currently **not independently falsifiable from the frozen Contract + O-LG alone**.

This is a test-plan/observation mismatch, not evidence that the implementation violates the Contract.

## Decision

1. **C-LG remains FROZEN.**
2. **O-LG remains FROZEN.**
3. **Contract v1.0 remains FROZEN.**
4. No production code change is authorized by this review.
5. No sentinel or inferred absence rule may be introduced.
6. `LG-N03b` must remain blocked until a governed test-plan revision either:
   - removes the unsupported negative case; or
   - replaces it with a Contract-supported negative observation without adding semantic requirements.
7. The remaining LifeGraph cases may continue to be implemented/tested, but a complete 8-case Gate result must not be claimed while LG-N03b remains unresolved.

## Governance status

> **Observation Gap 001 = RESOLVED AS A SPECIFICATION/OBSERVATION BOUNDARY.**
>
> The Contract requires preservation/recording of birth metadata, but does not define an independently observable "absence" state.

No frozen semantic baseline is changed by this record.

## Next authorized step

A separate, narrow proposal may revise the **test plan only** to remove or replace LG-N03b, followed by adversarial review and re-freeze of the test plan. Until that happens, the harness remains blocked only for LG-N03b.
