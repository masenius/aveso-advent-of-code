from aocd import data
from itertools import cycle


def part_1(data):
    return sum(data)


def part_2(data):
    freq = 0
    freqs = {freq}
    for change in cycle(data):
        freq += change
        if freq in freqs:
            return freq
        else:
            freqs.add(freq)


if __name__ == '__main__':
    data = [int(i) for i in data.split('\n')]
    print(f"Part 1: {part_1(data)}")
    print(f"Part 2: {part_2(data)}")
