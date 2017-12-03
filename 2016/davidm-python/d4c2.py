#!/usr/bin/env python

from collections import Counter
import re
import operator


def shift_cipher(name, steps):
    steps = room_id % 26
    text = ""

    for word in name:
        for char in word:
            char = ord(char) + steps
            if char > 122:
                char -= 26
            text = text + chr(char)
        text = text + " "

    return text


f = open("d4.input", 'r')
id_sum = 0

for room in f.readlines():
    room = re.findall(r"[\w]+", room)
    name = room[:-2]
    room_id = int(room[-2])
    checksum = room[-1]

    check_list = Counter(reduce(operator.add, name)).most_common()

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
        actual_room_name = shift_cipher(name, room_id)
        if "northpole" in actual_room_name:
            print actual_room_name, room_id
