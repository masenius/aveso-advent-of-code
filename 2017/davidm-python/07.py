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
        try:
            references[key]
        except:
            references[key] = None
        for ref in refs:
            references[ref]= key

for k in references:
    if references[k] is None:
        print(k)
