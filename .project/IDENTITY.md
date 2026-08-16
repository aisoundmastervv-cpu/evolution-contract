# Canonical Project Identity

This file is a provenance guard for continuity across agents and sessions.

## Canonical repository
- Full name: `aisoundmastervv-cpu/evolution-contract`
- GitHub repository ID: `1335431893`
- Default branch: `main`
- Repository is the sole canonical GitHub source of truth for this project.

## Frozen H2 anchors
- Immutable baseline commit: `ffb45c06fd835f9b8cc765d91797090b9bfccc34`
- Diagnostic H2 continuation commit: `ef43452a45e86ac6e398a0afe5c0af79fe732cce`
- H2 validation result: `6/6 PASS`

## Architecture continuation anchor
- Continuity architecture branch: `architecture/continuity-gates-v1`
- Current architecture commit: `f31f9c4ea617407bfcc8a8bbad7ef088a85bb3c3`
- Architecture PR: `#7`

## Mandatory context-recovery protocol
Before any write, implementation, branch creation, commit, push, or PR action, the agent MUST verify all of the following against GitHub:

1. Repository full name is exactly `aisoundmastervv-cpu/evolution-contract`.
2. Repository ID is exactly `1335431893`.
3. The intended continuation point is identified by an exact GitHub commit SHA, not by filename, chat memory, or local state.
4. The frozen H2 anchors above still resolve to the expected commits.
5. The current branch/PR lineage is compatible with the requested task.
6. Any referenced artifact from another conversation, repository, agent, or workspace is treated as **foreign context** until independently verified against this repository.

If any identity check fails or is unavailable, the agent MUST stop mutation and perform read-only recovery only.

## No cross-context inheritance
Names, files, PR numbers, branches, claims, and design decisions from another project or conversation MUST NOT be imported merely because they are semantically similar. A matching filename or concept is not provenance.

## Source hierarchy
1. GitHub repository state and commit graph
2. GitHub PR/CI evidence attached to that graph
3. Project continuity files in `.project/`
4. Current conversation statements, only when reconciled with GitHub
5. Memory, cached summaries, sandbox artifacts, or other chat contexts — reference only

## Mutation rule
The agent must establish identity and lineage **before** making any mutation. Authorization to implement does not override provenance verification; it only removes the need to stop after a verified scope is established.

## Incident rule
If an agent discovers that it has mixed contexts or mutated an unverified repository, it must:
- stop further mutation;
- preserve the accidental artifact for auditability;
- mark it non-canonical;
- close/revert only through an explicit, traceable action when appropriate;
- re-establish canonical identity from this file and the GitHub commit graph before continuing.
