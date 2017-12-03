#!/usr/bin/env python

import re

compressed = open("d9.input", "r").readline()
decompressed = ""

marker = ""
i = 0

while i < len(compressed):
    if compressed[i] == ')':
        chars, repeats = re.findall(r'[\d]+', marker)
        marker = ""
        string = compressed[i + 1:i + int(chars) + 1] * int(repeats)
        decompressed = decompressed + string
        i = i + int(chars) + 1
    elif compressed[i] == '(' or marker:
        marker = marker + compressed[i]
        i = i + 1
    else:
        decompressed = decompressed + compressed[i]
        i = i + 1

print len(decompressed)
