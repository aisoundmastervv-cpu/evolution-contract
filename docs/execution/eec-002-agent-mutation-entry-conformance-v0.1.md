# EEC-002 — Agent Mutation Entry Conformance v0.1

## Status

Draft / reference gate.

## Claim

Any mutation-capable agent execution working on `evolution-contract` MUST obtain mutation authorization through the canonical mutation entrypoint. The entrypoint MUST first establish canonical repository identity and EEC-001 environment conformance. If the gate is not `READY`, the mutation command MUST NOT execute.

## Scope

EEC-002 verifies the **entrypoint contract**. It does not claim that an arbitrary agent with unrestricted shell/filesystem access is technically unable to bypass the entrypoint. That stronger capability claim is reserved for EEC-003.

## Required sequence

```text
agent session
    ↓
canonical mutation entrypoint
    ↓
EEC-001 identity + environment gate
    ↓
READY
    ↓
mutation command
    ↓
execution/evidence
```

A new agent session MUST NOT inherit mutation authorization from a previous session, workspace, process, or memory. Authorization is re-established from canonical repository state and the current conformant environment.

## Fail-closed requirements

The entrypoint MUST stop before invoking the mutation command when any of the following is true:

- canonical repository cannot be established;
- canonical identity is missing or inconsistent;
- exact expected commit is missing or mismatched;
- workspace is not clean;
- EEC-001 environment preflight is not ready.

## Evidence requirements

A conformant implementation MUST demonstrate at least:

1. a conformant environment reaches `READY` and permits the supplied mutation command;
2. an identity/environment failure prevents the mutation command from executing;
3. the workflow records the raw result independently of the workspace.

## Relationship to EEC-003

EEC-003 is a separate capability boundary. It may only be declared when the actual mutation-capable agent harness prevents direct filesystem mutation without passing the entrypoint. A repository script alone cannot establish that stronger property when the agent has unrestricted shell access.
