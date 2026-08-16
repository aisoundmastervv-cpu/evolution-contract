# H3 Order/Position Diagnostic — Run 31929841805 v0.1

## Diagnostic status

- Execution: `31929841805`
- Execution commit: `31ec257f07e4ec0013228f196f2b93a10e607ece`
- Environment ID: `769dd253bbe337d630018b1b6c09729399e57ad3312568054007c218a19a4b77`
- Arm: `H3-OS-NICE-CAUSAL-v0.1`
- Proposal: `H3-ACP-001`
- Raw artifact ID: `9258952230`
- Raw artifact SHA-256: `2abc6d7e099cb08537bb93083fa331567d98839c4aea7bd03163e5643b2ddbdb`
- Diagnostic status: **UNRESOLVED POSITION/TEMPORAL EFFECT**
- Governance status: **DIAGNOSTIC ONLY — NO CHANGE TO H3 VERDICT**

## 1. Purpose

This document records a post-execution diagnostic question identified from the already-frozen H3 evidence. It does not modify the execution record, raw artifact, registered arm, acceptance threshold, pair validity, or causal verdict.

The question is whether execution position within a pair, or a closely related temporal/runner-state effect, can explain part of the extreme observations in pairs 9 and 10.

## 2. Registered execution order

The execution workflow alternates order by pair:

- odd pairs: `control -> treatment`;
- even pairs: `treatment -> control`.

Therefore pair position is deliberately separated from treatment role across the ten registered pairs. This makes position diagnostically observable without constituting a protocol violation.

## 3. Exact reconstructed sequence

| Pair | First | First ms | Second | Second ms | Registered ratio T/C |
|---:|---|---:|---|---:|---:|
| 1 | control | 226.697 | treatment | 186.816 | 0.824078 |
| 2 | treatment | 188.830 | control | 186.808 | 1.010824 |
| 3 | control | 191.017 | treatment | 187.426 | 0.981201 |
| 4 | treatment | 188.090 | control | 186.906 | 1.006335 |
| 5 | control | 154.976 | treatment | 187.702 | 1.211168 |
| 6 | treatment | 188.195 | control | 187.002 | 1.006380 |
| 7 | control | 186.792 | treatment | 186.913 | 1.000648 |
| 8 | treatment | 186.874 | control | 185.991 | 1.004748 |
| 9 | control | 186.133 | treatment | 108.976 | 0.585474 |
| 10 | treatment | 188.302 | control | 109.023 | 1.727177 |

The sequence above is reconstructed directly from the immutable execution evidence reported by run `31929841805`.

## 4. Diagnostic observation

Pairs 9 and 10 form a distinctive adjacent pattern:

- pair 9: the **second** execution is treatment at `108.976 ms`;
- pair 10: the **second** execution is control at `109.023 ms`.

The two fast observations differ by only `0.047 ms`, while their roles are opposite. Thus the same approximately `109 ms` runtime occurs in the second position for both treatment and control.

Across all ten pairs, the second execution is faster than the first in 8/10 pairs. The mean first-position runtime is `188.5906 ms`; the mean second-position runtime is `171.3563 ms`. The mean difference is dominated by the two approximately `109 ms` observations. The median second-minus-first difference is approximately `-1.6075 ms`, so the overall position signal is not equivalent to a uniform large slowdown/speedup across all pairs.

For pairs 1–8 only, the mean first-position runtime is `188.933875 ms` and the mean second-position runtime is `186.9455 ms`, with median second/first ratio approximately `0.993683`. This shows that the extreme position effect is concentrated in the final adjacent pair block rather than being a simple monotonic first-versus-second effect throughout the run.

## 5. Interpretation

The evidence supports the following narrow statement:

> **Pairs 9 and 10 contain a strong unresolved temporal/positional signal: both second executions completed at approximately 109 ms while treatment/control role reversed.**

This is evidence that execution position or a time-dependent runner state may contribute to the observed variation.

It is **not** sufficient to establish a causal confound. The sample contains only ten pairs, the extreme signal is concentrated in two adjacent observations, and the existing protocol deliberately alternates treatment position.

The observation therefore remains a diagnostic hypothesis rather than a causal conclusion.

## 6. What this diagnostic does not do

This document does not:

- remove pairs 9 or 10;
- alter any raw evidence;
- alter the execution record;
- alter `H3-ACP-001`;
- alter the `9/10` positive-pair requirement;
- alter the `1.10` median-ratio requirement;
- change the execution verdict;
- convert `INCONCLUSIVE` into `FAILED`;
- establish that the runner contains a confound;
- authorize another H3 execution.

The historical execution remains exactly as recorded.

## 7. Follow-up boundary

Any attempt to resolve the diagnostic question experimentally must be treated as a separate registered study or execution with its own identity, protocol, evidence, and review. It must not be used to retroactively reinterpret or rewrite run `31929841805`.

A future diagnostic should specifically distinguish at least:

1. pair position (`first` vs `second`);
2. treatment role (`control` vs `treatment`);
3. pair index / temporal location;
4. repeated fast-runtime events;
5. environment/runner state if independently observable.

No such follow-up is authorized by this document.

## 8. Current conclusion

`RUN 31929841805 = FROZEN`

`RAW EVIDENCE = FROZEN`

`H3-ACP-001 = UNCHANGED`

`H3 VERDICT = INCONCLUSIVE`

`POSITION/TEMPORAL EFFECT = UNRESOLVED DIAGNOSTIC QUESTION`

`CAUSAL INTERPRETATION OF POSITION EFFECT = NOT ESTABLISHED`
