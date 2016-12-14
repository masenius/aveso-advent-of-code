#!/usr/bin/env python

f = open("d2.input", 'r')
pad = [[0, 0, 1, 0, 0], [0, 2, 3, 4, 0], [5, 6, 7, 8, 9], [0, "A", "B", "C", 0], [0, 0, "D", 0, 0]]
button = [1, 1]
code = ""


def move(button, direction):
    if direction == 'U':
        button[1] -= 1 if button[1] > 0 and pad[button[1] - 1][button[0]] != 0 else 0
    if direction == 'D':
        button[1] += 1 if button[1] < 4 and pad[button[1] + 1][button[0]] != 0 else 0
    if direction == 'L':
        button[0] -= 1 if button[0] > 0 and pad[button[1]][button[0] - 1] != 0 else 0
    if direction == 'R':
        button[0] += 1 if button[0] < 4 and pad[button[1]][button[0] + 1] != 0 else 0


for instruction in f.readlines():
    for direction in instruction:
        move(button, direction)
    code += str(pad[button[1]][button[0]])

print code
