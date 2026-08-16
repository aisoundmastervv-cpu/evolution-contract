# Current Experimental State

## H2
- Status: **FROZEN**.
- Mathematical consistency: **CONFIRMED**.
- Regression evidence: **6/6 PASS** on diagnostic commit `ef43452a45e86ac6e398a0afe5c0af79fe732cce`.
- Causal validity: **NOT TESTED**.
- Empirical validity: **NOT TESTED**.

## H3
- Status: **ARM REGISTERED / CRITERION ADMITTED / RUN NOT EXECUTED**.
- EEC-003 execution gate: **CLOSED / PASS**.
- The repository's existing cloud-execution substrate remains infrastructure-only: it records execution identity, artifacts, machine state, recovery, and operational failures, but does not assign semantic meaning to resource usage or measure CPU cost.
- Therefore the existing cloud layer is **not** itself an H3 causal bridge.
- H3 direction remains an OS-level causal bridge, with Linux cgroup CPU controller as the primary design arm and Linux `nice` / scheduler priority as the fallback.
- A2 established that the cgroup CPU-controller capability is absent in the validated environment. No new A2 experiment or retry is authorized by this state.
- The registered executable fallback arm is `H3-OS-NICE-CAUSAL-v0.1`, documented in `docs/execution/h3-os-causal-execution-arm-v0.1.md`.
- Registered control condition: `nice 0`.
- Registered treatment condition: `nice +10`.
- The treatment parameter is fixed before execution and MUST NOT be changed after observing results.
- Mapping `efficiency -> actuator parameter` remains non-implicit; no outcome-derived parameter selection is permitted.
- `H3-ACP-001` is **ADMITTED** as the normative acceptance criterion for this execution arm.
- Registered primary observable: paired wall-clock runtime ratio `R_i = T_treatment,i / T_control,i`.
- Registered minimum sample: **10 valid paired trials**.
- Registered directional criterion: **at least 9/10 pairs with `R_i > 1`**.
- Registered effect floor: **median `R >= 1.10`**.
- No outlier deletion is permitted; invalidity must be established by protocol violation, not by an inconvenient numerical result.
- H3 causal run remains **NOT EXECUTED**.
- No H3 causal conclusion is established.

## H3 execution prerequisites
- The target workload is deterministic, CPU-bound, external to frozen H2 implementation/tests, and identified by exact workload revision `h3-cpu-workload-v0.1`.
- Control and treatment use the same workload and inputs.
- Niceness is independently observed at runtime through `/proc/self/stat` and verified against the registered condition.
- Scheduler contention is explicitly created by child workers launched at registered baseline `nice 0`; this is part of the workload runner, not an adaptive intervention.
- Environment identity is verified using the EEC-003 mechanism before execution.
- Raw control and treatment evidence are materialized with complete provenance and uploaded as an artifact.
- The machine-readable projection `config/h3-acp-001.env` is consumed by validation and execution workflow; divergence from the admitted criterion is a conformance failure.
- `src/bin/h3_runner.rs` implements the registered actuator verification and deterministic workload; it has a non-executing validation mode.
- `.github/workflows/h3-runner-validation.yml` validates the runner and environment contract without executing the causal workload.
- `.github/workflows/h3-causal-execution.yml` is manually gated and performs the registered 10-pair causal execution only when explicitly dispatched.

## Provenance incident containment
- PR #8 (`h3/design-requirements-v1`) was created from this repository but was not part of the canonical approved continuation and was closed without merge.
- Its artifact is non-canonical and must not be treated as approved H3 state.
- The incident demonstrated that conversation continuity alone was insufficient to prevent cross-context identity confusion.
- `.project/IDENTITY.md` is the mandatory repository-level identity anchor.

## Architecture
```text
IDENTITY → INTEGRITY → EXECUTION → EVIDENCE → HYPOTHESIS
    │
    └── canonical project identity + lineage verification
                         │
                         └── QUALITY (advisory)
```

The G-System/capability layer remains separate from SIGADEFA Ω.
