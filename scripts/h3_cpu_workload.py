#!/usr/bin/env python3
"""Frozen H3 CPU-demand workload.

The workload has no efficiency input and performs the same deterministic
instruction sequence until the registered execution window ends.
"""

import signal

running = True


def stop(_signum, _frame):
    global running
    running = False


signal.signal(signal.SIGTERM, stop)
signal.signal(signal.SIGINT, stop)

state = 0x12345678
while running:
    for i in range(10_000):
        state = (state * 1_664_525 + 1_013_904_223 + i) & 0xFFFFFFFF

print(f"checksum={state:08x}")
