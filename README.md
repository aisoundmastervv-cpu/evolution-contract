# evolution-contract

Scaffold for the SIGADEFA Ω **Evolution Application Contract v1.0**.

## Status

- Contract: **CLOSED** (design-level)
- Implementation: **SCAFFOLD** — statically reviewed by hand, not yet
  verified by a real `cargo test` run anywhere. This repository's CI
  is the first place that claim gets checked against a compiler.
- Integration: not started. `Process`, `LifeNodeDraft`, `ChildBuilder`,
  `PidAllocator` are stand-ins for `SigadefaProcess`, `LifeGraph`,
  `branch_process`, and a production PID allocator respectively.

## What's proven here

`src/lib.rs` implements and tests, via `apply_evolution_plan`:

- **Capability safety** — `EvolvablePid` is only constructible through
  `EvolvablePid::from_filtered`, which rejects `Immune`/`Protected`
  processes.
- **Structural atomicity** — an invalid plan (duplicate death request,
  conflicting death/branch, budget overflow, per-parent offspring cap,
  population floor violation) is rejected wholesale with zero mutation.
- **Build-then-commit atomicity** — a failure partway through
  constructing children (see `failed_build_commits_nothing`) commits
  nothing: no deaths, no children, no generation bump, no LifeGraph
  mutation. Only the failure itself is recorded via `AuditEvent`.
- **Temporal tolerance** — requests referencing an entity that's
  disappeared between plan evaluation and apply time (e.g. a parent
  killed administratively) are individually skipped, not treated as
  plan-wide errors.
- **Lazy heap invalidation** — dead PIDs are never scrubbed out of the
  ready queue; they're filtered on pop (`next_runnable`).

## Smoke gate

Every commit runs `cargo fmt --check`, `cargo check`, `cargo test`,
`cargo clippy -- -D warnings` via GitHub Actions
(`.github/workflows/rust.yml`). A green run on `main` is the only
thing that upgrades this repo's status from "statically reviewed" to
"compiler-verified."

## Next steps (one boundary at a time, gated by CI)

1. `Process` → real `SigadefaProcess`
2. `LifeNodeDraft` → real `LifeGraph`
3. `ChildBuilder` → real `branch_process`
4. `PidAllocator` → production PID allocation
5. Connect `FitnessEvaluator` / `EvolutionEngine`
6. Re-run full smoke gate after each boundary
