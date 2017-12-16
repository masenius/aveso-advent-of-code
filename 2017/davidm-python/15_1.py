a = 703
b = 516


a_fact = 16807
b_fact = 48271

div = 2147483647

def calc(prev, fact):
    return (prev * fact) % div

score = 0
for _ in range(40000000):
    a = calc(a, a_fact)
    b = calc(b, b_fact)
    if bin(a)[-16:] == bin(b)[-16:]:
        score += 1

print(score)