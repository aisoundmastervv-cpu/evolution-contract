# LifeGraph N05b Observation-Surface Review v1

Status: **OBSERVATION GAP — IMPLEMENTATION NOT VALIDATED**

## Question

Does LG-N05b have a Contract-supported semantic counterstate, and can the current harness observe it without introducing an implementation-specific absence convention?

## Frozen basis

C-LG/O-LG explicitly require, for accepted death, that the required death metadata is recorded in the historical representation. The semantic requirement therefore exists. The prior negative-counterstate audit classified N05b as Contract-supported in principle.

## Current observation surface

The harness observes historical nodes with:

- `death_cycle: Option<u64>`;
- `death_reason: Option<DeathReason>`.

The current N05b fixture removes `death_reason` by setting it to `None` and then expects O-LG to reject the state.

## Adversarial observation finding

The `None` value is not itself the problem: unlike N03b, the implementation already exposes death metadata as an optional semantic field, and the Contract/O-LG relation is about whether the required metadata is recorded.

However, the current implementation/harness does not establish that **all** death metadata required by the Contract is represented by `death_reason` alone. `death_cycle` is also exposed as optional and is part of the observed death representation. The current N05b fixture removes only one field while naming the expected failure generically as "death metadata missing".

Therefore the present case is under-specified at the observation level: it must identify exactly which Contract-supported death-metadata item is being removed and ensure that the other required metadata remains present.

## Verdict

**N05b is Contract-supported, but the current harness case is not yet observation-adequate.**

This is an observation/test-construction gap, not a reason to change C-LG/O-LG.

## Required next action

Do not change the frozen semantic baseline. Revise N05b only after tracing the exact death-metadata fields required by Contract v1.0 §6, then construct an isolated counterstate for one explicitly required field. If Contract treats a composite death-metadata record as atomic and does not identify its components, the test must instead observe absence of the whole required semantic record rather than selecting an implementation field.

No Gate execution is authorized from the current N05b implementation.
