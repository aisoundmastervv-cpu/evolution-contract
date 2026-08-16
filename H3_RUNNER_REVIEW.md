# H3 Runner Review Boundary

Before a real H3 execution, review only implementation correctness against the preregistered `A2-cgroup-linear-v1` contract.

Do not change:

- registration commit `d7f32d45c181082e01e38b4cfb529f9eed8da18a`
- `cpu.weight` mapping
- primary endpoint
- five-plus-five trial protocol
- validity requirements
- 5-second measurement window
- acceptance thresholds
- outcome labels

A capability failure must produce `INCONCLUSIVE`, not a fallback to another execution arm.
