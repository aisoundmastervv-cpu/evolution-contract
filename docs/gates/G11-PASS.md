# G11 — Semantic Correspondence Gate

Status: **PASS**

## Frozen semantic inputs

- Contract: `docs/spec/evolution-application-contract-v1.0.md`
- Contract status: **FROZEN**
- G11.1 baseline: `docs/G11.1-semantic-baseline.md`
- G11.1 status: **FROZEN**
- Test plan: `docs/G11-test-plan.md`
- Test plan status: **ACCEPTED**

## Machine execution

- Workflow: `G11 Harness`
- Workflow run ID: `31919382047`
- Execution event: `push`
- Execution branch: `validation/real-two-fix`
- Execution commit: `e1917e9a9801aeaeeb50699cdb87ed488afe73a1`
- Harness command: `cargo test --lib g11_harness -- --nocapture`
- Exit status: `0`
- Cases executed: `10`
- Cases passed: `10`
- Cases failed: `0`

## Oracle coverage

- O1-A — capability safety: covered by TC-N01, TC-P01, TC-B04; PASS.
- O1-B — structural atomicity: covered by TC-N02, TC-B01; PASS.
- O1-C — build-before-commit atomicity: covered by TC-P02, TC-N03, TC-B02; PASS.
- O1-D — temporal tolerance: covered by TC-N04, TC-P03; PASS.

The previously rejected TC-B03 is intentionally excluded. It was identified as an invalid test construction because sequential applications changed generation state and therefore conflated temporal tolerance with generation-mismatch validation. Its removal did not modify C1, O1, or the frozen Contract.

## Evidence artifact

- Artifact: `g11-raw-evidence-e1917e9a9801aeaeeb50699cdb87ed488afe73a1`
- Artifact ID: `9255874930`
- Artifact SHA-256: `cccd8f88c9c1132b0dc78b6d2f9d1979af0b781a348e2bd8a293b21188c9720e`
- Artifact retention: GitHub Actions artifact; not expired at time of recording.

## Evidence sufficiency verdict

The raw machine evidence, provenance, exact execution commit, accepted test plan, and frozen semantic baseline were reviewed together. The execution covers every frozen oracle predicate O1-A through O1-D and produced zero semantic test failures.

Therefore:

> **G11 = PASS**

This record is an immutable governance record of the verdict. It records the evidence identifiers and does not alter the frozen Claim, Oracle, Contract, or implementation semantics.
