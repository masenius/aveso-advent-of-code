from itertools import chain

d = {}

with open('12.input') as f:
    for line in f.readlines():
        key, values = line.strip().split("<->")
        d[key.strip()] = list(map(lambda x: x.strip(), values.split(",")))

childs = []

def getChilds(key, childs):
    for k in d[key]:
        if k not in childs:
            childs.append(k)
            getChilds(k, childs)

groups = 0
for key in d.keys():
    if key not in childs:
        getChilds(key, childs)
        groups += 1

print(groups)
