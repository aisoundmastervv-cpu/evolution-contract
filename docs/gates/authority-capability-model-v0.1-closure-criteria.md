# Authority Capability Model v0.1 — Closure Criteria

Status: DRAFT / NOT ADMITTED

This artifact records the mandatory closure conditions identified during governance review. It does not itself grant authority.

## 1. Closed-world authority predicate

For any concrete request:

```text
AUTHORIZED(actor, object_revision, scope, t)
```

the result is `TRUE` if and only if a complete, valid, canonical, non-revoked authority lineage exists satisfying every required condition.

If any required condition is absent or unverifiable:

```text
AUTHORIZED = FALSE
```

Execution enforcement has no implicit `MAYBE` state.

## 2. Required lineage components

For execution authority, all of the following are mandatory:

```text
actor
actor authority domain
canonical authority event
object id
object revision
object hash
scope
valid_from
valid_until
parent lineage / authorized root
```

Missing any one component yields:

```text
missing component
    -> incomplete lineage
    -> authority absent
    -> NOT_AUTHORIZED
```

## 3. Actor-authority closure

`actor_authority_domain` is a claim, not proof.

For every non-root authority event:

```text
actor_authority(actor, domain)
    requires
valid_canonical_lineage(actor, domain)
```

The authority root must be separately defined and governed. An authority event cannot bootstrap the authority of its own issuer.

## 4. Explicit temporal validity

Both fields are mandatory:

```text
valid_from
valid_until
```

No higher-level-contract fallback may supply a missing `valid_until` in v0.1. No authority is perpetual by default.

## 5. Counterexample closure test

Governance must test at least these cases:

### AUTH-CLOSED-001 — missing lineage component

Remove exactly one required lineage component from an otherwise valid authority record.

Expected:

```text
AUTHORIZED = FALSE
```

### AUTH-ACTOR-001 — unverified actor authority

Keep all event fields valid-looking but remove the independent authority lineage for the actor/domain.

Expected:

```text
AUTHORIZED = FALSE
```

### AUTH-TIME-001 — missing validity boundary

Remove `valid_until`.

Expected:

```text
AUTHORIZED = FALSE
```

### AUTH-SCOPE-001 — scope mismatch

Use a valid authority record whose scope does not contain the requested operation.

Expected:

```text
AUTHORIZED = FALSE
```

### AUTH-REV-001 — revision mismatch

Use a valid authority record for the same `object_id` but a different revision/hash.

Expected:

```text
AUTHORIZED = FALSE
```

### AUTH-REVOKE-001 — revoked grant

Use a valid authorization whose lineage contains a later applicable revocation event.

Expected:

```text
AUTHORIZED = FALSE
```

### AUTH-ESC-001 — self-escalation

Construct a lineage in which the execution subject attempts to grant itself a higher authority domain.

Expected:

```text
AUTHORIZED = FALSE
```

## 6. Admission condition

`Authority Capability Model v0.1` MUST NOT be admitted until governance can show that no valid normative interpretation produces `AUTHORIZED = TRUE` when any required lineage condition is absent or unverifiable.

This closure criteria artifact is a review instrument only. It creates no authority and does not authorize `AuthorizedExecutionToken`, enforcement, implementation, or execution.
