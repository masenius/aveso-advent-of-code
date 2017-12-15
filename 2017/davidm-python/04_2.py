from collections import Counter

def is_valid(pwd):
    pwd = map(lambda x: ''.join(sorted(x)), pwd.split())
    for val in Counter(pwd).values():
        if val > 1:
            return False
    return True

valids = 0
with open('04.input') as f:
    for pwd in f.readlines():
        if is_valid(pwd.strip()):
            valids += 1

print(valids)