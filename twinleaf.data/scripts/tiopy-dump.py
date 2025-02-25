#!/usr/bin/env python3

import twinleaf
import IPython

dev = twinleaf.Device()

#dev.scan_rpcs()
#IPython.embed()

for sample in dev.samples(5):
       print(sample)
