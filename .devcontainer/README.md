# Cloud development workspace

This directory defines a reproducible, provider-neutral development container for `evolution-contract`.

## Purpose

The container provides the execution surface needed to inspect and validate the repository from a cloud development environment. It does **not** select a cloud provider, configure credentials, create resources, or deploy anything.

## Governance boundary

Opening this container is an operational setup step only. The first project-level actions after entering the workspace should remain read-only:

```text
pwd
git status --short --branch
git rev-parse HEAD
git remote -v
```

Cloud target discovery must establish facts before any target is selected:

```text
provider
→ account/project
→ region
→ existing non-production environment
→ environment/resource identity
```

No create, modify, deploy, or credential changes are part of this workspace definition.
