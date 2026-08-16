# Environment Contract v0.1

**Status:** implementation candidate; not yet a governance approval.

## Purpose

Define the minimum execution environment that can be reconstructed from canonical repository state. The workspace is derived state and is never a source of truth.

## Canonical inputs

The environment MUST be reconstructible from:

1. repository URL;
2. exact Git commit SHA;
3. this contract;
4. the bootstrap implementation referenced by this contract.

A branch name is not sufficient to identify an execution input.

## Required capabilities

The environment MUST provide:

- Git;
- a POSIX-compatible shell (`bash` for the reference implementation);
- SHA-256 hashing (`sha256sum` for the reference implementation);
- Rust stable toolchain with `cargo` and `rustc`;
- `cargo fmt` for formatting validation;
- `cargo test` for repository validation.

The reference implementation targets a Linux CI/runtime surface. Provider-specific infrastructure is outside this contract.

## Repository invariants

After bootstrap:

- the workspace MUST be a Git repository;
- `HEAD` MUST equal the requested commit SHA;
- the workspace MUST contain the repository's canonical files at that commit;
- no workspace-local path may participate in canonical identity;
- bootstrap MUST NOT mutate the remote repository.

## Environment identity

The reference implementation derives an environment identity from reproducible inputs:

- repository URL;
- exact commit SHA;
- Environment Contract content;
- bootstrap implementation content;
- `rustc --version`;
- `cargo --version`.

The absolute workspace path, hostname, process ID, and timestamp are descriptive execution metadata and MUST NOT affect `environment_id`.

## Readiness

The environment is `READY` only when the preflight checks pass. A failed preflight is fail-closed: execution MUST NOT proceed as though the environment were conformant.

## Re-bootstrap semantics

Destroying a workspace MUST NOT destroy project identity or evidence. A fresh workspace may be created from the same canonical commit and revalidated. Two successful re-bootstrap operations on the same execution surface and toolchain MUST produce the same `environment_id`.

## Out of scope

This contract does not define:

- persistent agent workspaces;
- cloud provider selection;
- deployment infrastructure;
- production authorization;
- evidence retention policy;
- application-level scientific validity.
