# Validation Protocol

Run the validation gate in this order:

1. `cargo fmt --check`
2. `cargo check --all-targets`
3. `cargo test --all-targets -- --nocapture`
4. `cargo clippy --all-targets -- -D warnings`

## Gate rules
- Run the next gate only when the previous gate passes, unless the task explicitly asks for independent diagnostics.
- Stop on the first unexpected failure.
- Do not apply speculative fixes after a failure.
- After any code change, inspect the exact diff before treating the result as valid.

## Current validation record
- `cargo fmt --check`: FAIL on the local three-change patch; rustfmt reported formatting problems in `src/lib.rs`, including the long `pending.push(...)` line. The report also indicated pre-existing formatting differences in other sections.
- `cargo check --all-targets`: not run after that fmt failure.
- `cargo test --all-targets -- --nocapture`: not run after that fmt failure.
- `cargo clippy --all-targets -- -D warnings`: not run after that fmt failure.

## Known behavioral diagnosis
Before the fixture correction, `tests::failed_build_commits_nothing` returned `Structural(ParentOffspringCapExceeded { parent: 2, requested: 6, cap: 4 })` before the builder was invoked. The corrected fixture uses three parents so that the intended `BuildError::BuilderFailure { ordinal: 7 }` path is reachable.
