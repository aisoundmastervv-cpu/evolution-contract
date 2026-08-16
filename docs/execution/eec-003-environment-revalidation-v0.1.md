# EEC-003 — Environment Revalidation

**Status:** experimental execution gate

## Claim

After an initial conformant `AGENT_ENVIRONMENT: READY`, a mutation-capable agent MUST revalidate the current environment identity at the mutation boundary. If the environment identity changes while canonical repository identity, exact commit, and workspace cleanliness remain unchanged, mutation MUST be blocked.

## Controlled scenario

```text
initial environment = conformant
repository          = canonical
HEAD                = expected commit
git status          = CLEAN
        ↓
AGENT_ENVIRONMENT: READY
        ↓
change only environment identity outside workspace
        ↓
repository          = unchanged
HEAD                = unchanged
git status          = CLEAN
        ↓
repeat mutation entrypoint
        ↓
AGENT_ENVIRONMENT: NOT_READY
reason=environment-identity-mismatch
        ↓
mutation sentinel = ABSENT
```

## Environment identity

The experiment uses the identity inputs already defined by Environment Contract v0.1:

- canonical repository URL;
- exact commit SHA;
- Environment Contract content hash;
- bootstrap implementation content hash;
- `rustc --version`;
- `cargo --version`.

The controlled drift changes only the observed `rustc` identity through an executable wrapper outside the repository workspace. No repository file is modified by the drift operation.

## Evidence boundary

A PASS applies only to the tested invariant:

> clean canonical workspace + unchanged repository identity + unchanged commit + changed environment identity → revalidation rejects mutation.

This does not claim arbitrary bypass resistance or universal enforcement against mutation paths that do not invoke the reference mutation entrypoint.
