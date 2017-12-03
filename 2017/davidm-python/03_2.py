import operator
from collections import defaultdict

input = 289326
path = defaultdict(list)
c = (0,0)
d = (0,0)
w = (1,0)
left_turn = [(1,0),(0,-1),(-1,0),(0,1)]
adjacents = [(1,0),(0,-1),(-1,0),(0,1), (1,1), (1,-1), (-1, 1), (-1, -1)]

def turn(a):
    return left_turn[(left_turn.index(a) + 1) % len(left_turn)]

def add(a, b):
    return tuple(map(operator.add, a, b))

for i in range(input):
    sum = 0
    for grid in adjacents:
        if add(c, grid) in path:
           sum += path[add(c, grid)]
    path[c] = sum if sum != 0 else 1
    if add(w, c) not in path:
        d = w
        w = turn(d)
    c = add(c, d)
    if sum > input:
        print(sum, i)
        break
