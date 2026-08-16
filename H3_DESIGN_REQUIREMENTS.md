# H3 Design Requirements

**Status:** DESIGN-FROZEN CANDIDATE / PRE-REGISTRATION DRAFT

**Hypothesis:** H3 — Causal Execution Hypothesis

**Direction:** OS-level independent causal bridge

**Primary bridge:** Linux cgroup CPU controller

**Fallback bridge:** Linux `nice` / scheduler priority

**H2 baseline:** preserved; no H2 implementation or tests are modified by this document.

---

## 1. Purpose

This document defines the minimum experimental contract required before implementing or executing H3.

The purpose is to test whether the `efficiency` gene can participate in a causal chain leading to a measurable difference in CPU allocation **through an execution mechanism external to SIGADEFA's fitness logic**.

This document is a protocol boundary, not an implementation specification for the H2 crate.

The governing architectural principle is:

> **SIGADEFA may control the cause only through an external system interface; SIGADEFA must not implement the function that maps gene to observed CPU.**

The experiment must therefore distinguish three things:

1. the intervention selected by SIGADEFA;
2. the causal resource-control mechanism implemented by Linux;
3. the independently measured execution outcome.

---

## 2. External Actuator

The primary external actuator is the Linux cgroup CPU controller.

SIGADEFA may request an externally visible CPU-control parameter for a process or process group. The parameter is consumed by the operating system's resource-control and scheduling machinery.

The actuator is considered external only if the resulting CPU allocation is determined by the operating system mechanism rather than by code inside SIGADEFA that directly throttles, sleeps, skips, or scales computation.

### Primary

- Linux cgroup CPU controller.
- The exact controller interface and supported parameter must be established by the execution environment before implementation.

### Fallback

If the cgroup CPU controller is unavailable or cannot provide a sufficiently reproducible intervention in the approved execution environment, the fallback is Linux `nice` / scheduler priority.

Fallback use must be explicitly recorded as a protocol deviation or as a pre-registered alternate arm; it must not silently replace the primary bridge after outcome inspection.

---

## 3. Causal Mechanism

The causal mechanism under test is the operating system's resource scheduling/control path.

Conceptually:

```text
efficiency gene
      |
      v
pre-registered external actuator parameter
      |
      v
Linux cgroup CPU controller
      |
      v
OS scheduling / CPU allocation
      |
      v
fixed workload execution
      |
      v
independent measurement
```

The experiment does **not** claim that Linux guarantees a particular wall-clock time or exact CPU percentage for every workload. It tests whether a controlled intervention in the external resource parameter produces a reproducible difference in CPU allocation under the specified execution conditions.

---

## 4. Forbidden Zone

The following mechanisms are prohibited in the H3 implementation and workload:

- `thread::sleep(...)` or equivalent delays derived from `efficiency`;
- reducing the number of workload iterations as a function of `efficiency`;
- selecting a cheaper or more expensive workload as a function of `efficiency`;
- computing an expected CPU value from `efficiency` and reporting it as measurement;
- computing `observed_cpu = f(efficiency)` anywhere in the SIGADEFA measurement path;
- scaling workload intensity directly from `efficiency`;
- inserting an artificial busy-loop or throttle whose intensity is derived from `efficiency`;
- changing the workload after observing CPU results;
- post-hoc changing of the gene-to-actuator mapping to obtain a desired effect;
- using previous measured CPU values to adapt the current actuator parameter unless such adaptation is independently pre-registered as part of the experimental design.

A code path that directly implements the hypothesized causal relationship is invalid evidence for H3, even if it produces the expected result.

---

## 5. Frozen Workload

The workload must be deterministic and identical across compared interventions, except for the external resource-control parameter.

The workload contract must specify before execution:

- executable/workload identity;
- input data or seed;
- iteration count or termination condition;
- process topology;
- concurrency level;
- affinity policy, if any;
- expected output and correctness check;
- warm-up policy;
- measurement interval;
- trial count;
- environment requirements.

The workload must perform real computation sufficient to create measurable CPU demand. It must not encode the `efficiency` gene into its amount of work.

A workload that fails its correctness check is not valid H3 evidence, regardless of its resource measurements.

---

## 6. Independent Measurement

CPU consumption must be measured by an execution/measurement layer outside the semantic fitness calculation.

The measurement path must obtain actual execution/resource information from the operating system or an independently controlled runtime facility. It must not infer CPU consumption from `efficiency`, the actuator parameter, or the workload configuration.

The measurement record must preserve at least:

- intervention/gene identifier;
- actuator parameter actually applied;
- workload identity/version;
- execution environment identity;
- trial identifier;
- CPU measurement;
- wall-clock measurement where available;
- exit status;
- workload correctness result;
- relevant CPU/resource-control configuration;
- raw evidence sufficient to reproduce the reported result.

The measurement layer must not modify the actuator parameter based on the measurement it is collecting during a trial.

---

## 7. Mapping Policy

**Mapping policy is intentionally unspecified at this design-freeze stage.**

No specific formula such as linear, exponential, or threshold mapping is part of the current H3 protocol.

Any mapping used for an actual execution must be **pre-registered before outcome observation** for that execution arm.

The selected mapping must:

- be deterministic for a given gene value and protocol version;
- be independent of measured CPU results;
- be independent of post-hoc trial results;
- be applied identically according to the registered rule;
- remain fixed for the registered experiment unless the protocol explicitly defines separate arms.

The mapping must describe only how the gene selects an **external actuator parameter**. It must never define the expected observed CPU result.

This separation is mandatory:

```text
efficiency -> actuator parameter       ALLOWED
actuator parameter -> observed CPU     OS/execution mechanism
observed CPU -> efficiency             NOT a causal implementation
observed CPU = f(efficiency)            FORBIDDEN
```

---

## 8. Control and Intervention Design

The experiment must contain a controlled comparison in which the workload and execution environment are held as constant as practical while the pre-registered external actuator intervention changes.

At minimum, the protocol must define:

- intervention levels/arms;
- control condition;
- trial order or randomization rule, if used;
- number of repeated trials;
- treatment of failed or invalid trials;
- environment isolation requirements;
- criteria for environmental contamination or interference.

The control condition must not secretly implement the hypothesized causal mechanism inside SIGADEFA.

---

## 9. Acceptance Criteria

H3 causal evidence requires all of the following:

1. **Identity:** the executed workload, code, protocol, and environment are provenance-verifiable.
2. **Integrity:** the workload builds and passes its correctness checks.
3. **Execution:** the registered workload actually executes under the registered actuator conditions.
4. **Independent measurement:** CPU allocation/resource usage is obtained independently of the gene-to-result hypothesis.
5. **Intervention effect:** changing the external actuator according to the registered intervention produces a measurable difference in CPU allocation consistent with the operating-system mechanism.
6. **Reproducibility:** the observed effect survives the registered repeated-trial protocol rather than appearing only in an isolated run.
7. **No programmed causality:** inspection of the implementation confirms that SIGADEFA does not directly encode the observed CPU relationship.

A correlation between `efficiency` and CPU without a valid external intervention is insufficient.

A successful actuator intervention without valid independent measurement is insufficient.

A measured difference produced by a workload whose computational effort itself changes with `efficiency` is invalid evidence.

---

## 10. Falsification Criteria

H3 is falsified for the tested execution environment/protocol if the pre-registered experiment satisfies its validity conditions but fails to demonstrate the registered causal effect under the specified intervention.

In particular, a valid null result must remain a null result. The mapping, workload, measurement procedure, or analysis must not be changed after observing the result solely to recover the expected effect.

The following are also falsification/invalid-evidence outcomes as applicable:

- the external actuator has no measurable causal effect under the registered conditions;
- the effect disappears under registered replication;
- the observed effect is attributable to workload changes rather than the external actuator;
- the measurement path is shown to derive its result from the gene or actuator instead of actual CPU/resource state;
- the experiment requires a forbidden in-process mechanism to produce the claimed result.

A failure caused by an unrelated infrastructure defect must be classified as **INCONCLUSIVE / EXECUTION FAILURE**, not automatically as H3 falsification.

---

## 11. Confounder Controls

The implementation protocol must account for at least:

- background CPU load;
- CPU affinity/topology where relevant;
- concurrent processes;
- thermal or power-management effects where relevant;
- container/VM resource limits;
- cgroup hierarchy and parent constraints;
- scheduler state;
- workload startup/warm-up effects;
- measurement overhead;
- CPU frequency variation;
- process placement and concurrency.

The protocol must record which controls are actually available in the execution environment rather than assuming that the environment is perfectly isolated.

---

## 12. Provenance and Evidence

Every H3 execution must be traceable to:

```text
canonical repository
    -> commit SHA
    -> protocol version
    -> workload identity
    -> execution environment
    -> actuator configuration
    -> raw measurement
    -> derived evidence
```

Chat messages, sandbox artifacts, copied logs, or remembered values are not authoritative substitutes for GitHub provenance.

The final causal claim must cite the exact executable commit and raw execution evidence from the canonical execution path.

---

## 13. Gate Semantics for H3

H3 must use the existing gate architecture without collapsing distinct claims:

```text
IDENTITY
   |
   v
INTEGRITY
   |
   v
EXECUTION
   |
   v
EVIDENCE
   |
   v
HYPOTHESIS
```

`QUALITY` remains advisory unless explicitly promoted by a separate approved contract.

A quality/conformance failure must not be reported as causal falsification.

An execution failure must not be reported as evidence for or against H3.

A successful execution is evidence only after the recorded measurement and provenance checks pass.

---

## 14. Implementation Boundary

This document authorizes design constraints only. It does **not** authorize silently modifying the frozen H2 implementation or tests.

The next implementation may add an independent execution harness, workload, resource-control adapter, and measurement path only if those components preserve the constraints in this document.

The first implementation must not claim H3 validity. It may establish only that the registered execution environment can perform the planned intervention and produce independently measurable evidence.

---

## 15. Approval State

**Current state:** PRE-REGISTERED DESIGN CANDIDATE — PENDING APPROVAL.

No causal conclusion is authorized by this document alone.

Approval of this document must precede the first H3 causal execution intended to generate hypothesis-level evidence.
