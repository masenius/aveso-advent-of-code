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


def leftpad(string, length):
    return '0' * (length - len(string)) + string


def findAdjacent(x, y, grid):
    grid[y][x] = 'x'
    dirs = [(1, 0), (0, 1), (-1 , 0), (0, -1)]
    for dir in dirs:
        candidate = tuple(map(add, (x, y), dir))
        x_c, y_c = candidate
        if x_c >= 0 and x_c < 128 and y_c >= 0 and y_c < 128 and grid[y_c][x_c] == '1':
            findAdjacent(x_c, y_c, grid)


grid = []
for i in range(128):
    grid.append(list(leftpad(knothash('oundnydw-' + str(i)), 128)))

regions = 0
for y in range(128):
    for x in range(128):
        if grid[y][x] == '1':
            regions += 1
            findAdjacent(x, y, grid)


print(regions)

