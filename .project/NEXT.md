# Next Action

## Approved next step
Diagnose the `cargo fmt --check` failure on the local three-change patch.

### Do not change code yet
- Do not run `cargo fmt` as a write operation.
- Do not modify production code.
- Do not modify tests.
- Do not modify `Cargo.toml` or `Cargo.lock`.
- Do not commit or push.
- Do not reset, checkout, rebase, or clean the workspace.

### Diagnostic goal
Determine exactly which formatting differences are pre-existing and which, if any, were introduced by the three-change patch. The purpose is to decide whether formatting can be repaired with a minimal targeted edit or whether the repository baseline itself is not rustfmt-clean.

### After diagnosis
Report:
1. exact rustfmt findings;
2. whether the findings pre-date the patch;
3. smallest safe next change, if any;
4. expected impact on the approved three-change diff;
5. confirmation that no code or Git history was changed.

No fix is approved until that report is reviewed.
