#!/usr/bin/env python3

import twinleaf

dev = twinleaf.Device()

# columns = ["accel.x", "accel.y", "accel.z"]
columns = []
for sample in dev.samples(5, columns):
       print(sample)
