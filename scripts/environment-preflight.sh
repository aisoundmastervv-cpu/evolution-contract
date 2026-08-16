#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${REPO_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || true)}"
if [[ -z "$REPO_ROOT" || ! -d "$REPO_ROOT/.git" ]]; then
  echo "ENVIRONMENT: NOT_READY"
  echo "reason=repository-root-not-found" >&2
  exit 1
fi

cd "$REPO_ROOT"

require_command() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "ENVIRONMENT: NOT_READY"
    echo "reason=missing-command:$1" >&2
    exit 1
  }
}

for command in git bash sha256sum rustc cargo; do
  require_command "$command"
done

EXPECTED_COMMIT="${EXPECTED_COMMIT:-}"
ACTUAL_COMMIT="$(git rev-parse HEAD)"
if [[ -n "$EXPECTED_COMMIT" && "$ACTUAL_COMMIT" != "$EXPECTED_COMMIT" ]]; then
  echo "ENVIRONMENT: NOT_READY"
  echo "reason=commit-mismatch expected=$EXPECTED_COMMIT actual=$ACTUAL_COMMIT" >&2
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "ENVIRONMENT: NOT_READY"
  echo "reason=workspace-not-clean" >&2
  exit 1
fi

if [[ ! -f docs/execution/environment-contract-v0.1.md ]]; then
  echo "ENVIRONMENT: NOT_READY"
  echo "reason=environment-contract-missing" >&2
  exit 1
fi

cargo fmt --all -- --check
cargo check --all-targets

echo "ENVIRONMENT: READY"
echo "repository=$(git remote get-url origin 2>/dev/null || printf '%s' local)"
echo "commit=$ACTUAL_COMMIT"
echo "rustc=$(rustc --version)"
echo "cargo=$(cargo --version)"
