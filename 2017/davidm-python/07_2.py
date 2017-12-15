from collections import defaultdict
import re

weight = defaultdict()
references = defaultdict()

with open('07.input') as f:
    for line in f.readlines():
        words = re.findall(r'[a-z]+', line)
        we = int(re.findall(r'\d+', line)[0])
        key = words[0]
        refs = words[1:]
        weight[key] = we
        references[key] = refs

def getWeigths(ref):
    w = weight[ref]
    for r in references[ref]:
        w += getWeigths(r)
    return w

def getError(ref):
    d = defaultdict(list)
    for r in references[ref]:
        d[getWeigths(r)].append(r)
    for k in d:
        if len(d[k]) == 1:
            return getError(d[k][0])
    return ref

r = getError('mwzaxaj')
print(weight[r], getWeigths(r))
