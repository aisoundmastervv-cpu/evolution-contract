# H3 Acceptance Criterion Proposal v0.1

## Proposal identity

- Proposal ID: `H3-ACP-001`
- Status: **ADMITTED**
- Proposal type: new normative acceptance criterion
- Scope: `H3-OS-NICE-CAUSAL-v0.1`
- Basis commit: `ebf805523806720f090f76a818f924051ad58cf3`
- Execution arm registration: `H3-OS-NICE-CAUSAL-v0.1`
- H2 status: **FROZEN**
- Admission decision: **ADMIT**
- Admission commit: this commit SHA

This document is a **new methodological requirement**. It is not recovered historical policy and MUST NOT be represented as if it existed in canonical history before its admission.

## Historical status

A canonical-history audit did not establish a pre-existing H3 acceptance threshold. A numerical criterion appearing in the non-canonical PR #8 lineage was explicitly excluded from normative recovery. Therefore this criterion enters force only from its explicit admission onward.

## 1. Primary observable

The primary outcome is the ratio of treatment to control wall-clock runtime for the same deterministic CPU-bound workload:

`R_i = T_treatment,i / T_control,i`

where each pair `i` uses the same workload definition, input, environment contract, and execution arm, with only process niceness changed from `0` to `+10`.

Interpretation:

- `R > 1` means treatment took longer than control.
- `R = 1` means no runtime difference.
- `R < 1` means treatment was faster.

Process CPU time and user/system CPU time remain secondary observables for diagnosing whether a runtime difference is consistent with scheduler intervention. They are not substitutes for the primary acceptance observable.

## 2. Normative minimum effect

The H3 causal criterion is satisfied only if all of the following hold:

1. There are **10 valid paired trials**.
2. At least **9 of 10 paired runtime ratios** satisfy `R_i > 1`.
3. The **median paired ratio is at least 1.10**.

In other words, the treatment must produce a reproducible direction of effect and a median runtime increase of at least **10%**.

### Rationale for 10 paired trials

Ten paired trials are the registered minimum practical sample for this first execution arm because the comparison is within-workload and paired: each treatment observation has a directly corresponding control observation. This reduces nuisance variation introduced by unrelated workload differences while retaining a bounded execution budget.

The requirement that 9/10 pairs move in the same direction is stricter than simply requiring a positive median. Under a null model in which direction is equally likely, 9 or 10 positive pairs has a one-sided sign-test probability of approximately 0.0107. This gives the direction-of-effect requirement a pre-registered statistical meaning without relying on a fragile parametric distributional assumption.

### Rationale for the 10% effect floor

A criterion of `1.10` is a practical-effect floor rather than a claim about universal CPU scheduling behavior. A smaller observed change can be dominated by ordinary runner scheduling noise and timer variation. Requiring at least a 10% median change makes the acceptance condition distinguish a practically meaningful shift from a near-zero perturbation.

The 10% value is therefore a **new registered proposal parameter**, not an experimentally discovered constant and not a recovered historical requirement.

## 3. Repeatability and noise rules

### Valid trial

A trial is valid only if:

- environment identity is valid before execution;
- the workload revision and input are identical across the pair;
- the control process is independently observed at `nice 0`;
- the treatment process is independently observed at `nice +10`;
- both processes complete normally;
- wall-clock runtime is recorded;
- no unauthorized mutation occurs;
- raw evidence is preserved.

### Pairing

Control and treatment MUST be paired by workload instance. A failed or invalid member invalidates the pair; it MUST NOT be silently replaced after inspecting the other member's result.

### Ordering

To reduce systematic temporal bias, trial pairs SHOULD alternate order across the run sequence (control→treatment, then treatment→control) while preserving the registered actuator values. The order must be recorded in raw evidence.

### No outlier deletion

No trial may be removed because its result is inconvenient, surprising, or statistically unusual. Invalidity must be established by a pre-registered protocol violation, not by the numerical result itself.

### Noise classification

If fewer than 10 valid pairs are produced, the result is **INCONCLUSIVE / INSUFFICIENT EVIDENCE**, not a causal failure.

If 10 valid pairs exist but fewer than 9 have `R_i > 1`, the directional criterion is not satisfied.

If 10 valid pairs exist, at least 9 have `R_i > 1`, but median `R < 1.10`, the result is **INCONCLUSIVE / EFFECT BELOW REGISTERED FLOOR**.

## 4. Secondary consistency checks

The following checks are required for interpretation but do not independently create causal support:

- treatment process niceness is observed as `+10`;
- control process niceness is observed as `0`;
- environment identity is unchanged across valid paired execution;
- workload input and revision are identical;
- process CPU time and wall-clock time are both recorded.

A large runtime effect accompanied by contradictory or missing actuator provenance is not causal evidence.

## 5. Causal acceptance semantics

### PASS — causal support

All of the following must be true:

- 10 valid paired trials;
- 9 or 10 pairs have `R_i > 1`;
- median `R >= 1.10`;
- actuator provenance complete;
- environment provenance complete;
- raw evidence complete;
- no protocol violation.

### INCONCLUSIVE

Use when the execution produced valid evidence but the registered criterion is not met, including insufficient valid pairs or effect below the registered floor.

### INVALID / NON-EVIDENTIARY

Use when the protocol itself was violated, provenance is incomplete, the actuator cannot be independently verified, or environment identity is invalid.

### INFRASTRUCTURE FAILURE

Use only when workflow/runner infrastructure prevents the experiment from materializing. Do not reinterpret infrastructure failure as experimental failure.

## 6. Anti-post-hoc rules

After admission, the following MUST NOT be changed after seeing H3 results:

- number of required pairs;
- 9/10 directional requirement;
- 1.10 median-effect floor;
- control parameter;
- treatment parameter;
- primary observable;
- exclusion rules.

Any change requires a new Proposal ID and a new admission cycle. Existing results MUST NOT be used to justify retroactive threshold selection.

## 7. Admission record

This criterion is now **ADMITTED** as `H3-ACP-001`.

Admission means the numerical parameters are normative constraints for `H3-OS-NICE-CAUSAL-v0.1` and may now be consumed by the H3 workload runner. The runner MUST consume them as contract/configuration data rather than silently embedding a different threshold.

Admission does **not** constitute H3 execution, H3 evidence, or causal support.

## 8. Decision

`H3-ACP-001 = ADMITTED`

`H3 RUN = NOT EXECUTED`

`H3 CAUSAL EVIDENCE = NONE`
