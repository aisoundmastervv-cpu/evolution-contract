# Cloud Production Packaging v0.1

**Status:** DEFINED / IMPLEMENTATION PENDING  
**Production deployment authorization:** NOT GRANTED

## 1. Purpose

This record defines the production packaging boundary required to turn the approved provider-neutral cloud execution implementation into a concrete, immutable deployment artifact.

It does not select a cloud provider, create a production environment, authorize deployment, or claim production readiness.

## 2. Problem being resolved

The current cloud implementation is a provider-neutral reference/conformance substrate. Its implementation plan explicitly excludes provider selection, cloud accounts, orchestration, and production deployment.

The current conformance workflow executes a test binary on a GitHub Actions runner and produces conformance evidence. That execution environment and evidence artifact are not production deployment artifacts.

Therefore a distinct production packaging layer is required before a Deployment Candidate can be fully identified.

## 3. Production execution unit

The production execution unit MUST be explicitly defined before production deployment authorization.

For this project, the initial packaging target is defined as:

> **A reproducible, immutable build artifact containing the approved cloud execution implementation and its explicitly declared production runtime entrypoint, together with the exact metadata required to reconstruct and verify that artifact.**

The runtime entrypoint is currently **PENDING** because the repository currently exposes a Rust library crate rather than a declared production service/binary entrypoint.

No test binary, conformance harness, GitHub Actions job, or local/reference provider is implicitly promoted to production execution status.

## 4. Packaging contract

A production packaging implementation MUST establish:

1. exact source revision;
2. declared production entrypoint;
3. reproducible build procedure;
4. immutable artifact identity;
5. artifact SHA-256 digest;
6. build provenance sufficient to reconstruct the artifact;
7. declared runtime dependencies;
8. declared configuration boundary;
9. declared runtime interface;
10. separation from conformance-only tooling.

## 5. Current repository boundary

The repository currently defines `evolution-contract` as a Rust library crate with `src/lib.rs` as its library path.

The existing cloud conformance workflow compiles `src/cloud_execution.rs` as a test binary using `rustc --edition 2021 --test` and executes that binary for conformance evidence.

These facts establish the current implementation boundary but do not establish a production execution unit.

## 6. Required production entrypoint

Before an artifact can become a Deployment Candidate, the implementation MUST identify exactly one production execution contract for the candidate, or explicitly define a bounded set of entrypoints if the architecture requires more than one process.

The entrypoint MUST be:

- version-controlled;
- reproducibly buildable;
- independently identifiable;
- distinguishable from test/conformance entrypoints;
- compatible with the approved cloud architecture;
- operationally invocable without changing semantic authority.

Current status:

```text
Production entrypoint: PENDING
```

## 7. Build method

The production build method MUST be deterministic or otherwise reproducible to the extent required to verify that the artifact corresponds to the declared source revision.

At minimum, the build record MUST identify:

```text
Source revision:       PENDING
Toolchain identity:    PENDING
Build command:         PENDING
Build inputs:          PENDING
Artifact path/type:    PENDING
Artifact SHA-256:      PENDING
```

The existing conformance command:

```text
rustc --edition 2021 --test src/cloud_execution.rs
```

MUST NOT be treated as the production build command merely because it produces an executable.

## 8. Artifact identity

A production artifact MUST be immutable and independently verifiable.

The candidate MUST record at least:

```text
Artifact name:         PENDING
Artifact type:         PENDING
Artifact version:      PENDING
Artifact SHA-256:      PENDING
Source revision:       PENDING
Build provenance:      PENDING
```

A mutable branch, workflow run, runner workspace, or source tree is not an immutable artifact identity.

## 9. Configuration boundary

Production configuration MUST be separated from the artifact unless configuration is intentionally compiled into the artifact and explicitly included in its identity.

The packaging contract MUST identify:

- required runtime configuration;
- configuration schema/version;
- configuration digest mechanism;
- secrets boundary;
- provider configuration boundary;
- operational defaults.

Configuration MUST NOT silently change semantic Contract, State Model, executor, verdict, or predicate behavior.

## 10. Conformance boundary

Conformance tooling remains separate from production packaging.

The following are evidence mechanisms, not production packaging inputs unless explicitly reclassified by a later approved decision:

- GitHub Actions conformance runner;
- conformance test binary;
- raw test output;
- conformance evidence archive;
- formatter/trigger mechanisms used solely to obtain evidence.

The production artifact MUST NOT depend on temporary evidence-generation mechanisms.

## 11. Provider boundary

This packaging record intentionally does not select AWS, GCP, Azure, Kubernetes, Terraform, or another provider.

Provider selection occurs only after the production execution unit and immutable artifact contract are defined.

Therefore:

```text
Production artifact
        ↓
Provider selection
        ↓
Target environment
```

not:

```text
Provider selection
        ↓
Invent a production artifact around it
```

## 12. Semantic boundary

Production packaging is operational infrastructure.

It MUST NOT redefine or modify:

- Contract semantics;
- C-LG/O-LG;
- Test Plan semantics;
- Validation Machine State Model;
- Reference Executor semantics;
- verdict semantics;
- semantic predicates.

Any required semantic change is outside this packaging record and requires its own governance path.

## 13. Required packaging evidence

Before the Deployment Candidate can move from `IDENTIFICATION PENDING` to `IDENTIFIED`, packaging evidence MUST establish:

1. production execution unit identity;
2. successful reproducible build;
3. immutable artifact creation;
4. artifact digest;
5. source-to-artifact binding;
6. runtime entrypoint verification;
7. separation from conformance-only tooling;
8. declared configuration boundary.

## 14. State

Current packaging state:

```text
Production execution unit       PARTIALLY DEFINED
Production entrypoint           PENDING
Build method                     PENDING
Immutable artifact               PENDING
Artifact SHA-256                 PENDING
Configuration contract           PENDING
Provider                         NOT SELECTED
Target environment               NOT SELECTED
Production authorization         NOT GRANTED
```

## 15. Next transition

The next implementation action is to define the production execution entrypoint and build contract in the repository.

Only after that implementation exists and produces a verifiable immutable artifact may the Deployment Candidate record be updated with the artifact identity.

The next governance review remains:

**Cloud Production Readiness Review v0.1**

It MUST NOT be executed until the Deployment Candidate identity requirements are satisfied.

## 16. Governance invariant

> A build artifact becomes a production deployment candidate only when its execution unit, source revision, build provenance, immutable identity, configuration boundary, and target environment can be explicitly bound and independently verified.

---

**Record:** Cloud Production Packaging v0.1  
**Status:** DEFINED / IMPLEMENTATION PENDING  
**Production authorization:** NOT GRANTED  
**Decision date:** 2026-08-16
