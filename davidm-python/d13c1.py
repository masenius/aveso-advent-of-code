#!/usr/bin/env python

from collections import deque

favorite_number = 1362
start = (1, 1)
goal = (31, 39)
distance = 0
queue = deque()
visited = []


def unvisited(move):
    if move in visited:
        return False
    visited.append(move)
    return True


def valid_move(pos):
    x, y = pos
    if x >= 0 and y >= 0:
        calc = format(x * x + 3 * x + 2 * x * y + y + y * y + favorite_number, 'b')
        if calc.count('1') % 2 == 0:
            return True
    return False


def calc_moves(pos):
    x, y = pos
    possible_moves = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
    valid_moves = []

    for move in possible_moves:
        if valid_move(move) and unvisited(move):
            valid_moves.append(move)

    return valid_moves


queue.append(start)

while True:
    distance += 1
    moves = []

    while len(queue) > 0:
        moves = moves + calc_moves(queue.popleft())

    if goal in moves:
        print distance
        break

    queue.extend(moves)
