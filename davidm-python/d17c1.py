#!/usr/bin/env python

from collections import deque
from hashlib import md5

pi = 'bwnlcvfs'


def finished(moves):
    for move in moves:
        x, y, path = move
        if x == 3 and y == 3:
            print path
            return True
    return False


def get_valid_moves(pos):
    x, y, path = pos
    valid_moves = []

    h = md5(pi + path).hexdigest()[:4]

    if int(h[0], 16) > 0xa and y > 0:
        valid_moves.append((x, y - 1, path + 'U'))
    if int(h[1], 16) > 0xa and y < 3:
        valid_moves.append((x, y + 1, path + 'D'))
    if int(h[2], 16) > 0xa and x > 0:
        valid_moves.append((x - 1, y, path + 'L'))
    if int(h[3], 16) > 0xa and x < 3:
        valid_moves.append((x + 1, y, path + 'R'))

    return valid_moves


start = (0, 0, "")
q = deque()

q.append(start)

while True:
    moves = []

    while len(q) > 0:
        new_moves = get_valid_moves(q.popleft())
        if new_moves:
            moves = moves + new_moves

    q.extend(moves)

    if finished(moves):
        break
