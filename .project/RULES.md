# Project Rules

## Source of truth
- GitHub commit history is the source of truth.
- Sandbox files, chat excerpts, and remembered content are references only until verified against GitHub.
- A file is identified by its GitHub commit/ref and locked blob identity, never by a filename or a claimed local hash.

## Gate semantics
1. **IDENTITY** — exact baseline commit, ancestry, and locked scaffold blobs.
2. **INTEGRITY** — `cargo check --all-targets` on the working commit.
3. **EXECUTION** — the requested experiment actually runs.
4. **EVIDENCE** — record the concrete test/measurement result.
5. **HYPOTHESIS** — interpret evidence only after execution.
6. **QUALITY** — formatting/linting are advisory unless explicitly promoted to a blocking contract.

A failure in one gate must never be reported as a failure of a later gate.

## Frozen work
- H2 is frozen: mathematical consistency confirmed by 6/6 regression tests; causal and empirical validity remain untested.
- Do not modify H2 implementation or its tests while working on H3.
- H3 is blocked until an independent causal bridge, workload, and measurement path are identified.

## Anti-self-fulfilling rule
A hypothesis implementation must not contain the mechanism whose causal existence the hypothesis is intended to test.

## Agent continuation
- When the user explicitly authorizes implementation, do not stop after diagnosis if the next change is within that approved scope.
- Continue through the smallest necessary implementation and its validation.
- Stop only for an unexpected validation result, destructive/irreversible action, missing authority, or a genuinely ambiguous scope boundary.
- Do not invent a blocker merely because a diagnostic step revealed another step that is already authorized.

## Change discipline
- Prefer the smallest diff that enforces the current architectural decision.
- Do not rewrite frozen artifacts to make a gate pass.
- Keep diagnostic patches separate from baselines.
- After changes, inspect the exact diff and run the relevant gates before reporting completion.
