# H3 Acceptance Criterion — Historical Audit v0.1

## Audit status

- Audit result: **CANONICAL THRESHOLD NOT FOUND**
- H3 execution: **BLOCKED** pending explicit acceptance-criterion registration
- Audit base: canonical continuation at `ebf805523806720f090f76a818f924051ad58cf3`
- Repository: `aisoundmastervv-cpu/evolution-contract`

## Question

Was an H3 acceptance criterion already registered in canonical project history such that it can be recovered and reused without creating a new methodological rule?

## Canonical finding

No canonical H3 acceptance threshold or numerical causal acceptance criterion was found in the verified canonical lineage used for the current continuation.

The repository's current H3 state explicitly requires that an acceptance threshold be recovered from canonical history or explicitly registered before execution. This audit confirms that the first alternative is not currently available.

## Non-canonical historical material discovered

A prior unmerged/draft H3 PR (#8, `H3: pre-register independent causal execution requirements`) contains an execution workflow with a numerical criterion:

- control/intervention CPU ratio for `gene=0`: median ratio `<= 0.5`;
- comparison condition for `gene=255`: median ratio `>= 2.0 * median_ratio_e0`;
- five valid trials for each condition;
- otherwise `INCONCLUSIVE` or `NULL-FALSIFICATION`.

The same PR contains an earlier mapping/registration reference using `nice` values and an `efficiency -> nice` mapping. The PR is not part of the canonical continuation used for the present H3 state. Its base/head lineage is separate from the canonical EEC-003 continuation, and comparison against `main` shows divergence rather than ancestry.

Therefore this criterion is classified as **historical non-canonical proposal**, not as a recoverable canonical requirement.

## Governance consequence

The non-canonical criterion MUST NOT be silently promoted to canonical H3 acceptance logic.

The registered arm `H3-OS-NICE-CAUSAL-v0.1` therefore remains valid as an execution-arm registration, but H3 execution remains blocked until an acceptance criterion is explicitly registered through the project's proposal/admission process.

The already registered actuator parameters (`nice 0` control and `nice +10` treatment) MUST NOT be retroactively justified by the non-canonical PR #8 criterion.

## Required next step

Create a separate H3 Acceptance Criterion Proposal containing:

1. primary observable;
2. control/treatment comparison;
3. minimum effect criterion;
4. repeatability requirement;
5. noise/variance handling;
6. inconclusive/failure semantics;
7. provenance requirements;
8. rationale for every numerical parameter;
9. explicit statement that the criterion is newly proposed and not recovered from canonical history.

No H3 causal execution is authorized by this audit.
