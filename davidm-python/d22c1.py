#!/usr/bin/env python

import re
from collections import defaultdict

f = open('d22.input', 'r')
disks = defaultdict()

for line in f.readlines():
    if 'dev' in line:
        x, y, size, used, avail, perc = map(int, re.findall(r'[\d]+', line))
        disks[(x, y)] = (used, avail)
        
viable_pairs = 0
        
for disk_k, disk_v in disks.items():
    disk_x, disk_y = disk_k
    for k,v in disks.items():
        x, y = k
        if disk_x != x or disk_y != y:
            disk_used, disk_avail = disk_v
            used, avail = v
            if used and used < disk_avail:
                viable_pairs += 1
            
print viable_pairs
        