# SIGADEFA Ω Evolution Application Contract v1.0

Status: **RECOVERED-DRAFT — NOT FROZEN**

## Provenance

This file is the canonical Git location reserved for the design-level Evolution Application Contract v1.0. Its existence closes the previous source-addressability gap, but its content is deliberately not declared frozen until it has been reconciled with the recovered prior design artifact.

The recovered prior material identifies this contract as the design-level specification and describes the application pipeline as:

**Validate → Resolve Stale → Build → Commit → Audit**

It also identifies the core contract concepts `ProtectionLevel`, capability witness `EvolvablePid`, `PopulationPolicy`, `DeathRequest`, `BranchRequest`, `EvolutionPlan`, structural/build error classes, and audit outcomes.

## Recovery boundary

The following facts are established by the recovered material and may be used as recovery evidence:

- Contract status was described as **design-level CLOSED** while implementation status was **SCAFFOLD**.
- Capability safety was an explicit invariant: protected/immune processes are not eligible for an evolvable capability witness.
- Structural validation includes duplicate death, conflicting death/branch, reproduction budget, per-parent cap, mutation-rate validity, population floor, and generation mismatch.
- Build failures are separated from structural failures.
- Temporal tolerance includes stale-request handling without invalidating unrelated valid requests.
- Build-then-commit semantics use a pending-child/build phase before commit.
- The design contains LifeGraph/genealogy and audit concepts.

## Important restriction

This recovery document is **not yet a complete normative transcription** of the original contract. No omitted semantic detail may be inferred from `src/lib.rs`, current tests, README claims, or current implementation behavior and then silently promoted into Contract v1.0.

## Freeze condition

This document may be marked `FROZEN` only after:

1. the recovered prior design artifact has been reconciled against this file;
2. every normative claim has an identified source in the recovered material;
3. implementation-derived statements have been separated from design-level requirements;
4. the resulting v1.0 text is reviewed as a specification independent of the current implementation;
5. the freezing commit SHA is recorded by dependent Gates.

Until then, this file is a **canonical location, not a frozen authority**.
