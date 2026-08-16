# Architecture Boundary

SIGADEFA Ω and G-System are separate projects with a narrow evidence/capability interface.

```text
G-System                         SIGADEFA Ω
 evidence / gates  <-------->    hypotheses / experiments
 capabilities                    fitness / selection
 autonomy                        evolution
```

SIGADEFA Ω does not implement the capability system.
G-System does not implement the evolution mathematics.

The boundary is intentional: experimental results become evidence; the external control plane may later use that evidence to grant bounded capabilities.
