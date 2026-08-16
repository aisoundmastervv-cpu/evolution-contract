# Current Experimental State

## H2
- Status: **FROZEN**.
- Mathematical consistency: **CONFIRMED**.
- Regression evidence: **6/6 PASS** on diagnostic commit `ef43452a45e86ac6e398a0afe5c0af79fe732cce`.
- Causal validity: **NOT TESTED**.
- Empirical validity: **NOT TESTED**.

## H3
- Status: **ARM REGISTERED / RUN BLOCKED**.
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
- H3 causal run remains **NOT EXECUTED**.
- No H3 causal conclusion is established.

## H3 execution prerequisites
- The target workload must be deterministic, CPU-bound, external to frozen H2 implementation/tests, and identified by exact repository path and revision.
- Control and treatment must use the same workload and inputs.
- Niceness must be independently observed at runtime.
- Environment identity must be verified using the EEC-003 mechanism before and during the registered execution protocol.
- Raw control and treatment evidence must be materialized with complete provenance.
- A canonical pre-registered H3 acceptance threshold has not been located in the verified repository state. It MUST be recovered from canonical Git history, or explicitly registered before execution; it MUST NOT be invented or retuned after results.
- If the acceptance threshold cannot be established before execution, H3 remains blocked.

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
