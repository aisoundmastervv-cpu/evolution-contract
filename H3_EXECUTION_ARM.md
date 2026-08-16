# H3 Execution Arm — A2-cgroup-linear-v1

**Status:** PRE-REGISTERED EXECUTION ARM

**Protocol:** H3 Design Requirements, amended version
**Bridge:** Linux cgroup-v2 CPU controller
**Mapping ID:** A2-cgroup-linear-v1
**Gene domain:** `efficiency ∈ [0,255]`
**Actuator:** sibling cgroup-v2 `cpu.weight`
**Registration rule:** fixed before first outcome observation; no outcome-dependent remapping

## 1. Mapping

For a gene value `e ∈ [0,255]`:

`cpu.weight(e) = 100 + floor(9900 * e / 255)`

Therefore:

- `e = 0` → `cpu.weight = 100`
- `e = 255` → `cpu.weight = 10000`

The mapping is deterministic and is not derived from CPU measurements.

## 2. Experimental comparison

The primary comparison uses two sibling cgroup-v2 groups on the same execution environment:

- **control:** fixed `cpu.weight = 100`
- **intervention:** registered mapping above

Both groups execute the identical frozen workload concurrently for the same registered measurement window. The workload receives no `efficiency` value and does not alter its computational effort based on the gene.

The intervention is evaluated at pre-registered gene values `0` and `255`, producing actuator weights `100` and `10000` respectively.

The execution uses CPU affinity to the same logical CPU for control and intervention. The selected CPU and runner environment are recorded as evidence.

## 3. Primary endpoint

Primary endpoint: CPU time reported by each cgroup's `cpu.stat` (`usage_usec`) over the registered measurement interval.

For each trial, the primary comparison statistic is:

`R = intervention_cpu_usec / control_cpu_usec`

The registered effect criterion is:

- for `efficiency = 0`, the median `R` across valid trials must be within `[0.5, 2.0]`;
- for `efficiency = 255`, the median `R` must be at least `2.0` times the median `R` at `efficiency = 0`;
- all five trials for each gene value must be valid for a causal-support determination.

Wall-clock duration is secondary.

## 4. Validity requirements

A trial is valid only if:

- both sibling cgroups exist and are configured as registered;
- both workloads are the same executable/version and input;
- both workloads pass the correctness/exit-status check;
- both processes remain attached to their intended cgroups for the measurement interval;
- `cpu.stat` is readable before and after execution;
- no forbidden in-process efficiency-dependent throttling or workload scaling occurs;
- the actual `cpu.weight` values are recorded before outcome observation;
- the execution commit and this registration commit are preserved in provenance;
- the same CPU affinity is applied to both compared workloads;
- the fixed measurement window is exactly 5 seconds per trial, apart from unavoidable scheduling/termination overhead recorded in evidence.

A trial with failed cgroup setup, failed attachment, unreadable counters, failed workload correctness, or material environmental contamination is invalid and cannot contribute to causal support.

## 5. Trial protocol

The execution harness must run the registered workload concurrently in control and intervention cgroups.

There are five registered trials at `efficiency = 0` followed by five registered trials at `efficiency = 255`. The trial order is fixed and must not be changed after observation.

Before each trial, fresh sibling cgroups are created. Both workloads are started, stopped from running briefly for cgroup attachment, then released simultaneously. CPU accounting is sampled immediately before release and after the fixed 5-second measurement window. Both workloads are then terminated cleanly and their correctness/exit status is recorded.

The same registered arm must be used for all repetitions. No mapping, workload, primary endpoint, trial count, or effect criterion may be changed after observing results.

## 6. Outcome classification

The final experiment must report exactly one of:

- `CAUSAL SUPPORT`
- `NULL-FALSIFICATION`
- `INCONCLUSIVE`

`CAUSAL SUPPORT` requires all validity requirements to pass and the registered effect criterion to pass.

`NULL-FALSIFICATION` requires all validity requirements to pass but the registered effect criterion to fail under the complete five-trial replication.

Any invalid execution, insufficient sensitivity, uncontrolled contamination, or inability to establish the registered intervention requires `INCONCLUSIVE`.

This registration itself is not evidence of H3 and does not authorize a causal conclusion.

## 7. Provenance

This file's commit SHA is the registration anchor for `mapping_id = A2-cgroup-linear-v1`. The execution evidence must reference this exact registration commit SHA and the exact executable commit used for the run.
