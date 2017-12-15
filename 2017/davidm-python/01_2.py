with open('01.input') as f:
    input = f.read()

result = 0
for i in range(len(input)):
    num = int(input[i])
    if num == int((input[i:] + input[:i])[int(len(input)/2)]):
        result += num

print(result)
