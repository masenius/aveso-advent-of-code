from functools import reduce
import binascii
from operator import add

def knothash(string):
    lengths = list(map(ord, list(string))) + [17, 31, 73, 47, 23]
    list_ = list(range(256))
    current = 0
    skip = 0

    for _ in range(64):
        for length in lengths:
            l = list_[current:] + list_[0:current]
            l = list(reversed(l[0:length])) + l[length:]
            list_ = l[len(list_) - current:] + l[0:len(list_) - current]
            current = (current + length + skip) % len(list_)
            skip += 1

    r = []
    for i in range(int(len(list_)/16)):
        r.append(reduce(lambda x, y: x ^ y, list_[i * 16:(i + 1) * 16]))

    return str(bin(int(binascii.hexlify(bytes(bytearray(r))), 16)))[2:]


sum = 0
for i in range(128):
    sum += knothash('oundnydw-' + str(i)).count('1')

print(sum)