#!/usr/bin/env python


def is_trap(pos, previous_row):
    left, center, right = previous_row[pos - 1:pos + 2]
    if left & center & ~right or ~left & center & right or left & ~center & ~right or ~left & ~center & right:
        return True
    return False


previous_row = [0]

for char in open('d18.input', 'r').readline():
    previous_row.append(1) if char is '^' else previous_row.append(0)
previous_row.append(0)

safe = previous_row.count(0) - 2

for row in range(39):
    new_row = [0]
    pos = 1
    while pos < len(previous_row) - 1:
        new_row.append(1) if is_trap(pos, previous_row) else new_row.append(0)
        pos += 1
    new_row.append(0)

    safe += new_row.count(0) - 2

    previous_row = new_row

print safe
