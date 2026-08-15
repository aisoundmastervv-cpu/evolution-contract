# Working Rules

## Authority
1. GitHub is the shared source of truth.
2. The current project context lives in `.project/`.
3. A task is not approved merely because an agent can make it compile or pass a test; scope must be explicit.

## Change discipline
1. Prefer the smallest possible diff.
2. Diagnose before fixing.
3. Do not refactor unrelated code.
4. Do not alter tests merely to make a gate green unless the test fixture is demonstrably wrong and the change is explicitly authorized.
5. Preserve the frozen scaffold and existing project contracts unless an explicit task authorizes a change.

## Git safety
1. `git commit` is FORBIDDEN unless explicitly authorized in the current task.
2. `git push` is FORBIDDEN unless explicitly authorized in the current task.
3. Do not run `git reset --hard`, `git checkout`, `git clean`, or destructive history operations unless explicitly authorized.
4. Do not rewrite or discard existing unapproved work merely to obtain a clean tree.
5. Before any commit, show the exact diff and validation result and obtain explicit approval.

## Validation discipline
1. Stop at the first unexpected validation failure.
2. Do not automatically fix a second problem discovered by validation.
3. Distinguish production-code defects, test-fixture defects, formatting-only defects, and environment/tooling defects.
4. Report the exact observed failure before proposing a fix.

## Continuity
When a new session starts, read `CONTEXT.md`, `VALIDATION.md`, and `NEXT.md` first. Update `.project/NEXT.md` whenever the approved next action changes.
