#!/usr/bin/env python

pi = 3004953
elves = [1] * pi
to = 'N'

while elves.count(0) < pi - 1:
    for i in range(pi):
        if elves[i] and to =='N':
           to = i
        elif elves[i]:
            elves[to] += elves[i]
            elves[i] = 0
            to = 'N'

for i in range(pi):
    if elves[i]:
        print i + 1
