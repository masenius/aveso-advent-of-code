from collections import defaultdict

d = defaultdict(int)
instructions = []

with open('08.input') as f:
    for line in f.readlines():
        instructions.append(line.strip().split())
        d[instructions[-1][0]] = 0

def run(ins):
    if '==' in ins:
        return d[instruction[4]] == int(instruction[6])
    elif '!=' in ins:
        return d[instruction[4]] != int(instruction[6])
    elif '>=' in ins:
        return d[instruction[4]] >= int(instruction[6])
    elif '<=' in ins:
        return d[instruction[4]] <= int(instruction[6])
    elif '<' in ins:
        return d[instruction[4]] < int(instruction[6])
    elif '>' in ins:
        return d[instruction[4]] > int(instruction[6])

x = 0
for instruction in instructions:
    if run(instruction):
        if instruction[1] == 'inc':
            d[instruction[0]] += int(instruction[2])
        else:
            d[instruction[0]] -= int(instruction[2])
    x = max(d.values()) if max(d.values()) > x else x

print(max(d.values()))
print(x)
