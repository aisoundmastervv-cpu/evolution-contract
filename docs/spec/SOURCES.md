# Specification Source Registry

Status: **GOVERNANCE BASELINE**

## Rule

Any artifact used as normative source material for a Gate MUST be addressable by a versioned Git path and an exact commit. Chat messages, writing blocks, local workspace files, and implementation behavior are not normative sources by themselves.

## Canonical Contract v1.0

**Normative source:** `docs/spec/evolution-application-contract-v1.0.md`

**Version:** `1.0`

**Status:** `FROZEN`

**Freezing commit:** `118dd043d6d5d208bc197cda7583c1ea05f0bf47`

This is the authoritative design-level source for the Evolution Application Contract v1.0.

## Recovery provenance

The canonical document was reconciled from the previously saved design artifact **“Evolution Application Contract v1.0 — reference skeleton”**. The recovery retained design-level semantics and excluded current implementation observations, test results, and implementation-specific control flow from the normative text.

The prior ChatGPT-generated artifacts remain useful as recovery evidence, but they are not normative sources once the canonical contract is frozen.

## Gate dependency rule

A Gate may claim semantic validation only against a source listed here with an explicit version, frozen status, and exact commit SHA. If a referenced source cannot be resolved, the Gate is blocked rather than reconstructed from implementation behavior.

A Gate must reference the exact frozen commit, not a moving branch tip.

## Change discipline

Changes to a frozen contract require a new contract version or an explicitly governed amendment. The v1.0 file must not be silently rewritten while retaining version `1.0` and its frozen status.
