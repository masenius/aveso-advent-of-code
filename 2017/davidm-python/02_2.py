import re

def get_divisable(d):
    for a in range(len(d)):
        for b in range(len(d)):
            r = d[a] / d[b]
            if a != b and int(r) == r:
                return int(r)


sum = 0
with open('02.input') as f:
    for line in f.readlines():
        d = [int(x) for x in re.findall('\d+', line)]
        sum += get_divisable(d)

print(sum)