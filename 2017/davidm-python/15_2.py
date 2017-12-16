a = 703
b = 516

a_fact = 16807
b_fact = 48271

div = 2147483647

def calc(prev, fact):
    return (prev * fact) % div

vals = 0
a_vals = []
b_vals = []
while vals < 5000000:
    a = calc(a, a_fact)
    b = calc(b, b_fact)
    if a % 4 == 0:
        a_vals.append(bin(a)[-16:])
    if b % 8 == 0:
        b_vals.append(bin(b)[-16:])
    vals = min([len(a_vals), len(b_vals)])

score = 0
for i in range(min([len(a_vals), len(b_vals)])):
    if a_vals[i] == b_vals[i]:
        score += 1

print(score)