#!/usr/bin/env bash
set -euo pipefail

EXPECTED_COMMIT="${1:-${EXPECTED_COMMIT:-}}"
shift || true

if [[ -z "$EXPECTED_COMMIT" ]]; then
  echo "EEC-002: NOT_READY" >&2
  echo "reason=exact-commit-required" >&2
  exit 2
fi

if [[ "${1:-}" != "--" ]]; then
  echo "EEC-002: NOT_READY" >&2
  echo "reason=mutation-command-required" >&2
  exit 2
fi
shift

if [[ "$#" -eq 0 ]]; then
  echo "EEC-002: NOT_READY" >&2
  echo "reason=mutation-command-empty" >&2
  exit 2
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$REPO_ROOT" ]]; then
  echo "EEC-002: NOT_READY" >&2
  echo "reason=repository-root-not-found" >&2
  exit 1
fi

cd "$REPO_ROOT"

if [[ -z "${EXPECTED_ENVIRONMENT_ID:-}" ]]; then
  echo "EEC-002: NOT_READY" >&2
  echo "reason=expected-environment-id-required" >&2
  exit 1
fi

EXPECTED_COMMIT="$EXPECTED_COMMIT" EXPECTED_ENVIRONMENT_ID="$EXPECTED_ENVIRONMENT_ID" bash "$REPO_ROOT/scripts/agent-environment-gate.sh"

echo "EEC-002: READY"
echo "mutation-entrypoint=agent-mutation-entrypoint.sh"
echo "mutation=AUTHORIZED"

exec "$@"
