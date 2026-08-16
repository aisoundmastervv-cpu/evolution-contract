#!/usr/bin/env bash
set -euo pipefail

EXPECTED_COMMIT="${1:-${EXPECTED_COMMIT:-}}"
EXPECTED_ENVIRONMENT_ID="${EXPECTED_ENVIRONMENT_ID:-}"
CANONICAL_REPOSITORY="aisoundmastervv-cpu/evolution-contract"
CANONICAL_REPOSITORY_ID="1335431893"

if [[ -z "$EXPECTED_COMMIT" ]]; then
  echo "AGENT_ENVIRONMENT: NOT_READY"
  echo "reason=exact-commit-required" >&2
  exit 2
fi

REPO_ROOT="${REPO_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || true)}"
if [[ -z "$REPO_ROOT" || ! -d "$REPO_ROOT/.git" ]]; then
  echo "AGENT_ENVIRONMENT: NOT_READY"
  echo "reason=repository-root-not-found" >&2
  exit 1
fi

cd "$REPO_ROOT"

REMOTE_URL="$(git remote get-url origin 2>/dev/null || true)"
case "$REMOTE_URL" in
  "https://github.com/$CANONICAL_REPOSITORY"|"https://github.com/$CANONICAL_REPOSITORY.git"|"git@github.com:$CANONICAL_REPOSITORY.git") ;;
  *)
    echo "AGENT_ENVIRONMENT: NOT_READY"
    echo "reason=canonical-repository-mismatch remote=$REMOTE_URL" >&2
    exit 1
    ;;
esac

if [[ ! -f .project/IDENTITY.md ]]; then
  echo "AGENT_ENVIRONMENT: NOT_READY"
  echo "reason=canonical-identity-missing" >&2
  exit 1
fi

grep -Fq "Full name: \`$CANONICAL_REPOSITORY\`" .project/IDENTITY.md
grep -Fq "GitHub repository ID: \`$CANONICAL_REPOSITORY_ID\`" .project/IDENTITY.md
grep -Fq "Repository is the sole canonical GitHub source of truth" .project/IDENTITY.md

ACTUAL_COMMIT="$(git rev-parse HEAD)"
if [[ "$ACTUAL_COMMIT" != "$EXPECTED_COMMIT" ]]; then
  echo "AGENT_ENVIRONMENT: NOT_READY"
  echo "reason=commit-mismatch expected=$EXPECTED_COMMIT actual=$ACTUAL_COMMIT" >&2
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "AGENT_ENVIRONMENT: NOT_READY"
  echo "reason=workspace-not-clean" >&2
  exit 1
fi

EXPECTED_COMMIT="$EXPECTED_COMMIT" bash scripts/environment-preflight.sh

if [[ -n "$EXPECTED_ENVIRONMENT_ID" ]]; then
  CONTRACT_SHA256="$(sha256sum "$REPO_ROOT/docs/execution/environment-contract-v0.1.md" | awk '{print $1}')"
  BOOTSTRAP_SHA256="$(sha256sum "$REPO_ROOT/scripts/bootstrap-environment.sh" | awk '{print $1}')"
  RUSTC_VERSION="$(rustc --version)"
  CARGO_VERSION="$(cargo --version)"
  IDENTITY_INPUT=$(printf '%s\n' \
    "repository=$REMOTE_URL" \
    "commit=$ACTUAL_COMMIT" \
    "environment_contract_sha256=$CONTRACT_SHA256" \
    "bootstrap_sha256=$BOOTSTRAP_SHA256" \
    "rustc=$RUSTC_VERSION" \
    "cargo=$CARGO_VERSION")
  ACTUAL_ENVIRONMENT_ID="$(printf '%s' "$IDENTITY_INPUT" | sha256sum | awk '{print $1}')"
  if [[ "$ACTUAL_ENVIRONMENT_ID" != "$EXPECTED_ENVIRONMENT_ID" ]]; then
    echo "AGENT_ENVIRONMENT: NOT_READY"
    echo "reason=environment-identity-mismatch expected=$EXPECTED_ENVIRONMENT_ID actual=$ACTUAL_ENVIRONMENT_ID" >&2
    exit 1
  fi
fi

echo "AGENT_ENVIRONMENT: READY"
echo "repository=$CANONICAL_REPOSITORY"
echo "repository_id=$CANONICAL_REPOSITORY_ID"
echo "commit=$ACTUAL_COMMIT"
if [[ -n "$EXPECTED_ENVIRONMENT_ID" ]]; then
  echo "environment_id=$EXPECTED_ENVIRONMENT_ID"
fi
