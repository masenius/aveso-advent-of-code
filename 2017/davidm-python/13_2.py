from collections import defaultdict

input = defaultdict(int)
with open('13.input') as f:
    for line in f.readlines():
        key, val = map(lambda x: int(x.strip()), line.strip().split(':'))
        input[key] = val

def caught(delay):
    for ps in input.keys():
        if (ps + delay) % (input[ps] * 2 - 2) == 0:
            return True
    return False

i = 0
while caught(i):
    i += 1

print(i)
