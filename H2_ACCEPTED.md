# H2 — ACCEPTED

## Status

**ACCEPTED as a mathematical / architectural hypothesis.**

This status does not claim causal execution in a real process. That question is H3.

## Causal model

`efficiency gene → observed CPU cost → fitness`

For a fixed workload:

`observed_cpu_ms = 500 × (1 − 0.9 × efficiency / 255)`

Fitness is evaluated against a fixed environmental budget:

`fitness = 100 / (100 + observed_cpu_ms)`

## Required properties

- observed CPU cost strictly decreases with efficiency;
- fitness strictly decreases as observed CPU cost increases;
- composite fitness strictly increases with efficiency;
- every adjacent efficiency pair has a positive selection difference;
- boundaries remain finite and inside `(0, 1)`.

These properties are covered by six regression tests.

## Boundary

H2 validates the mathematical selection surface. It does **not** establish that a real execution environment produces the proposed genotype-to-phenotype causal link. That is H3.
