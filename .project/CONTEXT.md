# Project Context

## Project
- Repository: `aisoundmastervv-cpu/evolution-contract`
- Default branch: `main`
- GitHub is the source of truth for shared project state.

## Current work state
- GitHub `main` currently ends at `74c9e9448967155cc09ee3cb7da8a6aedbef1060` (`Add frozen scaffold CI workflow`).
- The active local workspace previously reported branch `work` and local commit `a89aeec` (`Fix evolution plan build errors`), which has NOT been pushed to GitHub.
- The local commit contains exactly three intended code changes: `BuildError -> ApplyError::Build`, `**pid -> *pid`, and the `failed_build_commits_nothing` fixture change `(i % 2) -> (i % 3)`.
- The local workspace also has an existing untracked `Cargo.lock`; `target/` was removed after diagnostics.
- No push has been performed.

## Validation state
- The diagnostic run established that the original `failed_build_commits_nothing` fixture failed structural validation because two parents received 6 offspring each while the cap was 4.
- The fixture was changed to distribute 12 requests across parents 2, 3, and 4, giving each exactly 4 offspring and making `FailingBuilder { fail_at: 7 }` reachable.
- The local validation after the three intended changes stopped at `cargo fmt --check` because `src/lib.rs` is not rustfmt-clean. The remaining gates were not run in that run.
- The Codex agent also created local commit `a89aeec` despite an explicit no-commit instruction. Treat that commit as unapproved until explicitly reviewed.

## Current objective
Finish validation of the minimal three-change patch without introducing unrelated changes, then decide explicitly what to do with the local unpushed commit.

## Continuity rule
At the start of a new work session, read `.project/RULES.md`, `.project/VALIDATION.md`, and `.project/NEXT.md` before touching code. Do not infer approval from old chat history when the repository state or these files say otherwise.
