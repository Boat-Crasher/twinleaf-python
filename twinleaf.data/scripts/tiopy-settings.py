#!/usr/bin/env python3

import twinleaf

dev = twinleaf.Device()

settings = dev.settings()
import pprint
pp = pprint.PrettyPrinter(indent=1)
pp.pprint(settings)
