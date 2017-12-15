from collections import defaultdict

input = defaultdict(int)
with open('13.input') as f:
    for line in f.readlines():
        key, val = map(lambda x: int(x.strip()), line.strip().split(':'))
        input[key] = val

d = 0
for ps in input.keys():
    if input[ps] and ps % (input[ps] * 2 - 2) == 0:
        d += ps * input[ps]

print(d)
