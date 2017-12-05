with open('05.input') as f:
    input = list(map(int, f.read().split('\n')))

index = 0
steps = 0
while index < len(input):
    i = index
    index += input[i]
    if input[i] > 2:
        input[i] -= 1
    else:
        input[i] += 1
    steps += 1

print(steps)