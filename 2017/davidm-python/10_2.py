from functools import reduce
import binascii

input = '206,63,255,131,65,80,238,157,254,24,133,2,16,0,1,3'
lengths = list(map(ord, list(input))) + [17, 31, 73, 47, 23]
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

r = binascii.hexlify(bytes(bytearray(r)))

print(r)
