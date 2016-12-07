#!/usr/bin/env python

from collections import Counter

f = open("d6.input", 'r')
message = [""]*8
result = ""

for line in f.readlines():
    for i in range(0,8):
        message[i] = message[i] + line[i]
    
for column in message:
    result = result + Counter(column).most_common(1)[0][0]
    
print result