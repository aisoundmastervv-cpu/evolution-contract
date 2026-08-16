# H3-CAP-001 — Execution Design v0.1

**Status:** DESIGN / NOT AUTHORIZED FOR EXECUTION

**Proposal ID:** `H3-CAP-001`

**Design class:** Causal-disambiguation execution design

**Parent admission record:** `docs/gates/h3-cap-001-admission-record-v0.1.md`

**Admitted proposal revision:** `8778f0798c23557911ebc9c855533ac4575e21f0`  
**Admitted proposal blob SHA:** `428c8e0efb13e6f67935909bb10ef83566a0cfd2`  
**Frozen workflow run:** `31929841805`  
**Frozen execution commit:** `31ec257f07e4ec0013228f196f2b93a10e607ece`  
**Frozen verdict:** `INCONCLUSIVE`

## 1. Purpose

This document defines the prospective execution design admitted as the next governance stage for H3-CAP-001.

Its sole purpose is to make execution order/position an identifiable causal factor so that a future execution can distinguish:

- **E1 — Treatment-associated explanation:** treatment assignment contributes causally to the observed wall-time difference independently of execution position;
- **E2 — Order-associated explanation:** execution position or sequence contributes materially to the observed wall-time difference.

This design does **not** revise, repair, reinterpret, or invalidate frozen execution `31929841805` or its `INCONCLUSIVE` verdict.

## 2. Design invariant

> A future execution exists to resolve the admitted causal ambiguity; it does not exist to repair the historical result or to obtain a preferred verdict.

The historical branch remains immutable and causally closed.

## 3. Identifiable factors

The future design treats the following as distinct causal factors:

- **Treatment assignment:** control vs treatment;
- **Execution position:** first vs second within a paired comparison;
- **Pair/block:** the matched unit used to control workload-level variation.

For every paired observation there is exactly one control execution and one treatment execution. The order of those two executions is independently assigned at the pair level.

The design therefore does not attempt to infer order from treatment. Order is an explicit prospective assignment variable.

## 4. Prospective comparison structure

Each pair contains:

```text
one control execution
one treatment execution
one pre-assigned execution order
one paired comparison
```

The order factor has two levels:

```text
O = -1  treatment first, control second
O = +1  control first, treatment second
```

Future pairs MUST be allocated to both order levels using a pre-specified allocation rule with no adaptive reassignment based on observed outcomes. The design MUST target balance between the two order levels over the planned sample.

The order assignment MUST be recorded before the pair executes and MUST be immutable after execution begins.

The concrete randomization mechanism and exact sample count are execution-design implementation details that require separate governance review before execution authorization.

## 5. Primary estimand

For pair `i`, define the paired treatment contrast:

```text
D_i = Y_T,i - Y_C,i
```

where `Y_T,i` is the treatment wall-time and `Y_C,i` is the control wall-time for the same pair.

Define:

```text
Z_i = +1  if treatment executes second
Z_i = -1  if treatment executes first
```

The primary prospective model is:

```text
D_i = tau + delta * Z_i + epsilon_i
```

where:

- `tau` is the treatment-associated paired effect after accounting for the order factor;
- `delta` is the order-associated contribution to the paired contrast;
- `epsilon_i` is residual pair-level variation.

Under balanced prospective assignment of `Z`, `tau` is identified by the mean paired contrast across the two order strata, while the difference between the strata identifies the order contribution. Equivalently:

```text
estimated treatment effect = mean(D | Z=+1 and Z=-1)
order contribution          = [mean(D | Z=+1) - mean(D | Z=-1)] / 2
```

The sign convention is fixed by this document and MUST NOT be changed after observing results.

## 6. Why this resolves the frozen ambiguity

The frozen paired execution alternated role order deterministically, producing the diagnostic possibility that treatment/control contrast was correlated with execution position.

This design instead varies execution position prospectively across otherwise comparable treatment/control pairs and records the position factor explicitly.

A future result therefore produces two independently interpretable quantities:

1. the treatment-associated component `tau`;
2. the order-associated component `delta`.

A large observed paired contrast that disappears or changes materially across order strata is evidence relevant to E2. A treatment-associated contrast that remains stable across the two order strata is evidence relevant to E1.

Neither pattern is to be interpreted from a single pair or from selectively excluded observations.

## 7. Prospective evidence criterion

The evidence criterion is fixed before execution:

### E1-supporting pattern

E1 may be supported only if all of the following hold in the pre-specified analysis:

1. the estimated treatment effect `tau` is directionally consistent with the registered treatment hypothesis;
2. the treatment-associated effect remains present after the order factor is included;
3. the estimated order contribution `delta` does not by itself account for the observed treatment-associated contrast;
4. all valid prospective pairs are included under the pre-specified validity rules;
5. no historical pair, threshold, or analysis rule is modified to obtain the result.

### E2-supporting pattern

E2 may be supported if the prospective data show that the treatment/control contrast materially changes with execution position and the estimated order contribution explains the observed contrast to a degree that makes treatment assignment non-identifying under the pre-specified analysis.

### INCONCLUSIVE pattern

The result MUST remain `INCONCLUSIVE` if the prospective data do not provide sufficient information to discriminate E1 from E2 under the pre-specified analysis, including failure of required order balance, loss of required paired observations, or evidence that the execution environment invalidates the comparison.

No numerical threshold is introduced by this design. Any quantitative decision threshold required for a future causal verdict MUST be separately specified and governed before execution authorization.

## 8. Validity and exclusion rules

Validity rules MUST be prospective and fixed before execution.

The design MUST NOT:

- discard a pair because its result is inconvenient;
- exclude an order stratum after observing its outcome;
- change the estimand after seeing the data;
- alter historical thresholds;
- merge the new observations into frozen execution `31929841805`;
- use the future result to reclassify the historical `INCONCLUSIVE` verdict.

Any invalid pair must be retained in the provenance record with its explicit invalidity reason. It MUST NOT silently disappear from the lineage.

## 9. Required recorded fields

Every future pair must record at minimum:

- new execution identity;
- pair identity;
- treatment assignment;
- execution position for each member;
- order assignment `Z`;
- workload identity/revision;
- environment identity;
- start/end or wall-time measurement;
- validity status and reason if invalid;
- runner/workflow revision;
- relevant governance-policy revision.

The order assignment MUST be observable independently of treatment assignment in the evidence artifact.

## 10. Relationship to frozen evidence

The historical execution remains a separate provenance branch:

```text
Frozen execution 31929841805
        |
        +--> frozen evidence
        +--> INCONCLUSIVE
        +--> residual order-vs-treatment ambiguity
        |
        +--> H3-CAP-001 admission
                    |
                    +--> this execution design
                              |
                              +--> future authorized execution
                                        |
                                        +--> new evidence
                                        +--> new causal review
```

No future execution may mutate or overwrite the frozen branch.

## 11. A1 / A2 / EEC-003 dependencies

This design does not silently admit execution infrastructure.

Before execution authorization, governance MUST reconcile this design with the applicable:

- A1 registration and semantics;
- A2 registration and semantics;
- EEC-003 environment-integrity requirements;
- established A2 capability boundary;
- frozen workload and environment identity requirements.

In particular, the previously observed A2 cgroup capability boundary is an environment fact, not an authorization to weaken the preflight or redesign the registered arm.

If the required environment capability remains unavailable, the future execution MUST stop as `NOT AUTHORIZED` or otherwise enter the applicable non-execution state; it MUST NOT proceed by weakening the admitted environment requirement.

## 12. Execution authorization boundary

This document is a design artifact only.

It does NOT authorize:

- runner implementation;
- workflow implementation;
- execution parameter selection outside a separately approved design;
- A2 registration changes;
- threshold changes;
- execution dispatch;
- new evidence collection;
- causal verdict issuance.

The state remains:

```text
H3-CAP-001 admission          ADMITTED
Execution design              THIS ARTIFACT
Design authorization          UNDER REVIEW
Execution authorization       NOT GRANTED
New execution                NOT AUTHORIZED
Causal verdict               NOT AVAILABLE
```

## 13. Required next governance transition

The next governance action is a **separate review of this execution design**.

That review MUST determine whether:

1. the proposed order factor is causally identifiable;
2. the paired comparison is sufficient to discriminate E1/E2;
3. the prospective evidence criterion is adequately pre-specified;
4. validity/exclusion rules are sufficiently closed before observation;
5. A1/A2/EEC-003 dependencies are correctly incorporated;
6. the design is ready for a separate execution-authorization decision.

Only after an explicit governance disposition of this design may the project consider implementation or execution authorization.

## 14. Canonical state

```text
FROZEN EVIDENCE                    CLOSED / IMMUTABLE
H3-CAP-001                         ADMITTED
EXECUTION DESIGN                   DRAFT / UNDER REVIEW
ORDER FACTOR                       EXPLICIT / PROSPECTIVELY ASSIGNED
E1/E2 DISCRIMINATION               PRE-SPECIFIED
EXECUTION AUTHORIZATION            NOT GRANTED
RUNNER IMPLEMENTATION              NOT AUTHORIZED
WORKFLOW IMPLEMENTATION            NOT AUTHORIZED
NEW EXECUTION                      NOT AUTHORIZED
CAUSAL VERDICT                     NOT AVAILABLE
```

**This document defines the next design stage only. It creates no execution authority.**
