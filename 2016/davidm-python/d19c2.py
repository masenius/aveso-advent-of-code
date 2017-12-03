#!/usr/bin/env python

pi = 3004953
elves = range(1, pi + 1)
to = 'N'

while len(elves) > 1:
    i = 0
    while i < len(elves):
        opposite = i + len(elves)/2
        if opposite >= len(elves):
            opposite -= len(elves)
        else:
            i += 1
        elves.pop(opposite)

print elves[0]