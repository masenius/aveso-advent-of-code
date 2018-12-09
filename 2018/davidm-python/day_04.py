import re
from collections import defaultdict, Counter

from aocd import data


def parse_data(data):
    guards = defaultdict(list)
    id_ = None
    sleep = None
    for row in data:
        if 'Guard' in row:
            id_ = int(re.findall(r'\d+', row)[-1])
        elif 'falls' in row:
            sleep = int(re.findall(r'\d+', row)[4])
        elif 'wakes' in row:
            wake = int(re.findall(r'\d+', row)[4])
            guards[id_] += range(sleep, wake)
    return guards


def part_1(data):
    longest_sleeper = None
    sleeping_minutes = []
    for id_, sm in data.items():
        if len(sm) > len(sleeping_minutes):
            sleeping_minutes = sm
            longest_sleeper = id_
    return longest_sleeper * Counter(sleeping_minutes).most_common(1)[0][0]


def part_2(data):
    longest_sleeper = None
    sleeping_minute = 0
    occurrence = 0
    for id_, sm in data.items():
        most_common = Counter(sm).most_common(1)[0]
        if most_common[1] > occurrence:
            occurrence = most_common[1]
            sleeping_minute = most_common[0]
            longest_sleeper = id_
    return longest_sleeper * sleeping_minute


if __name__ == '__main__':
    data = parse_data(sorted(data.split('\n')))
    print(f"Part 1: {part_1(data)}")
    print(f"Part 2: {part_2(data)}")
