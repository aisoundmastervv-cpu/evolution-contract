#!/usr/bin/env python3
"""Frozen CPU-demand workload for H3 A2 trials.

The workload has no efficiency input and performs the same deterministic
computation for every trial and condition.
"""
import hashlib
import signal
import sys

running = True

def stop(_signum, _frame):
    global running
    running = False

signal.signal(signal.SIGTERM, stop)
signal.signal(signal.SIGINT, stop)

payload = b"evolution-contract-h3-frozen-workload-v1" * 1024
state = b"seed"
iterations = 0

# Pause before execution so the parent can attach this process to its
# registered cgroup and CPU affinity before releasing both workloads.
signal.pause()

while running:
    state = hashlib.sha256(state + payload).digest()
    iterations += 1

if iterations <= 0:
    print("H3_WORKLOAD: no iterations", file=sys.stderr)
    sys.exit(2)

print(f"iterations={iterations}")
