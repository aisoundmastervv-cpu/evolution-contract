# LifeGraph Harness Design v1 — Observation-Surface Review

Status: **REVIEWED — REVISIONS REQUIRED**

## Scope

Review OP-1 through OP-4 for:

1. observability;
2. traceability to frozen C-LG/O-LG;
3. absence of hidden semantic requirements;
4. absence of circularity between adapter and Oracle;
5. falsifiable evidence production.

No harness implementation is authorized by this review.

## Findings

| Surface | Finding | Verdict |
|---|---|---|
| OP-1 Active-process projection | Observable identity set required by the frozen Oracle; does not add a semantic predicate by itself. | PASS |
| OP-2 Active-LifeGraph projection | Observable, but the phrase “node identity/correspondence information” risks making the adapter compute the very correspondence the Oracle is supposed to test. | REVISE |
| OP-3 Historical/genealogical projection | Observable semantic fields are directly required by the frozen Oracle; adapter must expose raw required relations/metadata rather than precomputed pass/fail. | PASS WITH CONSTRAINT |
| OP-4 Semantic transition outcome | Not yet independently defined as an observation surface. “Semantic outcome” can become circular if derived from the same implementation result that the test is intended to validate. | REVISE |

## Finding 1 — OP-1

The active-process identity set is a legitimate observation input. The Oracle consumes it to determine whether the required corresponding LifeGraph node exists.

No additional cardinality or converse relation is introduced.

**Verdict: PASS.**

## Finding 2 — OP-2 circularity risk

The current wording asks the adapter to expose “node identity/correspondence information.” If “correspondence” is already computed by the adapter, the adapter would contain part of O-LG and could make a failing relation invisible to the Oracle.

Required correction: OP-2 must expose the observable LifeGraph node identifiers and the contract-required fields needed to evaluate correspondence, but the adapter must not precompute the Oracle verdict or silently synthesize a process→node mapping.

This is an observation-interface correction, not a change to C-LG/O-LG.

## Finding 3 — OP-3

The historical/genealogical projection is valid because the frozen Oracle explicitly observes fossil/history plus required parent/child and birth/death metadata.

The adapter must expose those fields as observations and must not return a precomputed “history consistent” boolean.

**Verdict: PASS WITH CONSTRAINT.**

## Finding 4 — OP-4 circularity / acceptance oracle

The current design treats “semantic transition outcome” as if it were already observable. Contract v1.0 defines birth/death semantics, but the harness design does not yet identify an implementation-independent observation from which “accepted birth” or “accepted death” can be established.

If OP-4 is implemented by reading a production return value, helper result, or internal flag, it risks making the implementation itself the oracle. If OP-4 is computed from the same LifeGraph state being tested, it risks circularity.

Required correction: define OP-4 as an externally supplied test fixture classification for cases where the test deliberately invokes a transition scenario known by the test setup to be a birth/death scenario, or define a separate independent transition witness whose semantics are established before observing the LifeGraph result. The harness must not infer acceptance from the result under test.

This correction changes only harness design. It does not add a new Contract semantic requirement.

## Finding 5 — Evidence

“Deterministic serialized semantic projection” is acceptable as an evidence format requirement, but the serialization must be performed after observation and must not normalize away the negative condition being tested.

The exact serialization format remains implementation-neutral.

**Verdict: PASS.**

## Overall verdict

The design is conceptually sound but **not yet implementation-ready**.

Two observation-surface corrections are required:

1. revise OP-2 so the adapter exposes observable node data without precomputing correspondence;
2. revise OP-4 so birth/death classification is an independent test fixture/witness, not an inferred result of the implementation under test.

No C-LG/O-LG change is permitted.

Until these corrections and a repeat review pass:

- harness implementation: NOT AUTHORIZED;
- production changes: NOT AUTHORIZED;
- execution: NOT AUTHORIZED;
- Gate verdict: NOT AUTHORIZED.
