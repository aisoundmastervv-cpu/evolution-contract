# H1 — REJECTED

## Status

**REJECTED — causal validation failed.**

## Hypothesis

`efficiency gene → CPU budget → fitness`

## Failure

The original model used:

`budget_ms(efficiency) = max(10, 1000 − 3.88 × efficiency)`

and `fitness = min(1, budget / observed_cpu)`.

This produced a broad fitness plateau and a negative selection gradient with respect to efficiency at fixed observed CPU. The gene therefore acted as an evaluation constraint rather than as a cause of the measured phenotype.

## Negative-control role

H1 is retained as a rejected causal model. Its failure is evidence against self-referential fitness evaluation of this form; it is not evidence that CPU cost is irrelevant to evolution.
