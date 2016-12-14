#!/usr/bin/env python

f = open("d1.input", "r")

moves = f.readline().split(", ")
x = 0
y = 0
direction = [0, 1]


def rotate(direction, turn):
    change = (turn == 'R' and direction[0] != 0) or (turn == 'L' and direction[1] != 0)
    return [direction[1] * (-1 if change else 1), direction[0] * (-1 if change else 1)]


for move in moves:
    direction = rotate(direction, move[0])
    x = x + direction[0] * int(move[1:])
    y = y + direction[1] * int(move[1:])

print abs(x) + abs(y)
