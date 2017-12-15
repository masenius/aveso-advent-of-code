with open('01.input') as f:
    input = f.read()

current = int(input[0])
result = 0
num = 0
for num in input[1:] + input[0]:
    num = int(num)
    if current == num:
        result += num
    else:
        current = num

print(result)
