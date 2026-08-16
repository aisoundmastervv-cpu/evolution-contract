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

## 3. Primary endpoint

Primary endpoint: CPU time reported by each cgroup's `cpu.stat` (`usage_usec`) over the registered measurement interval.

The comparison is the intervention/control CPU-time allocation under the two registered actuator conditions.

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
- the execution commit and this registration commit are preserved in provenance.

## 5. Trial protocol

The execution harness must run the registered workload concurrently in control and intervention cgroups. The measurement window and trial count must be recorded by the execution implementation before the first outcome observation.

The same registered arm must be used for all repetitions. No mapping, workload, or primary endpoint may be changed after observing results.

## 6. Outcome classification

The final experiment must report exactly one of:

- `CAUSAL SUPPORT`
- `NULL-FALSIFICATION`
- `INCONCLUSIVE`

This registration itself is not evidence of H3 and does not authorize a causal conclusion.

## 7. Provenance

This file's commit SHA is the registration anchor for `mapping_id = A2-cgroup-linear-v1`. The execution evidence must reference this exact commit SHA and the exact executable commit used for the run.
