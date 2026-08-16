# LifeGraph N05b — Death Metadata Source Audit v1

Status: **SOURCE AUDIT COMPLETE — COMPONENT FIELD NOT SPECIFIED**

## Scope

This audit answers one narrow question:

> What exactly does frozen Contract v1.0 define as the death metadata required by the LifeGraph death transition?

No change is made to Contract v1.0, C-LG/O-LG, the frozen Test Plan, or implementation code.

## Canonical Contract source

`docs/spec/evolution-application-contract-v1.0.md`, frozen.

Relevant normative statement in §6 (Commit phase):

> a death removes the active life node from the active graph, **records death metadata**, and places the resulting historical node in the fossil/history set.

The same semantic requirement is repeated in §9 as part of **Genealogy preservation**:

> accepted births preserve parent/child relationships and associated birth/death metadata.

## What the Contract actually specifies

The Contract establishes three distinct semantic obligations for death:

1. the active life node is removed from the active graph;
2. death metadata is recorded;
3. the resulting historical node is placed in fossil/history.

The Contract does **not** specify the internal or semantic decomposition of "death metadata" into named fields.

It does not identify `death_reason`, `death_cycle`, or any other concrete field as individually normative.

It also does not define a semantic absence encoding for a component of death metadata.

## Consequence for N05b

N05b is legitimate at the level of the composite Contract requirement: a death state in which the required death metadata is not recorded is a valid semantic counterstate **in principle**.

However, the current test construction cannot claim that setting one implementation field such as `death_reason = None` is necessarily the Contract counterstate, because the Contract does not define that field or its relationship to the composite metadata requirement.

Likewise, treating `death_cycle = None`, `0`, an empty record, or another implementation convention as the Contract-level absence of death metadata would add an unstated semantic rule.

## Audit verdict

**N05b has Contract authority, but the Contract does not provide a field-level oracle for constructing its counterstate.**

Therefore the current N05b fixture is not justified as written.

The cleanest Contract-faithful formulation is a counterstate at the same semantic granularity as the Contract requirement:

> accepted death + active node removed + historical representation present + required death-metadata record absent.

Whether the current implementation can expose that composite absence independently is a separate observation-surface question.

## Governance boundary

This audit does not amend the Contract or the frozen test plan. Any change to N05b must be handled as test-construction work after this source audit, and must not invent a field-level semantic predicate absent from Contract v1.0.
