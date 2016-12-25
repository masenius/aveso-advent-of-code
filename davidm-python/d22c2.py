#!/usr/bin/env python

import re
from collections import defaultdict
from collections import deque

unusable = []
visited = []
vdata = []
end = (0, 0)
empty = (0, 0)
data = (31, 0)


def finished(moves):
    for state in moves:
        empty, data = state
        if data == end:
            return True
    return False


def visited_data(data):
    h = hash(data)
    if h not in vdata:
        vdata.append(h)
        return False
    return True


def visited_state(state):
    h = hash(state)
    if h not in visited:
        visited.append(h)
        return False
    return True


def dist(pos1, pos2):
    x1, y1 = pos1
    x2, y2 = pos2
    return abs(x1 - x2) + abs(y1 - y2) / 2


def valid_moves(state):
    empty, data = state
    x, y = empty
    possible_moves = []

    if x > 0:
        possible_moves.append((x - 1, y))

    if x < 31:
        possible_moves.append((x + 1, y))

    if y > 0:
        possible_moves.append((x, y - 1))

    if y < 29:
        possible_moves.append((x, y + 1))

    valid_moves = []

    for move in possible_moves:
        if move[1] < unusable[0][1] and dist(move, data) > dist(empty, data) and dist(move, data) > 1:
            continue

        if move == data:
            if visited_data(empty) or dist(empty, end) >= dist(data, end):
                continue
            d = empty
        else:
            d = data

        if move not in unusable and not visited_state((move, d)):
            valid_moves.append((move, d))

    return valid_moves


f = open('d22.input', 'r')
disks = defaultdict()

for line in f.readlines():
    if 'dev' in line:
        x, y, size, used, avail, perc = map(int, re.findall(r'[\d]+', line))
        disks[(x, y)] = (used, avail)

for disk_k, disk_v in disks.items():
    disk_x, disk_y = disk_k
    disk_used, disk_avail = disk_v

    if disk_used:
        for k, v in disks.items():
            x, y = k
            if disk_x != x or disk_y != y:
                used, avail = v
                if used > disk_avail + disk_used:
                    unusable.append((x, y))
    else:
        empty = (disk_x, disk_y)

distance = 0
q = deque()
q.append((empty, data))

while True:
    moves = []
    distance += 1

    while len(q) > 0:
        moves = moves + valid_moves(q.popleft())

    if finished(moves):
        print distance
        break

    q.extend(moves)
