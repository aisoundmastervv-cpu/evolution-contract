# H3 Order/Position Diagnostic Verdict — Run 31929841805 v0.1

## Verdict status

- Frozen execution: `31929841805`
- Diagnostic source record: `docs/execution/h3-order-effect-diagnostic-31929841805-v0.1.md`
- Diagnostic branch: `diagnostic/h3-order-effect-31929841805`
- Diagnostic scope: existing frozen evidence only
- Execution record: **UNCHANGED**
- Raw artifact: **UNCHANGED**
- Artifact digest: **UNCHANGED**
- Registered arm: **UNCHANGED**
- `H3-ACP-001`: **UNCHANGED**
- Historical causal verdict: **INCONCLUSIVE / UNCHANGED**
- New execution: **NOT AUTHORIZED BY THIS VERDICT**

## 1. Decision question

The diagnostic question was whether the existing ten-pair execution contains evidence of a `position × treatment × temporal location` structure strong enough to resolve the interpretation analytically, or whether a residual causal ambiguity remains.

This review is strictly post-execution. It does not alter, exclude, repair, or reinterpret the frozen execution record.

## 2. Primary reconstructed evidence

The ten registered pairs alternate execution order:

- odd pairs: `control -> treatment`;
- even pairs: `treatment -> control`.

The relevant paired observations are:

| Pair | First | First ms | Second | Second ms | Second − First ms |
|---:|---|---:|---|---:|---:|
| 1 | control | 226.697 | treatment | 186.816 | -39.881 |
| 2 | treatment | 188.830 | control | 186.808 | -2.022 |
| 3 | control | 191.017 | treatment | 187.426 | -3.591 |
| 4 | treatment | 188.090 | control | 186.906 | -1.184 |
| 5 | control | 154.976 | treatment | 187.702 | +32.726 |
| 6 | treatment | 188.195 | control | 187.002 | -1.193 |
| 7 | control | 186.792 | treatment | 186.913 | +0.121 |
| 8 | treatment | 186.874 | control | 185.991 | -0.883 |
| 9 | control | 186.133 | treatment | 108.976 | -77.157 |
| 10 | treatment | 188.302 | control | 109.023 | -79.279 |

## 3. Position effect

Across all ten pairs:

- first-position mean: **188.5906 ms**;
- second-position mean: **171.3563 ms**;
- mean second-minus-first difference: **-17.2343 ms**;
- median second-minus-first difference: **-1.6075 ms**;
- second position is faster in **8/10** pairs.

The mean difference is therefore not a sufficient description of a stable position effect. It is heavily driven by pairs 9 and 10.

For pairs 1–8 only:

- first-position mean: **188.933875 ms**;
- second-position mean: **186.9455 ms**;
- mean second-minus-first difference: **-1.988375 ms**;
- median second-minus-first difference: **-1.6075 ms**.

Therefore the extreme approximately `109 ms` event is **localized to the final two adjacent pairs**, rather than appearing as a uniform first-versus-second shift throughout the execution.

## 4. Treatment-role balance

Across all ten observations per role:

- control mean: **180.1345 ms**;
- treatment mean: **179.8124 ms**.

The near-equality of the raw role means does not establish absence of a causal effect, because the registered acceptance statistic is paired treatment/control ratio and the observations contain substantial temporal/positional structure.

More importantly, pairs 9 and 10 reverse treatment role while preserving second position:

- pair 9: second = treatment = `108.976 ms`;
- pair 10: second = control = `109.023 ms`.

The difference between those two fast observations is only **0.047 ms**.

## 5. Position × treatment interaction

Define the within-pair treatment effect as:

`Δ_i = T_treatment,i − T_control,i`.

When treatment is first (pairs 2, 4, 6, 8, 10):

- mean `Δ`: **+16.9122 ms**.

When treatment is second (pairs 1, 3, 5, 7, 9):

- mean `Δ`: **−17.5564 ms**.

The corresponding difference between these two treatment-position means is approximately:

`−34.4686 ms`.

This is a strong descriptive interaction in the full ten-pair sample.

However, the interaction is not stable after removing the two temporally localized extreme pairs for sensitivity analysis:

- treatment-first, pairs 2/4/6/8: mean `Δ` = **+1.3205 ms**;
- treatment-second, pairs 1/3/5/7: mean `Δ` = **−2.65625 ms**;
- interaction difference: approximately **−3.97675 ms**.

Therefore the apparent full-sample interaction is overwhelmingly concentrated in pairs 9 and 10.

The sensitivity analysis is diagnostic only. Pairs 9 and 10 remain part of the primary evidence and are not excluded from the execution record.

## 6. Temporal-location boundary

The execution design alternates order by pair index. Consequently, treatment position is structurally linked to pair parity:

- treatment second on odd-numbered pairs;
- treatment first on even-numbered pairs.

This means that the existing ten-pair design does **not** independently randomize treatment position against temporal location. A treatment-position interaction can therefore be described, but its causal attribution to position versus temporal location cannot be uniquely identified from this execution alone.

The final two pairs are especially informative because they provide a role reversal at the same within-pair position: the second execution is approximately `109 ms` in both cases. This is evidence for a positional/temporal mechanism as a plausible explanation of the extreme observations, but it does not identify the mechanism or prove a runner confound.

## 7. Formal diagnostic assessment

| Diagnostic component | Finding | Status |
|---|---|---|
| Position main effect | 8/10 second executions faster; mean strongly shifted by pairs 9–10 | **Observed, not uniform** |
| Treatment main effect | Raw role means approximately equal | **No standalone causal inference** |
| Position × treatment | Strong in full sample; collapses substantially without pairs 9–10 | **Localized / unstable** |
| Temporal location | Extreme signal concentrated in adjacent final pairs | **Observed** |
| Position × treatment × temporal location | Existing design cannot uniquely separate position, treatment-position interaction, and temporal location at this sample size/design | **Not causally identified** |
| Pair 9–10 role reversal | Same second-position runtime level for opposite roles, difference `0.047 ms` | **Strong diagnostic signal** |
| Causal confound | No identified mechanism established | **Not established** |

## 8. Diagnostic verdict

### `DIAGNOSTIC VERDICT = RESIDUAL CAUSAL AMBIGUITY REMAINS`

The existing frozen evidence has been analytically exhausted to the extent supported by the registered ten-pair design. It establishes a **localized position/temporal signal** in pairs 9–10 and shows that the full-sample treatment-position interaction is driven predominantly by those two observations.

At the same time, the evidence does **not** establish a unique causal confound, because the existing order schedule does not provide independent identification of position against temporal location and the extreme signal is localized to two adjacent observations.

Therefore the diagnostic layer cannot resolve the causal ambiguity analytically.

## 9. Governance consequence

This verdict does **not** modify the historical causal result.

The correct current state remains:

- `RUN 31929841805 = FROZEN`;
- `RAW EVIDENCE = FROZEN`;
- `H3-ACP-001 = UNCHANGED`;
- `H3 CAUSAL VERDICT = INCONCLUSIVE`;
- `POSITION/TEMPORAL EFFECT = OBSERVED, CAUSALLY UNRESOLVED`.

The diagnostic verdict does **not** authorize an execution. It establishes only that a future execution, if proposed, would need to be a separately admitted targeted study capable of distinguishing the remaining hypotheses rather than a repetition intended to repair the historical result.

## 10. Next-execution gate

No next execution is designed or admitted by this document.

If a future proposal is opened, its minimum design question must be:

> **Which remaining hypotheses are causally indistinguishable under run 31929841805, and what registered manipulation would make them distinguishable without rewriting the frozen baseline?**

Any such proposal requires its own Proposal ID, protocol, admission decision, execution identity, artifact, and causal review.

## 11. Final closure

`EXISTING EVIDENCE ANALYSIS = COMPLETE`

`POSITION SIGNAL = OBSERVED`

`TEMPORAL LOCALIZATION = OBSERVED IN PAIRS 9–10`

`POSITION × TREATMENT = STRONGLY PRESENT IN FULL SAMPLE, NOT ROBUST TO 9–10 SENSITIVITY CHECK`

`POSITION × TREATMENT × TEMPORAL LOCATION = NOT CAUSALLY IDENTIFIED`

`CAUSAL CONFOUND = NOT ESTABLISHED`

`RESIDUAL CAUSAL AMBIGUITY = YES`

`NEW EXECUTION = NOT DESIGNED / NOT ADMITTED`

`HISTORICAL H3 VERDICT = INCONCLUSIVE / UNCHANGED`
