#!/usr/bin/env python

import re

f = open("d15.input", "r")

discs = []

for line in f.readlines():
    positions = re.findall(r'(\d+)\spos', line)
    start = re.findall(r'(\d+)\.', line)
    discs.append((int(positions[0]), int(start[0])))


time = 0
while True:
    ok = True

    time += 1

    i = 1
    for disc in discs:
        positions, start = disc
        start += (time + i) % positions
        if start >= positions:
            start -= positions
        if start != 0:
            ok = False
        i += 1

    if ok:
        print time
        break
