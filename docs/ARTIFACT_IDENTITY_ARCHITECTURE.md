# Artifact Identity Architecture

Status: DESIGN / IMPLEMENTATION BASELINE

## Purpose

This repository MUST never execute an experimental artifact merely because a file has the expected name or happens to compile.

Every executable experiment is bound to an immutable artifact identity before execution.

The incident that motivated this architecture was a mismatch between a frozen `src/lib.rs` discussed during an experiment and a legacy `src/lib.rs` inherited from `main`. The old workflow discovered the mismatch only after compilation. This architecture makes that state unreachable as an accepted execution path.

## Core invariant

> **No execution without identity. No hypothesis conclusion without execution evidence.**

The execution target is a tuple, not a directory:

```text
ArtifactIdentity =
    repository
  + immutable commit
  + artifact manifest
  + file hashes
  + toolchain identity
  + test command
```

A branch name such as `main` or `h2-fitness-hypothesis` is NOT an artifact identity. Branches may move.

## Required gates

```text
PROPOSAL
   |
   v
ARTIFACT FREEZE
   |
   +--> immutable commit recorded
   +--> manifest generated
   +--> file hashes recorded
   +--> scope recorded
   |
   v
IDENTITY GATE
   |
   +--> checkout exact commit
   +--> verify every manifest entry
   +--> verify no unmanifested execution inputs
   |
   +-- FAIL --> STOP
   |
   v
EXECUTION GATE
   |
   +--> run exactly the declared command
   +--> capture raw stdout/stderr and exit code
   +--> record runner/toolchain identity
   |
   +-- FAIL --> STOP / classify as execution failure
   |
   v
HYPOTHESIS GATE
   |
   +--> interpret only the captured execution evidence
   +--> classify CONFIRMED / REJECTED / INCONCLUSIVE
```

## Artifact manifest

The canonical manifest MUST be machine-readable and MUST contain, at minimum:

- repository full name;
- immutable commit SHA;
- experiment identifier;
- artifact version;
- exact relative paths in scope;
- SHA-256 for every file;
- legacy MD5 only when compatibility with an existing manifest is required;
- declared test command;
- Rust toolchain identifier;
- creation timestamp;
- status (`FROZEN`, `SUPERSEDED`, or `REJECTED`).

SHA-256 is the canonical content identity. MD5 is retained only as a compatibility/forensic field for the existing H2 manifest.

Example:

```yaml
schema: 1
experiment: SIGADEFA-H2
status: FROZEN
repository: aisoundmastervv-cpu/evolution-contract
commit: <immutable-commit-sha>
test_command: cargo test -- fitness::tests
toolchain: <rust-toolchain-lock>
files:
  - path: Cargo.toml
    sha256: <sha256>
  - path: src/lib.rs
    sha256: <sha256>
  - path: src/fitness.rs
    sha256: <sha256>
```

## Why the immutable commit is mandatory

A manifest attached only to a branch is insufficient. A branch can move between identity verification and execution.

The runner MUST therefore resolve the experiment to an exact commit SHA and checkout that SHA. The branch is metadata only.

If the manifest says commit `A`, execution at commit `B` is an **IDENTITY GATE FAIL**, even when every visible file currently looks plausible.

## No post-hoc identity discovery

The previous incident was effectively:

```text
run
  -> compiler failure
  -> inspect source
  -> discover wrong baseline
```

The new architecture requires:

```text
resolve commit
  -> verify manifest
  -> verify bytes
  -> only then run compiler
```

Therefore compiler output can never be used to establish artifact identity.

## No manual byte transport

Frozen artifacts MUST live in Git history or a GitHub Actions artifact/release associated with an immutable run/commit.

Copying source through chat, manual reconstruction, or retyping is not an accepted production path.

If an artifact cannot be located by immutable Git identity, the system MUST classify it as `UNRECOVERABLE_ARTIFACT`, not silently reconstruct it.

## Negative control preservation

Rejected hypotheses remain immutable artifacts. They are not deleted or rewritten to make later experiments look cleaner.

For H2, for example:

```text
H1_REJECTED.md  -> negative control
H2_ACCEPTED.md  -> mathematical/architectural hypothesis
H3_PENDING.md   -> pending question
```

Their identities belong in the experiment manifest.

## Scope lock

The identity manifest also defines the experiment scope. A file outside the manifest cannot silently become part of the experiment.

Changes to any of these invalidate the frozen identity:

- source files;
- Cargo manifests/lockfiles;
- test files;
- configuration affecting execution;
- toolchain configuration;
- declared experiment metadata.

## Evidence record

Every execution MUST produce a machine-readable evidence record containing:

```text
experiment_id
artifact_commit
manifest_hash
verified_file_count
identity_result
command
runner
rustc_version
cargo_version
exit_code
raw_output
execution_result
```

The evidence record is append-only. A later interpretation does not overwrite raw execution evidence.

## Recovery rule

If a frozen artifact is missing from the working tree:

1. search Git history by immutable commit/blob identity;
2. search Actions artifacts associated with that commit/run;
3. search release assets associated with that immutable version;
4. if not found, stop.

Never substitute `main`, a similarly named branch, or a reconstructed file without creating a new experiment identity.

## Architectural consequence

The repository becomes the source of truth for executable artifacts. The chat is a design and reasoning interface, not an artifact transport layer.

That separation is intentional:

```text
Chat / reasoning
      |
      | proposes and records
      v
GitHub artifact identity
      |
      | immutable verification
      v
GitHub Actions execution
      |
      | raw evidence
      v
Hypothesis conclusion
```

This architecture directly prevents the incident in which a frozen `lib.rs` and the GitHub `main` version were silently treated as the same artifact.
