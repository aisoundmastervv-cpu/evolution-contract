# H3-CAP-001 — Execution Design v0.4

**Status:** DESIGN / NOT AUTHORIZED FOR EXECUTION

**Proposal ID:** `H3-CAP-001`

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

After this revision, the assignment rule, sample size, validity rules, estimand, evidence rule, confidence procedure, and stopping rule are all fixed prospectively by this document. None may be changed after the first observation is materialized.

## 3. Identifiable factors

The future design treats the following as distinct causal factors:

- **Treatment assignment:** control vs treatment;
- **Execution position:** first vs second within a paired comparison;
- **Pair/block:** the matched unit used to control workload-level variation.

For every paired observation there is exactly one control execution and one treatment execution. The order of those two executions is independently assigned at the pair level.

The design therefore does not attempt to infer order from treatment. Order is an explicit prospective assignment variable.

## 4. Prospective comparison structure and assignment

Each pair contains:

```text
one control execution
one treatment execution
one pre-assigned execution order
one paired comparison
```

The order factor has two levels:

```text
Z = -1  treatment first, control second
Z = +1  treatment second, control first
```

### 4.1 Fixed sample

The protocol schedules exactly **24 pairs**, with exactly **12 pairs in each order stratum**.

There is **no adaptive sample extension**. If fewer than 12 valid pairs are available in either order stratum after the 24 scheduled pairs, the prospective result is `INCONCLUSIVE`.

### 4.2 Fixed allocation mechanism

Order assignment MUST use **permuted blocks of four pairs**, with exactly two `Z=-1` and two `Z=+1` assignments in each block.

Before the first pair executes:

1. a random seed MUST be generated and recorded;
2. the complete 24-pair allocation list MUST be generated from that seed;
3. the allocation list MUST be materialized as an immutable pre-execution artifact;
4. the artifact identity MUST be recorded in the execution governance context.

No assignment may be regenerated, reordered, or replaced after the first observation is materialized. The seed and allocation list MUST be available in the evidence lineage.

The assignment mechanism MUST NOT use workload outcome, wall time, failure status, or any post-assignment observation as an input.

### 4.3 No post-hoc balancing

The 24-pair schedule is fixed before execution. A failed or invalid pair is **not replaced** by a new pair selected to restore a preferred result. All 24 scheduled pair identities and their eventual validity status remain in provenance.

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

With the fixed balanced allocation, the estimands are identified by the two order strata:

```text
mean_plus  = mean(D | Z=+1)
mean_minus = mean(D | Z=-1)

tau_hat   = (mean_plus + mean_minus) / 2
delta_hat = (mean_plus - mean_minus) / 2
```

The sign convention is fixed by this document and MUST NOT be changed after observing results.

The primary analysis MUST use all valid scheduled pairs and the pre-specified stratum assignments. No pair may be reclassified between strata.

## 6. Identification assumptions

The causal interpretation requires all of the following prospective assumptions:

1. treatment/control assignment within a pair is defined before execution;
2. order assignment is generated independently of workload outcomes and is fixed before execution;
3. both order strata contain the same treatment/control contrast definition;
4. the pair is the blocking unit and pair identity is not selected after observing outcomes;
5. the measured wall-time outcome uses the same measurement definition across both order strata;
6. environment validity is assessed by pre-specified rules rather than by the observed treatment contrast;
7. no execution-stage rule is changed after observing an outcome.

If any assumption required for identification is shown to be false, the causal analysis MUST terminate as `INCONCLUSIVE` rather than being repaired retrospectively.

## 7. Why this resolves the frozen ambiguity

The frozen paired execution alternated role order deterministically, producing the diagnostic possibility that treatment/control contrast was correlated with execution position.

This design instead varies execution position prospectively across otherwise comparable treatment/control pairs and records the position factor explicitly.

A future result therefore produces two independently interpretable quantities:

1. the treatment-associated component `tau_hat`;
2. the order-associated component `delta_hat`.

A treatment-associated contrast that remains stable across order strata supports E1. A contrast that changes systematically with order supports E2. The pre-specified statistical decision rule below determines whether either pattern is strong enough for a causal verdict.

Neither pattern is to be interpreted from a single pair or from selectively excluded observations.

## 8. Prospective evidence and verdict criterion

The primary decision rule is fixed before execution.

### 8.1 Exact confidence procedure

Let the valid observations in the two order strata be indexed by `+` and `-`, with counts `n_+` and `n_-`, sample means `m_+` and `m_-`, and sample variances `s_+^2` and `s_-^2` for `D_i`.

The protocol requires `n_+ >= 12` and `n_- >= 12` for any causal verdict.

The estimators are:

```text
m_+ = mean(D | Z=+1)
m_- = mean(D | Z=-1)

tau_hat   = (m_+ + m_-) / 2
delta_hat = (m_+ - m_-) / 2
```

Because the two order strata are distinct prospective samples, the standard error for both estimators is fixed as:

```text
SE_tau   = 0.5 * sqrt(s_+^2 / n_+ + s_-^2 / n_-)
SE_delta = 0.5 * sqrt(s_+^2 / n_+ + s_-^2 / n_-)
```

The confidence interval uses the **Welch-Satterthwaite degrees of freedom** for the corresponding two-stratum contrast:

```text
v = (s_+^2/n_+ + s_-^2/n_-)^2 /
    ((s_+^2/n_+)^2/(n_+-1) + (s_-^2/n_-)^2/(n_--1))
```

For a two-sided 95% interval, let `t_0.975,v` be the Student-t critical value with `v` degrees of freedom. Then:

```text
CI_tau   = tau_hat   +/- t_0.975,v * SE_tau
CI_delta = delta_hat +/- t_0.975,v * SE_delta
```

This is the complete primary confidence procedure. There is no separate within-stratum CI followed by an informal combination.

The implementation MUST use these formulas, the Welch-Satterthwaite degrees-of-freedom calculation, two-sided 95% coverage, and full-precision intermediate values. Rounding is permitted only for presentation after the decision has been computed. No bootstrap, Bayesian interval, robust interval, pooled-variance interval, or alternative confidence procedure may replace this primary procedure without a new design revision and governance decision.

The confidence procedure is symmetric for `tau_hat` and `delta_hat`; their only difference is the fixed contrast definition.

### 8.2 Fixed E1 criterion

E1 may be supported **only if all** of the following hold:

1. `CI_tau` excludes zero;
2. `tau_hat > 0`;
3. `CI_delta` includes zero;
4. `n_+ >= 12` and `n_- >= 12`;
5. no identification assumption in Section 6 is violated;
6. all environment-integrity requirements for the valid observations pass.

There is no alternative E1 interpretation rule.

### 8.3 Fixed E2 criterion

E2 may be supported **only if all** of the following hold:

1. `CI_delta` excludes zero;
2. `n_+ >= 12` and `n_- >= 12`;
3. `CI_tau` includes zero;
4. no identification assumption in Section 6 is violated;
5. all environment-integrity requirements for the valid observations pass.

The sign of `delta_hat` MUST be reported exactly as observed. No post-hoc relabelling of the order direction is permitted.

There is **no joint-interpretation fallback, secondary model, or discretionary rule** that may convert another pattern of confidence intervals into an E2 causal verdict.

### 8.4 Fixed INCONCLUSIVE criterion

The result MUST be `INCONCLUSIVE` if any of the following hold:

- `n_+ < 12` or `n_- < 12`;
- `CI_tau` and `CI_delta` both exclude zero;
- `CI_tau` and `CI_delta` both include zero;
- `CI_delta` excludes zero while `CI_tau` also excludes zero;
- `CI_tau` excludes zero but `tau_hat <= 0`;
- any identification assumption fails;
- the environment invalidates the comparison;
- required observations are missing under the validity rules;
- the result satisfies neither the fixed E1 nor the fixed E2 criterion.

No secondary analysis may override the primary verdict rule. Any exploratory analysis MUST be labelled exploratory and MUST NOT determine the causal verdict.

## 9. Validity and exclusion rules

Validity is determined by execution integrity, not by the observed value of `D_i`.

A scheduled pair is **valid** only if all of the following are true:

1. exactly one control and one treatment execution are present;
2. both executions use the same registered pair/workload identity;
3. the pre-assigned order is preserved;
4. both required wall-time observations are present and produced by the registered measurement mechanism;
5. required environment-integrity checks pass for both executions;
6. no forbidden mutation of workload, assignment, threshold, or governance policy occurs;
7. the evidence artifact contains the required identity/provenance fields.

A pair is **invalid** if any validity condition fails.

Invalid pairs MUST:

- remain in the provenance record;
- retain their original pair identity and assigned order;
- record an explicit invalidity reason;
- contribute zero observations to the primary estimator;
- count against the fixed 24-pair schedule;
- never be replaced by a newly selected pair.

The following are explicitly **not** valid reasons for exclusion:

- an inconvenient treatment effect;
- an inconvenient order effect;
- a surprising wall time;
- failure to match a desired hypothesis;
- an intermediate result that appears inconclusive.

No validity rule may be introduced or modified after the first observation.

## 10. Fixed stopping rule

The protocol stops after the **24 scheduled pairs** have reached a terminal validity state.

There is no outcome-dependent early stopping and no outcome-dependent extension.

The only permitted early stop is a protocol/environment failure that makes further authorized execution impossible; such a stop yields `INCONCLUSIVE` and records the reason.

A complete prospective causal analysis therefore requires:

```text
24 scheduled pairs
12 scheduled Z=-1
12 scheduled Z=+1
all pair validity states recorded
at least 12 valid pairs in each stratum
```

If these conditions are not met, no E1/E2 causal verdict may be issued.

## 11. Required recorded fields

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
- relevant governance-policy revision;
- pre-execution allocation artifact identity.

The order assignment MUST be observable independently of treatment assignment in the evidence artifact.

## 12. Protocol immutability boundary

After the first observation is materialized, the following are immutable for the prospective run:

- order allocation mechanism;
- generated allocation list;
- scheduled pair count;
- treatment/control definitions;
- estimand and sign convention;
- confidence procedure;
- evidence/verdict rule;
- validity/exclusion rules;
- stopping rule;
- registered thresholds and governance dependencies.

Any requested change after first observation requires a **new design revision and a new governance decision**. It MUST NOT be applied to the active prospective run.

## 13. Relationship to frozen evidence

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
                              +--> future separately authorized execution
                                        |
                                        +--> new evidence
                                        +--> new causal review
```

No future execution may mutate or overwrite the frozen branch.

## 14. A1 / A2 / EEC-003 dependencies

This design does not silently admit execution infrastructure.

Before execution authorization, governance MUST reconcile this design with the applicable:

- A1 registration and semantics;
- A2 registration and semantics;
- EEC-003 environment-integrity requirements;
- established A2 capability boundary;
- frozen workload and environment identity requirements.

In particular, the previously observed A2 cgroup capability boundary is an environment fact, not an authorization to weaken the preflight or redesign the registered arm.

If the required environment capability remains unavailable, the future execution MUST stop as `NOT AUTHORIZED` or otherwise enter the applicable non-execution state; it MUST NOT proceed by weakening the admitted environment requirement.

## 15. Execution authorization boundary

This document is a design artifact only.

It does NOT authorize:

- runner implementation;
- workflow implementation;
- execution parameter selection outside this approved design;
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
New execution                 NOT AUTHORIZED
Causal verdict                NOT AVAILABLE
```

## 16. Required next governance transition

The next governance action is a **separate formal design-approval review**.

That review MUST determine whether:

1. the order factor is causally identifiable;
2. the paired comparison is sufficient to discriminate E1/E2;
3. the fixed 24-pair allocation and block randomization are acceptable;
4. the exact confidence procedure and directional criterion are adequately pre-specified;
5. validity/exclusion rules are sufficiently closed before observation;
6. A1/A2/EEC-003 dependencies are correctly incorporated;
7. the design is ready for a separate execution-authorization decision.

Only after an explicit governance disposition of this design may the project consider implementation or execution authorization.

## 17. Canonical state

```text
FROZEN EVIDENCE                    CLOSED / IMMUTABLE
H3-CAP-001                         ADMITTED
EXECUTION DESIGN                   v0.4 / READY FOR FORMAL DESIGN REVIEW
ORDER FACTOR                       EXPLICIT / PROSPECTIVELY ASSIGNED
SCHEDULE                            FIXED: 24 PAIRS / 12 PER ORDER STRATUM
RANDOMIZATION                      FIXED: PERMUTED BLOCKS OF FOUR
ESTIMAND                            FIXED: tau_hat / delta_hat
CONFIDENCE PROCEDURE               FIXED: WELCH-SATTERTHWAITE + TWO-SIDED 95% STUDENT-t
TREATMENT DIRECTION                FIXED: tau > 0 FOR E1
E2 JOINT-INTERPRETATION FALLBACK   FORBIDDEN
VALIDITY RULES                     FIXED BEFORE OBSERVATION
STOPPING RULE                      FIXED: 24 SCHEDULED PAIRS
EXECUTION AUTHORIZATION            NOT GRANTED
RUNNER IMPLEMENTATION              NOT AUTHORIZED
WORKFLOW IMPLEMENTATION            NOT AUTHORIZED
NEW EXECUTION                      NOT AUTHORIZED
CAUSAL VERDICT                     NOT AVAILABLE
```

**This document defines the next design stage only. It creates no execution authority.**
