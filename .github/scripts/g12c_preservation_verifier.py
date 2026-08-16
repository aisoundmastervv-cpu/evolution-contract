#!/usr/bin/env python3
"""Fail-closed preservation verifier for the G12c experimental record.

This verifier intentionally does not reconstruct or synthesize G12c evidence.
A preservation PASS is possible only when the original immutable record and
its independent verification output are already present in the repository.
"""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path


RECORD = Path(".github/evidence/g12c/record.json")
VERIFICATION = Path(".github/evidence/g12c/verification.json")


def fail(message: str) -> "NoReturn":
    print(f"G12c-P DENY: {message}")
    raise SystemExit(1)


def load_json(path: Path) -> dict:
    if not path.is_file():
        fail(f"required artifact is absent: {path}")
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(f"cannot read valid JSON from {path}: {exc}")
    if not isinstance(value, dict):
        fail(f"artifact root must be an object: {path}")
    return value


def required(obj: dict, key: str, path: str) -> object:
    if key not in obj:
        fail(f"missing field {path}.{key}")
    return obj[key]


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    record = load_json(RECORD)
    verification = load_json(VERIFICATION)

    if required(record, "gate", "record") != "G12c":
        fail("record gate is not G12c")
    if required(record, "record_version", "record") != 1:
        fail("unsupported record version")
    if required(record, "evidence_status", "record") != "IMMUTABLE":
        fail("record is not explicitly marked IMMUTABLE")
    if required(record, "observed_outcome", "record") != "DENY":
        fail("preserved G12c outcome is not DENY")
    if required(record, "earned_capability", "record") is not None:
        fail("G12c must not grant a capability")

    provenance = required(record, "provenance", "record")
    if not isinstance(provenance, dict):
        fail("record.provenance must be an object")
    for key in ("repository", "commit_sha", "workflow_run_id"):
        value = required(provenance, key, "record.provenance")
        if not isinstance(value, str) or not value:
            fail(f"record.provenance.{key} must be a non-empty string")

    integrity = required(record, "integrity", "record")
    if not isinstance(integrity, dict):
        fail("record.integrity must be an object")
    record_sha = required(integrity, "record_sha256", "record.integrity")
    if not isinstance(record_sha, str) or len(record_sha) != 64:
        fail("record.integrity.record_sha256 must be a SHA-256 hex digest")

    actual_record_sha = sha256_file(RECORD)
    if actual_record_sha != record_sha:
        fail("record digest does not match the preserved record bytes")

    if required(verification, "gate", "verification") != "G12c":
        fail("verification gate is not G12c")
    if required(verification, "status", "verification") != "PASS":
        fail("independent verification is not PASS")
    if required(verification, "observed_outcome", "verification") != "DENY":
        fail("independent verifier did not observe DENY")
    if required(verification, "record_sha256", "verification") != record_sha:
        fail("verifier is not bound to the preserved record digest")

    print("G12c-P PASS")
    print("preserved immutable G12c evidence is present and independently bound by digest")
    print("earned_capability: NONE")


if __name__ == "__main__":
    main()
