# Agent Operating Rule

## Action Continuity Rule

When the user explicitly authorizes implementation or continuation, the agent MUST carry the task through the next safe, technically necessary repository action using the available GitHub/CI tools. It MUST NOT stop after diagnosis merely to report that a change, commit, workflow run, or verification is still needed.

### Required behavior

1. **Diagnose, then act.** Once a concrete blocker is established, make the smallest justified change that resolves that blocker, provided the change is within the authorized scope.
2. **Preserve provenance.** Never rewrite or silently repair an immutable baseline. Use a separate diagnostic/experiment commit or branch for fixes.
3. **Verify immediately.** After a change, run or trigger the narrowest relevant verification available (for example `cargo check --all-targets` after compiler-only fixes).
4. **Do not hand work back to the user** when the repository and required write/CI capabilities are already available to the agent. User involvement is required only for missing authorization, destructive ambiguity, unavailable credentials/capabilities, or a genuinely necessary product decision.
5. **Keep scope minimal.** Do not format, refactor, rename, rebase, create a new baseline, or expand the experiment unless required by the current gate or explicitly authorized.
6. **Keep gates separate.** Record Identity, Integrity, Execution, Evidence, and Hypothesis results independently. A failure in one gate must not be misreported as a failure in another.
7. **Never infer success.** A requested action is considered complete only when the repository/CI provides observable evidence of completion.

### Default progression

```text
USER AUTHORIZES IMPLEMENTATION
        ↓
INSPECT CURRENT REPOSITORY STATE
        ↓
MAKE MINIMAL NECESSARY CHANGE
        ↓
COMMIT/PUSH WHEN IN SCOPE
        ↓
RUN/TRIGGER THE NARROWEST RELEVANT CHECK
        ↓
REPORT OBSERVED FACT
        ↓
CONTINUE TO THE NEXT AUTHORIZED GATE
```

### Stop conditions

Stop only when:

- the requested scope is ambiguous or destructive;
- required access/capability is genuinely unavailable;
- a gate produces a result that requires a substantive decision outside the authorized scope; or
- the user explicitly asks to stop.

A mere need for a normal repository edit, commit, push, workflow dispatch, or test run is **not** a stop condition when the agent has the capability to perform it.
