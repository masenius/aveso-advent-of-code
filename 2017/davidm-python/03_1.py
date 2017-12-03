import operator

input = 289326
path = []
c = (0,0)
d = (0,0)
w = (1,0)
left_turn = [(1,0),(0,-1),(-1,0),(0,1)]

def turn(a):
    return left_turn[(left_turn.index(a) + 1) % len(left_turn)]

def add(a, b):
    return tuple(map(operator.add, a, b))

for i in range(input):
    path.append(c)
    if add(w, c) not in path:
        d = w
        w = turn(d)
    c = add(c, d)
    if i % 1000 == 0:
        print(i)

print(c, abs(c[0]) + abs(c[1]) - 1)
