#!/usr/bin/env python

import re

columns = 50
rows = 6

screen = [[False] * columns for n in range(rows)]

f = open("d8.input", "r")


def rect(screen, columns, rows):
    for row in range(0, rows):
        for column in range(0, columns):
            screen[row][column] = True


def rotate(screen, column, number, steps):
    if column:
        ts = zip(*screen)
        ts[number] = ts[number][-steps:] + ts[number][:-steps]
        ts = zip(*ts)
        ts = map(list, ts)
        return ts
    else:
        screen[number] = screen[number][-steps:] + screen[number][:-steps]
        return screen


for instruction in f.readlines():
    if "rect" in instruction:
        columns, rows = re.findall(r'[\d]+', instruction)
        rect(screen, int(columns), int(rows))
    if "rotate" in instruction:
        number, steps = re.findall(r'[\d]+', instruction)
        screen = rotate(screen, True if 'column' in instruction else False, int(number), int(steps))

lit = 0

for row in screen:
    for pixel in row:
        if pixel:
            lit += 1

print(lit)
