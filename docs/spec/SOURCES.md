# Specification Source Registry

Status: **GOVERNANCE BASELINE**

## Rule

Any artifact used as normative source material for a Gate MUST be addressable by a versioned Git path and commit. Chat messages, writing blocks, local workspace files, and implementation behavior are not normative sources by themselves.

## Current contract recovery

The repository README states that the SIGADEFA Ω Evolution Application Contract v1.0 is design-level CLOSED while the implementation is a scaffold. The earlier working materials contain the recovered design-level reference material, but that material was not previously represented by a canonical Git path.

Therefore the authoritative Contract v1.0 source is **not yet frozen in Git**.

## Canonical source

Target canonical path:

`docs/spec/evolution-application-contract-v1.0.md`

Required status before use as a normative Gate source:

- content recovered from the prior design-level artifact;
- reviewed for completeness against the recovered source;
- explicitly marked `FROZEN`;
- referenced by exact commit SHA from dependent Gates.

Until those conditions are met, the file must not be treated as a frozen oracle source.

## Gate dependency rule

A Gate may claim semantic validation only against a source listed here with an explicit version and frozen commit. If a referenced source cannot be resolved, the Gate is blocked rather than reconstructed from implementation behavior.

## Non-normative recovery source

The prior ChatGPT-generated artifact `Вставленная ​​уценка(3).md` is retained in the conversation/library as recovery evidence. It may help reconstruct and audit the contract, but it is not itself the canonical normative source.

## Change discipline

Changes to a frozen contract require a new version or an explicitly governed amendment. A Gate must never silently update its normative source by following a moving branch tip.
