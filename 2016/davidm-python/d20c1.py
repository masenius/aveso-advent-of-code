#!/usr/bin/env python

intervals = []
f = open('d20.input', 'r')

for line in f.readlines():
    fr, to = map(int, line.split('-'))
    intervals.append((fr, to))
    
intervals.sort()

merged_intervals = []

for i in range(len(intervals)):
    fr_new, to_new = intervals[i]
    
    if not merged_intervals:
        merged_intervals.append((fr_new, to_new))
        continue
    
    fr, to = merged_intervals[-1]
    
    if to_new <= to:
        continue
    
    if fr_new <= to + 1:
        merged_intervals[-1] = (fr, to_new)
    else:
        merged_intervals.append((fr_new, to_new))
    
print merged_intervals[0][1] + 1
