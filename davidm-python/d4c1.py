#!/usr/bin/env python

from collections import Counter
import re
import operator

f = open("d4.input", 'r')
id_sum = 0

for room in f.readlines():
    room = re.findall(r"[\w]+", room)
    name = room[:-2]
    name = reduce(operator.add, name)
    room_id = room[-2]
    checksum = room[-1]
    check_list = Counter(name).most_common()

    # Sort equals alphabetically
    calculated_checksum = ""
    equals = ""
    i = 0
    while i < len(check_list):
        equals = equals + str(check_list[i][0])
        if i + 1 == len(check_list) or check_list[i][1] != check_list[i + 1][1]:
            calculated_checksum = calculated_checksum + ''.join(sorted(equals))
            equals = ""
        i += 1

    if calculated_checksum[:5] == checksum:
        id_sum += int(room_id)

print id_sum
