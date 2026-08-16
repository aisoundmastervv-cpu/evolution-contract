# SIGADEFA Ω Evolution Application Contract v1.0

Status: **FROZEN**

## Normative status

This document is the canonical, design-level source of truth for the Evolution Application Contract v1.0.

It was reconstructed from the previously saved design artifact **“Evolution Application Contract v1.0 — reference skeleton”**. The recovery preserves the semantic rules of that artifact while removing implementation-specific code, current test results, and observations about the current Rust implementation.

The current implementation is not a source for this document. It is an implementation to be evaluated against this contract.

## Application pipeline

An evolution application follows these semantic stages, in order:

**Validate → Resolve Stale → Build → Commit → Audit**

The stages have distinct mutation boundaries. Validation, stale resolution, and build operate without committing evolution-state changes. Commit is the only stage that applies the accepted transition. Audit records the application outcome according to the contract.

## 1. Protection and capability safety

Processes have a protection level. The protection domain distinguishes at least:

- `Immune`
- `Protected`
- `Evolvable`
- `Sandboxed`

Only processes whose protection level is `Evolvable` or `Sandboxed` may obtain the capability required to participate in an evolution request.

The capability is a witness of having passed the protection boundary. A caller must not be able to manufacture an equivalent capability for an `Immune` or `Protected` process through the normal capability-construction boundary.

**Invariant:** an evolution request cannot authorize an `Immune` or `Protected` process merely by supplying its identifier.

## 2. Evolution plan

An evolution plan contains:

- the generation against which the plan was produced;
- the evaluation cycle at which it was produced;
- zero or more death requests;
- zero or more branch requests.

A death request identifies an evolution-capable process and a death reason.

A branch request identifies an evolution-capable parent and a mutation rate.

The population policy supplies structural bounds including:

- reproduction budget;
- maximum offspring per parent;
- minimum population;
- maximum population where applicable.

## 3. Structural validation

Structural validation occurs before any evolution-state mutation is committed.

The following conditions are structural violations:

- duplicate death requests for the same process;
- a conflicting death/branch request for the same process;
- reproduction budget exceeded;
- per-parent offspring cap exceeded;
- mutation rate outside the valid finite range `[0.0, 1.0]`;
- population floor would be violated;
- plan generation does not match the current population generation.

A structurally invalid plan is rejected as a whole.

**Structural atomicity invariant:** structural rejection produces no partial evolution-state mutation.

## 4. Temporal tolerance / stale requests

A plan may contain requests that were valid when the plan was produced but whose referenced process is no longer present when the plan is applied.

Staleness is resolved after structural validation and before build.

A stale death request or stale branch request is individually tolerated: it is skipped rather than turning the entire application into a structural failure. Independent requests that remain valid are still eligible for application.

**Temporal tolerance invariant:** staleness of one request does not invalidate unrelated valid requests in the same application.

Stale queue entries are likewise tolerated by lazy invalidation: a scheduler may encounter a reference to a process that has already disappeared and skip that entry rather than treating it as a fatal scheduler error.

## 5. Build phase

For each valid branch request, child construction occurs before commit.

Child construction may fail because of conditions including:

- PID allocation/collision failure;
- resource unavailability;
- invalid mutation;
- child-construction failure;
- allocation failure.

The build phase produces pending child results. Pending results are not yet committed to the active evolution state.

All commit-time collision conditions that can be determined from the pending results must be resolved before commit begins.

**Build-then-commit atomicity invariant:** a failed build does not partially commit the resulting evolution. In particular, a build failure does not commit deaths, children, a generation advance, or LifeGraph mutation.

## 6. Commit phase

Commit occurs only after structural validation, stale resolution, and successful construction of all required pending children.

The successful transition applies the accepted deaths and accepted births together as one evolution transition.

The life-graph semantics are:

- an active process has a corresponding life node;
- a death removes the active life node from the active graph, records death metadata, and places the resulting historical node in the fossil/history set;
- a birth creates the child process and corresponding life node, records the parent/child relation and birth metadata, and makes the child eligible for scheduling.

A successful application advances the population generation as specified by the evolution transition.

The commit boundary is intended to be infallible after all relevant failure conditions have been resolved during validation and build.

## 7. Audit

Evolution application outcomes are auditable. The contract distinguishes successful application from rejection/failure and partial-application outcomes.

The audit record must not be used as a substitute for the state-transition rules above. It records the externally relevant outcome of an application attempt.

The exact serialized audit representation is implementation-defined unless separately specified; the semantic distinction between successful application and rejected/failed/partial outcomes is normative.

## 8. Error classes

The contract distinguishes structural rejection from build failure and from commit-time collision/integrity failure.

Structural violations are detected before commit.

Build failures occur while constructing pending children and must not produce partial committed evolution.

A commit-time integrity failure is itself a contract violation of the build/commit boundary if it can leave partial state behind; therefore all determinable commit-time collision conditions must be prevented before mutation begins.

## 9. Observability and invariants

The following properties are normative and independently testable:

1. **Capability safety** — protected/immune processes cannot be admitted through the evolution capability boundary.
2. **Structural atomicity** — a structurally invalid plan causes no partial evolution-state mutation.
3. **Build-then-commit atomicity** — a failed child build causes no partial committed evolution.
4. **Temporal tolerance** — stale requests are individually tolerated and do not invalidate unrelated valid requests.
5. **Lazy invalidation safety** — stale scheduler references may be skipped without treating the stale reference itself as a fatal evolution error.
6. **LifeGraph consistency** — active processes and their life-graph representation remain mutually consistent across accepted transitions.
7. **Genealogy preservation** — accepted births preserve parent/child relationships and associated birth/death metadata.

These invariants define the semantic contract. They do not prescribe a particular Rust type layout, collection type, helper function, or control-flow implementation.

## 10. Scope boundary

This v1.0 contract does not prescribe:

- a particular programming language;
- concrete Rust symbols or module layout;
- the internal representation of processes, queues, or graphs;
- a particular PID allocator implementation;
- a particular child-builder implementation;
- a particular audit serialization format;
- fitness-selection algorithms beyond the evolution-plan application boundary;
- integration with the larger SIGADEFA kernel.

Those are implementation or integration concerns and must not silently become normative requirements for v1.0.

## Provenance and freeze rule

Source artifact recovered: **“Evolution Application Contract v1.0 — reference skeleton”** (saved prior design artifact).

Reconciliation rule applied:

- semantic design statements were retained;
- implementation-specific code was translated into implementation-neutral requirements only where the recovered artifact explicitly presented the behavior as a contract invariant;
- current implementation observations, current test coverage, and later proposed changes were excluded from the normative text;
- unresolved implementation details were not promoted to contract law.

**Status: FROZEN.**

Any normative change requires a new contract version or an explicitly governed amendment. This file must not be silently rewritten while retaining version `v1.0`.
