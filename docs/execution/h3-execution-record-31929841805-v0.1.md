# H3 Causal Execution Record — Run 31929841805 v0.1

## Record status

- Record type: immutable execution record
- Run ID: `31929841805`
- Workflow: `H3 Causal Execution` (`h3-causal-execution.yml`)
- Branch: `h3/execution-arm-registration-v1`
- Execution commit: `31ec257f07e4ec0013228f196f2b93a10e607ece`
- Execution arm: `H3-OS-NICE-CAUSAL-v0.1`
- Acceptance proposal: `H3-ACP-001`
- Execution conclusion: **INCONCLUSIVE**
- Workflow conclusion: **FAILURE** because acceptance evaluation was not satisfied

This record preserves the execution as observed. It does not modify the registered arm, acceptance criterion, workload, threshold, or raw evidence.

## Authorization and protocol boundary

- H3-ACP-001: **ADMITTED** before this execution.
- Registered arm: **VALID** before this execution.
- Protocol integration fix: `31ec257f...`, aligning `bootstrap_sha256` field naming between the workflow and environment gate.
- Environment preflight: **READY**.
- Runner validation: **PASS**.
- Execution therefore crossed the infrastructure/preflight boundary and materialized the registered experiment.

## Environment provenance

- Environment ID: `769dd253bbe337d630018b1b6c09729399e57ad3312568054007c218a19a4b77`
- Repository: `aisoundmastervv-cpu/evolution-contract`
- Execution commit: `31ec257f07e4ec0013228f196f2b93a10e607ece`
- Workload revision: `h3-cpu-workload-v0.1`
- Work units: `50000000`

## Registered intervention

- Actuator: Linux process niceness (`nice`)
- Control: `nice 0`
- Treatment: `nice +10`
- Intervention was held fixed; no post-result tuning occurred.

## Trial completeness

- Valid paired trials: **10/10**
- Invalid trials: **0**
- Raw trial records: **complete**
- Actuator provenance: independently observed in raw records
- Workload checksum: identical across paired control/treatment records

## Primary acceptance observable

`R_i = T_treatment,i / T_control,i`

Registered criterion from H3-ACP-001:

1. 10 valid paired trials;
2. at least 9/10 ratios `R_i > 1`;
3. median ratio `R >= 1.10`.

## Observed result

| Pair | Control ms | Treatment ms | Ratio |
|---:|---:|---:|---:|
| 1 | 226.697 | 186.816 | 0.824078 |
| 2 | 186.808 | 188.830 | 1.010824 |
| 3 | 191.017 | 187.426 | 0.981201 |
| 4 | 186.906 | 188.090 | 1.006335 |
| 5 | 154.976 | 187.702 | 1.211168 |
| 6 | 187.002 | 188.195 | 1.006380 |
| 7 | 186.792 | 186.913 | 1.000648 |
| 8 | 185.991 | 186.874 | 1.004748 |
| 9 | 186.133 | 108.976 | 0.585474 |
| 10 | 109.023 | 188.302 | 1.727177 |

Summary:

- Positive pairs (`R > 1`): **7/10**
- Median ratio: **1.0055411383275983**
- Directional criterion: **NOT SATISFIED** (`7 < 9`)
- Effect-floor criterion: **NOT SATISFIED** (`1.00554 < 1.10`)
- Registered verdict: **INCONCLUSIVE**

## Raw artifact

- Artifact ID: `9258952230`
- Artifact name: `h3-os-nice-causal-evidence-31ec257f07e4ec0013228f196f2b93a10e607ece`
- Artifact size: `15364` bytes
- Artifact SHA-256: `2abc6d7e099cb08537bb93083fa331567d98839c4aea7bd03163e5643b2ddbdb`
- Evidence schema: `h3-verdict-v0.1` plus `h3-trial-v0.1` records
- Artifact contents include `h3-verdict.json`, control/treatment trial records, and launcher status records for all 10 pairs.

## Causal interpretation boundary

This execution is **valid experimental evidence**, but it does **not** satisfy the pre-registered causal acceptance criterion. Therefore it MUST NOT be labeled `PASS` or `causal support`.

It also MUST NOT be labeled `INVALID`, `NON-EVIDENTIARY`, or `INFRASTRUCTURE FAILURE`: the registered workload and intervention executed, provenance was materialized, and 10 valid paired trials were obtained.

The appropriate procedural status is **INCONCLUSIVE / ACCEPTANCE CRITERION NOT MET**.

No threshold, actuator parameter, workload, exclusion rule, or trial result may be changed retroactively on the basis of this record.

## Integrity statement

This record describes Run `31929841805` at execution commit `31ec257f...` and preserves the artifact digest exactly as materialized by GitHub Actions. Any future execution MUST receive a separate execution record and MUST NOT overwrite this record.
