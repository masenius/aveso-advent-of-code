import re
from collections import Counter

from aocd import data


def generate_coords(claim):
    return [(x, y) for x in range(claim[1], claim[1] + claim[3]) for y in range(claim[2], claim[2] + claim[4])], claim[0]


def part_1(data):
    coords = [coord for coords, id_ in [generate_coords(claim) for claim in data] for coord in coords]
    return len(list(filter(lambda x: x > 1, Counter(coords).values())))


def part_2(data):
    claims = {}
    all_coords = []
    for claim in data:
        coords, id_ = generate_coords(claim)
        claims[id_] = coords
        all_coords += coords
    overlaps = list(map(lambda x: x[0], list(filter(lambda x: x[1] > 1, Counter(all_coords).items()))))
    for id_, coords in claims.items():
        if len(list(set(overlaps) & set(coords))) == 0:
            return id_


if __name__ == '__main__':
    data = [tuple(map(int, re.findall('\d+', input))) for input in data.split('\n')]
    print(f"Part 1: {part_1(data)}")
    print(f"Part 2: {part_2(data)}")
