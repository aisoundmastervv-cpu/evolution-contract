# Validation Machine State Model v0.1 — Approval Record

**Status:** APPROVED / AUTHORIZED

## Approved artifact

`docs/gates/validation-machine-state-model-v0.1.md`

**Approved revision:** `a3cdeaf009e2e1afff136b6883cb33840a742b1f`

## Approval basis

The State Model review found the model sufficiently formal for machine-level governance after the addition of the global unauthorized-transition guard.

The approved model defines:

- explicit machine states and terminal `STOP` state;
- authorized core transitions;
- `OBSERVATION_GAP -> STOP` for unavailable required observations;
- `UNDERDETERMINED -> STOP` when available evidence cannot support a permitted semantic conclusion;
- `FORBIDDEN_TRANSITION -> NOT_AUTHORIZED -> STOP` for rejected transitions;
- a global guard preventing unauthorized transitions from executing;
- explicit separation of verdict classes from machine states;
- frozen-artifact and agent-neutrality invariants;
- explicit non-goals preserving Contract, C-LG/O-LG, and Test Plan boundaries.

## Authorization

This approval authorizes the State Model as the machine-level governance specification for subsequent design work.

It does **not** by itself authorize:

- production-code changes;
- harness changes;
- Test Plan changes;
- Contract or C-LG/O-LG changes;
- implementation of a cloud execution layer;
- implementation of an agent protocol.

Those actions remain subject to their own applicable governance and approval requirements.

## Boundary

```text
Validation Machine State Model v0.1    APPROVED / AUTHORIZED
Contract                              UNCHANGED
C-LG / O-LG                           UNCHANGED
Test Plan v1.1                        UNCHANGED
Harness                               UNCHANGED
Production code                       UNCHANGED
Cloud execution layer                 NOT YET AUTHORIZED
```

Approval is recorded separately from the State Model artifact, preserving the distinction between **review approval** and **artifact authorization**.
