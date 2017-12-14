from operator import add

def c(d):
    if d == 'n':
        return (0, 1, -1)
    elif d == 'ne':
        return (1, 0, -1)
    elif d == 'se':
        return (1, -1, 0)
    elif d == 's':
        return (0, -1, 1)
    elif d == 'sw':
        return (-1, 0, 1)
    elif d == 'nw':
        return (-1, 1, 0)

def d(a, b):
    return int((abs(a[0] - b[0]) + abs(a[1] - b[1]) + abs(a[2] - b[2])) / 2)

with open('11.input') as f:
    input = list(map(c, f.read().split(',')))

start = (0, 0 ,0)
current = (0, 0, 0)
max = 0
for step in input:
    current = tuple(map(add, current, step))
    if d(current, start) > max:
        max = d(current, (0, 0, 0))

print(d(current, start))
print(max)