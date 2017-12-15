with open('06.input') as f:
    mem = list(map(int, f.read().strip().split()))

#mem = [0,2,7,0]

states = []
while tuple(mem) not in states:
    states.append(tuple(mem))
    high_index = mem.index(max(mem))
    high = mem[high_index]
    mem[high_index] = 0
    while high > 0:
        high_index = (high_index + 1) % len(mem)
        mem[high_index] += 1
        high -= 1

print(len(states))
print(len(states) - len(states[:states.index(tuple(mem))]))
