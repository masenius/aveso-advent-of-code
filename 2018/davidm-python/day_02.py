from collections import Counter
from itertools import permutations

from aocd import data


def part_1(data):
    twos = 0
    threes = 0
    for input in data:
        counter = Counter(input)
        for occurency in set(counter.values()):
            if occurency == 2:
                twos += 1
            if occurency == 3:
                threes += 1
    return twos * threes


def part_2(data):
    most_chars = []
    for a, b in permutations(data, 2):
        new_chars = [a[i] for i in range(len(a)) if a[i] == b[i]]
        if len(new_chars) > len(most_chars):
            most_chars = new_chars
    return most_chars


if __name__ == '__main__':
    data = data.split()
    print(f"Part 1: {part_1(data)}")
    print(f"Part 2: {''.join(part_2(data))}")
