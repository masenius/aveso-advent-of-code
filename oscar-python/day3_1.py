import csv

with open("day3_input.txt", "r") as fobj:
    values = []
    reader = csv.reader(fobj, delimiter=" ")
    num = 0
    for line in reader:
        vals = [int(val) for val in line
                if val.isdigit()]
        values.append(vals)
        num += 1

num_valid = 0
for triplet in values:
    a, b, c = sorted(triplet)
    if a + b > c:
        num_valid += 1

print(num_valid, num)
