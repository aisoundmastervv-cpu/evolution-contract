# H3 Causal Result Review — Run 31929841805 v0.1

## Review status

- Reviewed execution: `31929841805`
- Execution commit: `31ec257f07e4ec0013228f196f2b93a10e607ece`
- Arm: `H3-OS-NICE-CAUSAL-v0.1`
- Acceptance proposal: `H3-ACP-001`
- Raw artifact SHA-256: `2abc6d7e099cb08537bb93083fa331567d98839c4aea7bd03163e5643b2ddbdb`
- Review classification: **VALID EXECUTION / INCONCLUSIVE RESULT**

## 1. What was actually tested

The registered causal hypothesis states that, for the same deterministic CPU-bound workload, changing only the independently observable OS scheduler priority from `nice 0` to `nice +10` changes execution outcome in the direction predicted by reduced scheduling priority.

The registered primary observable is the paired wall-clock runtime ratio:

`R_i = T_treatment,i / T_control,i`

The admitted criterion requires 10 valid pairs, at least 9/10 ratios above 1, and median ratio at least 1.10.

The execution preserved the registered actuator values, workload revision, work units, environment identity, and execution commit across all valid pairs. The workflow also alternated control-first and treatment-first order across successive pairs, as requested by the acceptance protocol.

## 2. Integrity of the execution

The execution is evidentiary rather than infrastructural:

- environment identity: valid;
- runner validation: PASS;
- valid paired trials: 10/10;
- invalid pairs: 0;
- observed control niceness: 0;
- observed treatment niceness: 10;
- workload revision: identical;
- work units: identical;
- workload checksum: identical within each pair;
- raw evidence artifact: materialized;
- provenance: complete for the recorded fields.

Therefore the result is not `INVALID`, `NON-EVIDENTIARY`, or `INFRASTRUCTURE FAILURE`.

## 3. Acceptance result

Observed:

- positive pairs: **7/10**;
- median ratio: **1.0055411383275983**;
- required positive pairs: **9/10**;
- required median: **1.10**.

Both registered acceptance conditions are therefore unmet.

The correct registered classification is:

**INCONCLUSIVE / ACCEPTANCE CRITERION NOT MET**.

The workflow's technical conclusion is `failure` because the acceptance-evaluation step intentionally exits non-zero when the registered criterion is not satisfied. That CI failure MUST NOT be reclassified as an infrastructure failure.

## 4. What the result says about H3

The result does **not provide causal support** for the registered hypothesis under H3-ACP-001.

There is a weak directional signal in the raw sample: 7 of 10 treatment/control ratios are above 1. However, the central effect estimate is effectively null relative to the registered practical-effect floor: the median ratio is approximately `1.0055`, corresponding to about a 0.55% median increase rather than the required 10%.

Because the acceptance criterion was explicitly registered as the decision rule, this execution cannot be upgraded to PASS by interpreting the 7/10 direction count in isolation.

At the same time, this run does **not establish that the OS niceness intervention has no causal effect in general**. Failure to meet a pre-registered minimum-effect criterion is not equivalent to proving the null hypothesis. The result may reflect a small effect, execution noise, environment-specific scheduler behavior, or insufficient sensitivity of this workload/measurement arrangement.

Therefore the correct scientific/governance statement is:

> **This execution did not establish the registered H3 causal effect at the pre-registered 10% median-effect and 9/10 directional threshold. It remains inconclusive with respect to the broader causal hypothesis.**

## 5. Important diagnostic feature: possible temporal/order effect

Two adjacent pairs show a particularly strong positional pattern:

- Pair 9: control `186.133 ms`, treatment `108.976 ms` → `R = 0.5855`.
- Pair 10: treatment `188.302 ms`, control `109.023 ms` → `R = 1.7272`.

The faster observation is therefore the **second execution in both pairs**, almost exactly at the same wall-clock level (`108.976` vs `109.023` ms), despite the treatment role reversing between the two pairs.

This is a strong diagnostic indication that at least part of the observed variation may be associated with execution position/time-dependent runner state rather than niceness itself. It is not, by itself, proof of a confound: only two adjacent pairs display this extreme pattern, and other pairs are less dramatic. But it is important evidence against interpreting the raw 7/10 direction count as a clean causal signal.

The acceptance protocol already required alternating order, and the workflow implements that alternation. Thus this observation is not a protocol violation. It is a reason for caution in interpreting the result.

## 6. Interpretation boundary

The following conclusions are justified:

1. The registered intervention was successfully executed.
2. The environment and actuator provenance were materially established.
3. Ten valid paired observations were obtained.
4. The observed sample did not satisfy H3-ACP-001.
5. The execution therefore receives `INCONCLUSIVE`, not `PASS`.
6. The result does not justify retroactive threshold, workload, exclusion-rule, or actuator changes.

The following conclusions are **not** justified:

1. `nice +10` has no causal effect under all environments or workloads.
2. H3 is falsified in the strong logical sense.
3. The 7/10 positive direction is sufficient evidence of causality.
4. The CI `failure` means the experiment infrastructure failed.
5. Any individual pair may be discarded because its numerical result is surprising.

## 7. Governance consequence

H3-ACP-001 remains **ADMITTED** and unchanged.

The registered arm remains unchanged.

Run `31929841805` is a completed, immutable execution record with a materialized raw artifact. It MUST NOT be rewritten or replaced by a later run.

Any future execution must receive a separate run ID and separate execution record. If a future methodological change is proposed after observing this result, it requires a new Proposal ID and admission cycle; the present result cannot be used to retroactively alter the criterion.

## 8. Review verdict

`H3 RUN = EXECUTED`

`H3 EVIDENCE = VALID / MATERIALIZED`

`H3-ACP-001 = NOT SATISFIED`

`H3 CAUSAL VERDICT = INCONCLUSIVE`

`H3 CAUSAL SUPPORT = NOT ESTABLISHED`

`H3 STRONG NULL/FALSIFICATION CLAIM = NOT ESTABLISHED`
