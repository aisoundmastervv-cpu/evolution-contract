# H3 OS Causal Execution Arm v0.1

## Status

- Execution Arm: **REGISTERED**
- H3 causal run: **NOT EXECUTED**
- Registration basis: canonical repository state after EEC-003 closure
- Registration commit base: `ebf805523806720f090f76a818f924051ad58cf3`
- H2: **FROZEN**

## Scope

This document registers an H3 causal execution arm without modifying the H2 implementation, H2 tests, A1 evidence, A2 evidence, or EEC-003 evidence.

The arm is an OS-level causal bridge. It assigns an explicit intervention to an independently identifiable scheduler control. It does **not** infer an actuator parameter from an observed outcome.

## Execution Arm

### Arm ID

`H3-OS-NICE-CAUSAL-v0.1`

### Actuator

Linux process niceness (`nice`).

### Registered intervention

- Control condition: process niceness `0`.
- Treatment condition: process niceness `+10`.
- The treatment value is fixed before execution and MUST NOT be changed after observing any result.
- The intervention MUST be applied to the registered target workload process only.
- No efficiency measurement may be used to select or modify the treatment value.

### Why this arm is executable

The canonical state identifies Linux `nice` / scheduler priority as the H3 fallback when the primary cgroup CPU controller is unavailable. A2 established that the cgroup CPU controller capability is absent in the validated environment; therefore this registration does not silently substitute an unavailable cgroup actuator.

The cgroup arm remains a design-level primary mechanism and is **not** activated by this registration.

## Causal hypothesis

For the same registered workload, changing only the independently observable OS scheduler priority from the registered control condition (`nice 0`) to the registered treatment condition (`nice +10`) changes the workload's execution outcome in the direction predicted by reduced scheduling priority.

This is a causal test of the registered OS intervention. It is not a claim that any observed CPU-efficiency metric is itself a cause.

## Target workload

The H3 run MUST use one deterministic, CPU-bound workload definition held constant between control and treatment.

The workload implementation MUST be external to the frozen H2 implementation and MUST be identified by repository path and commit SHA in the execution evidence.

No H2 function, test, or internal evolution mechanism may be used as an implicit actuator or causal bridge.

## Observable

The run MUST record, for both control and treatment:

- wall-clock execution time;
- process CPU time;
- user/system CPU time where available;
- exit status;
- workload identity and input parameters;
- process niceness as observed independently from the launcher;
- environment identity established by EEC-003;
- repository HEAD and execution-arm ID.

The observable is the measured workload execution outcome. CPU measurements are observations, not the intervention itself.

## Control / treatment protocol

1. Verify environment identity before launching the workload.
2. Verify the target process starts with `nice 0` for the control run.
3. Execute the fixed workload without changing workload inputs.
4. Record raw control evidence.
5. Verify environment identity again before treatment.
6. Launch the same workload with `nice +10`.
7. Independently verify the target process niceness.
8. Execute the identical workload and inputs.
9. Record raw treatment evidence.
10. Preserve both raw records without post-hoc filtering.

The control and treatment runs MUST be separate executions with the same registered workload definition and environment contract.

## Provenance requirements

Every H3 result MUST identify:

- repository: `aisoundmastervv-cpu/evolution-contract`;
- exact execution commit SHA;
- execution arm: `H3-OS-NICE-CAUSAL-v0.1`;
- actuator: Linux `nice`;
- control parameter: `0`;
- treatment parameter: `+10`;
- workload path and workload revision;
- environment identity;
- workflow/run ID;
- raw evidence artifact and SHA-256.

## Invariants

- H2 remains frozen.
- H2 implementation and tests MUST NOT change.
- A1, A2, and EEC-003 evidence MUST NOT be rewritten.
- The cgroup actuator MUST NOT be substituted implicitly for this arm.
- The treatment parameter MUST NOT be tuned after observing results.
- The workload MUST remain identical between control and treatment.
- No causal interpretation may be made before raw evidence is materialized.
- No post-hoc selection of successful runs is permitted.

## Acceptance threshold

The canonical repository state available at registration time does not contain a machine-readable or otherwise verifiable H3 acceptance threshold. This registration therefore does **not** invent, alter, or infer one from results.

Before the first H3 causal run, the pre-existing acceptance threshold, if any, MUST be recovered from the canonical Git history and incorporated into the execution protocol without modification. If no canonical threshold exists, H3 execution MUST remain blocked until a threshold is explicitly registered.

This is a governance requirement, not an experimental result.

## Stop conditions

The H3 run MUST stop and be classified as non-evidentiary if any of the following occurs:

- environment identity mismatch;
- actuator application cannot be independently verified;
- the target workload differs between control and treatment;
- the registered niceness value cannot be established independently;
- raw evidence is missing or incomplete;
- provenance is incomplete;
- unauthorized mutation occurs;
- workload execution leaves the registered scope;
- the acceptance threshold is unavailable before execution.

## Failure semantics

- **Experiment failure:** the registered workload/intervention executed but the execution protocol's required observable was not obtained.
- **Environment failure:** execution identity or environment contract was invalidated.
- **Infrastructure failure:** workflow/runner failure prevented the experiment from materializing.
- **Inconclusive:** valid raw evidence exists, but the pre-registered acceptance criterion is not satisfied or the evidence cannot discriminate the hypothesis.
- **Causal support:** only a valid control/treatment execution with complete provenance and a satisfied pre-registered acceptance criterion may receive this status.

## Authorization state

`H3-OS-NICE-CAUSAL-v0.1 = REGISTERED`

`H3 RUN = NOT EXECUTED`

`H3 CAUSAL EVIDENCE = NONE`
