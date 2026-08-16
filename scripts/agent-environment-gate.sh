#!/usr/bin/env bash
set -euo pipefail

EXPECTED_COMMIT="${1:-${EXPECTED_COMMIT:-}}"
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

echo "AGENT_ENVIRONMENT: READY"
echo "repository=$CANONICAL_REPOSITORY"
echo "repository_id=$CANONICAL_REPOSITORY_ID"
echo "commit=$ACTUAL_COMMIT"
