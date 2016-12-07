#!/usr/bin/env python

f = open("d1.input", "r")

moves = f.readline().split(", ")
x = 0
y = 0
direction = [0, 1]
history = []
done = False

def rotate(direction, turn):
    change = (turn == 'R' and direction[0] != 0) or (turn == 'L' and direction[1] != 0)
    return [direction[1] * (-1 if change else 1), direction[0] * (-1 if change else 1)]
    
for move in moves:
    direction = rotate(direction, move[0])
    distance = int(move[1:])
    for step in range(0, distance):
        x = x + direction[0]
        y = y + direction[1]
        loc_str = str(x) + "," + str(y)
        if loc_str in history:
            done = True
            break;
        history.append(loc_str)
    if done:
        break;
    
    
print abs(x) + abs(y)
