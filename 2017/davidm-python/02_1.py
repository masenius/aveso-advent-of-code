import re

sum = 0
with open('02.input') as f:
    for line in f.readlines():
        d = [int(x) for x in re.findall('\d+', line)]
        sum += max(d) - min(d)

print(sum)