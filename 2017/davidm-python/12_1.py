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

getChilds('0', childs)
print(len(childs))