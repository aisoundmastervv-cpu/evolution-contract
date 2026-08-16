#!/usr/bin/env python3
import json
import statistics
import sys
from pathlib import Path

REGISTRATION_COMMIT = "d7f32d45c181082e01e38b4cfb529f9eed8da18a"
MAPPING_ID = "A2-cgroup-linear-v1"


def verdict(trials):
    invalid = [t for t in trials if not t.get("valid")]
    if invalid:
        return "INCONCLUSIVE", {"reason": "invalid_trials", "invalid_trial_ids": [t["trial_id"] for t in invalid]}

    r0 = [t["R"] for t in trials if t["efficiency"] == 0]
    r255 = [t["R"] for t in trials if t["efficiency"] == 255]
    if len(r0) != 5 or len(r255) != 5:
        return "INCONCLUSIVE", {"reason": "wrong_trial_count", "count_e0": len(r0), "count_e255": len(r255)}

    median_r0 = statistics.median(r0)
    median_r255 = statistics.median(r255)

    if not (0.5 <= median_r0 <= 2.0):
        return "INCONCLUSIVE", {"reason": "baseline_sensitivity_out_of_registered_range", "median_R0": median_r0, "median_R255": median_r255}

    if median_r255 >= 2.0 * median_r0:
        return "CAUSAL SUPPORT", {"median_R0": median_r0, "median_R255": median_r255, "effect_ratio": median_r255 / median_r0}

    return "NULL-FALSIFICATION", {"median_R0": median_r0, "median_R255": median_r255, "effect_ratio": median_r255 / median_r0}


def main(path):
    data = json.loads(Path(path).read_text())
    if data.get("registration_commit") != REGISTRATION_COMMIT or data.get("mapping_id") != MAPPING_ID:
        raise SystemExit("registration identity mismatch")
    v, details = verdict(data["trials"])
    data["verdict"] = v
    data["evaluation"] = details
    Path(path).write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")
    print(f"H3_VERDICT: {v}")
    print(json.dumps(details, sort_keys=True))
    return 0 if v != "INCONCLUSIVE" else 2

if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1]))
