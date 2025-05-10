#!/usr/bin/env python3

import twinleaf

dev = twinleaf.Device()

# columns = []
columns = ["imu.accel"]
# columns = ["imu.accel.x", "imu.accel.y", "imu.accel.z"]
for sample in dev._samples(5, columns):
       print(sample)
