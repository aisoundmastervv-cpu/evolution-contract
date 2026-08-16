# LifeGraph N05b — Observation Gap 002 Governance Review

Status: **RESOLVED — GENUINE OBSERVATION GAP / NO SEMANTIC CHANGE AUTHORIZED**

## Question

Can the current supported semantic observation surface expose the Contract-level N05b counterstate without introducing a new semantic predicate, implementation-specific absence convention, or frozen-layer change?

## Frozen basis

- Contract v1.0 — §6 and §9 — **FROZEN**
- C-LG / O-LG — **FROZEN**
- LifeGraph Test Plan v1 — **FROZEN**
- N05b source audit — `lifegraph-n05b-death-metadata-source-audit-v1.md`
- Observation Gap 002 — `lifegraph-observation-gap-002.md`
- Implementation revision audited: `validation/real-two-fix` at `fbd903a3ec8e85af2fdd7a8c050b3fa1d385921c`

No frozen document or production code is changed by this review.

## Review method

The observation surface was checked in five steps:

1. identify the implementation object corresponding to historical representation;
2. identify the semantic projection available for that object;
3. determine whether the projection represents the required death-metadata record as a whole;
4. determine whether the N05b counterstate can be constructed without selecting an implementation field as the Contract definition of metadata absence;
5. determine whether O-LG can receive the observation and independently evaluate it without an adapter-generated verdict.

## Finding 1 — Historical representation is observable

The current LifeGraph implementation stores historical nodes in `life_graph.fossils`. The harness exposes these through a historical/genealogical projection.

**Result: PASS.**

The historical representation itself is therefore not the observation gap.

## Finding 2 — Death metadata is observable only through implementation-level components

The current historical projection exposes at least:

- `death_cycle: Option<u64>`;
- `death_reason: Option<DeathReason>`;

These are independently observable implementation fields.

However, frozen Contract v1.0 does not define either field as the Contract-level meaning of "death metadata", nor does it define a semantic encoding for absence of the composite death-metadata record.

**Result: LIMITATION CONFIRMED.**

## Finding 3 — `None` cannot be promoted to Contract-level absence

The following transitions are not Contract-authorized semantic interpretations:

```text
 death_reason = None
        ↓
 required death-metadata record absent
```

or

```text
 death_cycle = None
        ↓
 required death-metadata record absent
```

or any conjunction/disjunction of those fields treated as the canonical Contract encoding of record absence.

Doing so would introduce a new semantic predicate that is absent from Contract v1.0.

**Result: REJECTED AS ORACLE CONSTRUCTION.**

## Finding 4 — Composite absence is not independently constructible

The required N05b counterstate is:

> accepted death + active node removed + historical representation present + required death-metadata record absent.

The current observation surface can independently expose the first, second, and third components. It cannot independently expose the fourth as a Contract-level semantic fact.

It can expose values of individual implementation fields, but that is not equivalent to observing absence of the Contract-defined record.

**Result: NO.**

## Finding 5 — Oracle independence remains intact

O-LG can consume the raw historical projection and evaluate relations from it. The adapter need not emit `metadata_missing = true` or any equivalent precomputed predicate.

The problem is earlier: there is no Contract-supported observation from which O-LG could derive that predicate without first assigning semantic meaning to implementation-level absence.

Therefore this is not an Oracle circularity problem. It is a missing semantic observation surface.

**Result: INDEPENDENCE PRESERVED; OBSERVATION INSUFFICIENT.**

## Finding 6 — No already-supported projection resolves the gap

The available projections are:

- active-process;
- active-LifeGraph;
- historical/genealogical.

None adds an independent Contract-level death-metadata-record presence/absence observation. The historical projection is the only relevant surface, and its available decomposition does not supply a separately defined composite-record presence predicate.

**Result: NO EXISTING SUPPORTED PROJECTION RESOLVES N05b.**

## Verdict

> **N05b = GENUINE OBSERVATION GAP.**

The Contract authority for N05b remains **YES**: Contract v1.0 requires death metadata to be recorded.

The current implementation observation surface does **not** provide an independent Contract-level observation of absence of that required record.

Therefore the current N05b fixture cannot be made Gate-valid merely by choosing a different `Option` value or combination of implementation fields.

## Governance consequences

1. Contract v1.0 remains **FROZEN**.
2. C-LG/O-LG remain **FROZEN**.
3. LifeGraph Test Plan v1 remains **FROZEN**.
4. No production semantic change is authorized.
5. No sentinel or implementation-specific absence predicate may be introduced.
6. N05b remains blocked for execution as a valid semantic negative case.
7. The next decision belongs to a separate, explicitly governed **Test Plan revision proposal**: either remove N05b or replace it with a negative case that is independently observable under the existing frozen semantic surface.

## Important non-decision

This review does **not** conclude that the implementation violates Contract v1.0.

It concludes only that the current observation mechanism cannot independently falsify the Contract-level death-metadata requirement by constructing its absence.

> **Boundary closed: Contract-supported requirement, observation unsupported.**
