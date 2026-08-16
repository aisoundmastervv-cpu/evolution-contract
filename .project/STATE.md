# Current Experimental State

## H2
- Status: **FROZEN**.
- Mathematical consistency: **CONFIRMED**.
- Regression evidence: **6/6 PASS** on diagnostic commit `ef43452a45e86ac6e398a0afe5c0af79fe732cce`.
- Causal validity: **NOT TESTED**.
- Empirical validity: **NOT TESTED**.

## H3
- Status: **BLOCKED**.
- The evolution crate has no independent execution mechanism, workload runner, or CPU measurement path.
- The repository's existing cloud-execution substrate is infrastructure-only: it records execution identity, artifacts, machine state, recovery, and operational failures, but does not assign semantic meaning to resource usage or measure CPU cost.
- Therefore the existing cloud layer is **not** an H3 causal bridge.
- Do not add an execution engine to the H2 crate merely to force H3 forward.
- H3 direction is now fixed at the design level: OS-level causal bridge, primary Linux cgroup CPU controller, fallback Linux `nice` / scheduler priority.
- Mapping `efficiency -> actuator parameter` remains intentionally unspecified until a specific execution arm is pre-registered independently of observed CPU outcomes.
- No H3 implementation or causal conclusion is authorized by this state record.

## Provenance incident containment
- PR #8 (`h3/design-requirements-v1`) was created from this repository but was not part of the canonical approved continuation and was closed without merge.
- Its artifact is non-canonical and must not be treated as approved H3 state.
- The incident demonstrated that conversation continuity alone was insufficient to prevent cross-context identity confusion.
- `.project/IDENTITY.md` is now the mandatory repository-level identity anchor.

## Next legitimate H3 step
- Read-only recovery and approval of the H3 design contract may proceed only after repository identity and branch lineage are verified against `.project/IDENTITY.md`.
- Implementation requires an approved design contract and must remain outside frozen H2 implementation/tests.

## Architecture
```text
IDENTITY → INTEGRITY → EXECUTION → EVIDENCE → HYPOTHESIS
    │
    └── canonical project identity + lineage verification
                         │
                         └── QUALITY (advisory)
```

The G-System/capability layer remains separate from SIGADEFA Ω.
