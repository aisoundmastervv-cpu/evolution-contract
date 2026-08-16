# Workspace Recovery Contract v0.1

**Status:** implementation candidate; not yet a governance approval.

## Rule

A workspace is disposable execution state. Loss of a workspace is a recoverable execution event, not loss of canonical project state.

## Recovery procedure

When the current workspace is missing, corrupted, or untrusted:

1. Stop mutation and do not treat the old workspace as evidence.
2. Resolve the canonical repository and exact required commit from Git/governance state.
3. Bootstrap a fresh workspace from that commit using the Environment Contract.
4. Run environment preflight and require `ENVIRONMENT: READY`.
5. Derive a new Environment Identity Record.
6. Re-run the required execution and generate new evidence.
7. Link the new evidence to the exact commit and environment identity.

## Prohibited recovery shortcut

Do not reconstruct a historical workspace merely to preserve its filesystem path or claim continuity from its local state. A recovered workspace is valid only because it was independently reconstructed from canonical state.

## Evidence rule

Old workspace-local files are not authoritative evidence unless they have already been persisted as independently identifiable evidence artifacts. New execution after recovery produces new evidence.

## Success condition

Recovery is successful when a fresh workspace reaches `ENVIRONMENT: READY` for the requested commit and the required execution can be repeated without relying on the destroyed workspace.
