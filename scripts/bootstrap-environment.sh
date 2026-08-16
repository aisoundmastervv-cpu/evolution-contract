#!/usr/bin/env bash
set -euo pipefail

REPOSITORY_URL="${1:-$(git remote get-url origin 2>/dev/null || true)}"
EXPECTED_COMMIT="${2:-$(git rev-parse HEAD 2>/dev/null || true)}"
TARGET_DIR="${3:-$(mktemp -d -t evolution-contract-env.XXXXXX)}"

if [[ -z "$REPOSITORY_URL" || -z "$EXPECTED_COMMIT" ]]; then
  echo "BOOTSTRAP: NOT_READY" >&2
  echo "usage: $0 <repository-url> <commit-sha> [target-dir]" >&2
  exit 2
fi

if [[ -e "$TARGET_DIR" && -n "$(find "$TARGET_DIR" -mindepth 1 -maxdepth 1 -print -quit)" ]]; then
  echo "BOOTSTRAP: NOT_READY" >&2
  echo "reason=target-dir-not-empty" >&2
  exit 1
fi

mkdir -p "$TARGET_DIR"
git -C "$TARGET_DIR" init -q
git -C "$TARGET_DIR" remote add origin "$REPOSITORY_URL"
git -C "$TARGET_DIR" fetch --quiet --depth 1 origin "$EXPECTED_COMMIT"
git -C "$TARGET_DIR" checkout --quiet --detach "$EXPECTED_COMMIT"

ACTUAL_COMMIT="$(git -C "$TARGET_DIR" rev-parse HEAD)"
if [[ "$ACTUAL_COMMIT" != "$EXPECTED_COMMIT" ]]; then
  echo "BOOTSTRAP: NOT_READY" >&2
  echo "reason=commit-mismatch expected=$EXPECTED_COMMIT actual=$ACTUAL_COMMIT" >&2
  exit 1
fi

REPO_ROOT="$TARGET_DIR" EXPECTED_COMMIT="$EXPECTED_COMMIT" bash "$TARGET_DIR/scripts/environment-preflight.sh"

CONTRACT_SHA256="$(sha256sum "$TARGET_DIR/docs/execution/environment-contract-v0.1.md" | awk '{print $1}')"
BOOTSTRAP_SHA256="$(sha256sum "$TARGET_DIR/scripts/bootstrap-environment.sh" | awk '{print $1}')"
RUSTC_VERSION="$(rustc --version)"
CARGO_VERSION="$(cargo --version)"
REPOSITORY_CANONICAL="$(git -C "$TARGET_DIR" remote get-url origin)"

IDENTITY_INPUT=$(printf '%s\n' \
  "repository=$REPOSITORY_CANONICAL" \
  "commit=$ACTUAL_COMMIT" \
  "environment_contract_sha256=$CONTRACT_SHA256" \
  "bootstrap_sha256=$BOOTSTRAP_SHA256" \
  "rustc=$RUSTC_VERSION" \
  "cargo=$CARGO_VERSION")
ENVIRONMENT_ID="$(printf '%s' "$IDENTITY_INPUT" | sha256sum | awk '{print $1}')"

cat > "$TARGET_DIR/environment-identity.json" <<EOF
{
  "environment_id": "$ENVIRONMENT_ID",
  "repository": "$REPOSITORY_CANONICAL",
  "commit": "$ACTUAL_COMMIT",
  "environment_contract_sha256": "$CONTRACT_SHA256",
  "bootstrap_sha256": "$BOOTSTRAP_SHA256",
  "rustc": "$RUSTC_VERSION",
  "cargo": "$CARGO_VERSION",
  "workspace_path": "$TARGET_DIR"
}
EOF

printf '%s\n' "$ENVIRONMENT_ID"
echo "BOOTSTRAP: READY"
echo "workspace=$TARGET_DIR"
echo "environment_id=$ENVIRONMENT_ID"
