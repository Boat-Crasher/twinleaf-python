#!/usr/bin/env python3

import twinleaf

dev = twinleaf.Device()

meta = dev._get_metadata()
import pprint
pp = pprint.PrettyPrinter(indent=1)
pp.pprint(meta)
