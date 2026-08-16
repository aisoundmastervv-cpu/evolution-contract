# H3 Execution Arm — A1-nice-v1

**Status:** PRE-REGISTERED ALTERNATE EXECUTION ARM

**Protocol:** H3 Design Requirements, amended version
**Bridge:** Linux `nice` / scheduler priority
**Mapping ID:** A1-nice-v1
**Gene domain:** `efficiency ∈ [0,255]`
**Actuator:** process nice value
**Reason for activation:** primary A2 cgroup arm could not write `cgroup.subtree_control` in the approved GitHub-hosted runner; this alternate arm is registered before any causal outcome observation.

## 1. Mapping

For a gene value `e ∈ [0,255]`:

`nice(e) = 19 - floor(19 * e / 255)`

Therefore:

- `e = 0` → `nice = 19`
- `e = 255` → `nice = 0`

The mapping is deterministic and is not derived from CPU measurements.

## 2. Experimental comparison

The control process runs at fixed `nice = 0`.

The intervention process runs with the registered mapping above. The intervention is evaluated at pre-registered gene values `0` and `255`.

Both processes execute the identical frozen CPU-demand workload concurrently on the same logical CPU. The workload receives no `efficiency` value and does not alter its computational effort based on the gene.

## 3. Primary endpoint

Primary endpoint: process CPU time obtained independently from Linux `/proc/<pid>/stat`, using the registered user-time and system-time counters.

For each trial:

`R = intervention_cpu_time / control_cpu_time`

Registered effect criterion:

- for `efficiency = 0`, median `R` across valid trials must be `<= 0.5`;
- for `efficiency = 255`, median `R` across valid trials must be `>= 2.0 * median_R(efficiency=0)`;
- all five trials for each gene value must be valid for causal support.

Wall-clock time is secondary.

## 4. Trial protocol

Five trials at `efficiency = 0` are followed by five trials at `efficiency = 255`.

Each trial uses a fresh pair of processes, the same workload executable/version, the same CPU affinity, and a fixed 5-second measurement window. Processes are stopped immediately after spawn, assigned their registered scheduling priorities, then released together. CPU counters are sampled immediately before release and after the 5-second window. Both processes are then terminated cleanly and their exit status is recorded.

The order and trial count are fixed before observation and must not be changed after results are seen.

## 5. Validity requirements

A trial is valid only if:

- both processes start successfully;
- the actual nice values match the registered control/intervention values;
- both processes use the same workload and CPU affinity;
- both `/proc/<pid>/stat` counters are readable before and after the measurement window;
- both workloads terminate cleanly with the registered exit status;
- no efficiency-dependent workload modification or in-process throttling occurs;
- the execution and registration commits are preserved in provenance.

Any failure to establish the registered intervention or independent measurement makes the trial invalid and the complete experiment `INCONCLUSIVE` unless the registered protocol explicitly permits exclusion.

## 6. Outcome classification

The final experiment reports exactly one of:

- `CAUSAL SUPPORT`
- `NULL-FALSIFICATION`
- `INCONCLUSIVE`

`CAUSAL SUPPORT` requires all ten trials to be valid and the registered effect criterion to pass.

`NULL-FALSIFICATION` requires all ten trials to be valid and the registered effect criterion to fail.

Infrastructure failure, measurement failure, contamination, or inability to establish the intervention requires `INCONCLUSIVE`.

## 7. Provenance

This file's commit SHA is the registration anchor for `mapping_id = A1-nice-v1`. Execution evidence must reference this exact registration commit SHA and the exact executable commit used for the run.
