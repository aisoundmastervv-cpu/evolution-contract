# H3-CAP-001 — Causal Ambiguity Resolution Proposal v0.1

**Status:** PROPOSED / NOT ADMITTED

**Proposal ID:** H3-CAP-001

**Proposal class:** Design / causal-ambiguity resolution

**Parent frozen execution:** `31929841805`

**Frozen execution commit:** `31ec257f07e4ec0013228f196f2b93a10e607ece`

**Frozen evidence artifact:** `h3-os-nice-causal-evidence-31ec257f07e4ec0013228f196f2b93a10e607ece`

**Artifact digest:** `sha256:2abc6d7e099cb08537bb93083fa331567d98839c4aea7bd03163e5643b2ddbdb`

**Frozen verdict:** `INCONCLUSIVE`

## 1. Purpose

H3-CAP-001 exists to resolve one specific residual causal ambiguity identified in frozen H3 evidence.

It does **not** revise, repair, reinterpret, or invalidate the historical execution `31929841805`. The frozen execution, its raw evidence, and its `INCONCLUSIVE` verdict remain immutable historical evidence.

The governing invariant is:

> A new proposal exists to resolve a specific causal ambiguity discovered in frozen evidence; it does not exist to repair the historical result.

## 2. Frozen baseline

The proposal is anchored to the frozen execution exactly as recorded:

- workflow run: `31929841805`;
- execution commit: `31ec257f07e4ec0013228f196f2b93a10e607ece`;
- arm: `H3-OS-NICE-CAUSAL-v0.1`;
- proposal/config identity used by the historical run: `H3-ACP-001`;
- environment identity: `769dd253bbe337d630018b1b6c09729399e57ad3312568054007c218a19a4b77`;
- configuration SHA-256: `56528fc374697de3938459dee11a38baae02796daf22b0db7ec373a216a565cc`;
- workload revision: `h3-cpu-workload-v0.1`;
- required paired trials: `10`;
- valid paired trials: `10`;
- positive pairs: `7`;
- registered median ratio: `1.0055411383275983`;
- registered verdict: `INCONCLUSIVE`.

This baseline is frozen. No value in this section is a target to be improved by the proposal.

## 3. Residual causal ambiguity

The frozen execution alternated within-pair execution order:

- odd-numbered pairs: control first, treatment second;
- even-numbered pairs: treatment first, control second.

The frozen evidence contains a distinctive reciprocal pattern in pairs 9 and 10:

- pair 9: control `186.133 ms`, treatment `108.976 ms`, ratio `0.58547`;
- pair 10: treatment `188.302 ms`, control `109.023 ms`, ratio `1.72718`.

The unusually low wall time occurs for the **second executed member** in both adjacent pairs, while the role receiving that position changes. The two observations therefore do not permit the historical contrast to be interpreted uniquely as a treatment effect.

The residual ambiguity is:

> **Is the observed treatment/control wall-time contrast attributable to the treatment assignment itself, or can it be explained by an execution-order effect that is correlated with treatment/control position within the frozen paired design?**

This is a diagnostic finding about the evidence. It is not a new claim that an order effect has been proven, and it does not modify the historical verdict.

## 4. Competing explanations to be discriminated

H3-CAP-001 does not adopt a new H3 causal hypothesis. It registers the minimum competing explanations that a future admitted design must be able to distinguish:

### E1 — Treatment-associated explanation

The treatment assignment contributes causally to the observed wall-time difference, independently of which role executes first within a pair.

### E2 — Order-associated explanation

Execution position or sequence contributes materially to the observed wall-time difference, such that role assignment alone is insufficient to identify the treatment effect from the frozen design.

These are **proposal-level discriminating explanations**, not an admission of either explanation as true.

## 5. Resolution objective

The proposal succeeds only if a future admitted design can distinguish E1 from E2 using a pre-specified comparison that does not depend on selectively removing pairs, changing the historical threshold, or retrospectively reclassifying the frozen evidence.

The design must therefore make execution order an explicitly identifiable causal factor rather than leaving it implicit in the paired sequence.

The proposal does not specify the runner, workflow implementation, threshold, sample count, or execution parameters. Those belong to a later execution design and require their own governance treatment after admission.

## 6. Non-goals and prohibitions

H3-CAP-001 MUST NOT be used to:

- alter the frozen execution record;
- alter the `INCONCLUSIVE` historical verdict;
- discard pairs 9 or 10 because they are inconvenient;
- change `MIN_POSITIVE_PAIRS` or `MEDIAN_RATIO_FLOOR` to obtain a preferred outcome;
- reinterpret `H3-ACP-001` as though it contained this proposal;
- treat the proposal's existence as execution authorization;
- encode a runner or workflow implementation before admission;
- preselect a desired causal conclusion.

## 7. Required proposal criteria

Before admission, governance must be able to identify, in the proposal revision being considered:

1. the frozen baseline above;
2. the exact residual ambiguity being resolved;
3. the competing explanations being discriminated;
4. the factor that must become causally identifiable (execution order/position);
5. the prospective evidence criterion that can distinguish the explanations;
6. the boundary between proposal design and execution implementation;
7. the admission authority and decision record required before any execution can occur.

## 8. Admission boundary

Creation of H3-CAP-001 establishes **no execution authority**.

The state transition is strictly:

```text
PROPOSED
  |
  | independent governance decision
  v
ADMITTED
  |
  | separate execution authorization / lineage
  v
EXECUTABLE
```

Until an explicit canonical admission record exists:

- H3-CAP-001 remains `NOT ADMITTED`;
- no new execution may claim authority from H3-CAP-001;
- no runner may encode H3-CAP-001 as an admitted policy;
- the frozen execution remains closed and immutable.

## 9. Required admission record

If governance admits H3-CAP-001, the admission record MUST identify at minimum:

1. `H3-CAP-001`;
2. the exact approved proposal revision;
3. the frozen baseline used for the decision;
4. the causal ambiguity accepted as the scope of resolution;
5. the approved prospective discrimination criterion;
6. the applicable A1/A2/EEC-003 governance dependencies;
7. the decision authority and timestamp;
8. the resulting authorization boundary for any later execution design.

Admission of this proposal must not silently admit an execution arm, runner, threshold, or workflow.

## 10. Lineage rule

If H3-CAP-001 is admitted and subsequently implemented, the resulting execution MUST form a new provenance lineage:

```text
H3 frozen execution 31929841805
        |
        +--> frozen evidence / INCONCLUSIVE
        |
        +--> diagnostic finding: residual order-vs-treatment ambiguity
                    |
                    +--> H3-CAP-001 (this proposal)
                              |
                              +--> admission record (future, separate)
                                        |
                                        +--> execution design (future, separate)
                                                  |
                                                  +--> new execution record
                                                            |
                                                            +--> new evidence
                                                            |
                                                            +--> new causal review
```

No future result may rewrite the historical branch of this lineage.

## 11. Current governance state

```text
H3 FROZEN EXECUTION                 CLOSED
H3 FROZEN EVIDENCE                  IMMUTABLE
DIAGNOSTIC ANALYSIS                 COMPLETE
RESIDUAL CAUSAL AMBIGUITY           IDENTIFIED
H3-CAP-001                          PROPOSED
H3-CAP-001 ADMISSION                NOT RECORDED
NEW EXECUTION                       NOT AUTHORIZED
RUNNER DESIGN                       NOT STARTED
```

**Decision boundary:** the next governance action is review and disposition of H3-CAP-001 itself. No execution work is authorized by this artifact.
