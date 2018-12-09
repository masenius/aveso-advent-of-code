from aocd import data

reacts = [chr(i) + chr(i + 32) for i in range(65, 65 + 26)] + [chr(i + 32) + chr(i) for i in range(65, 65 + 26)]


def reduce(data):
    length = len(data)
    while True:
        for char in reacts:
            data = data.replace(char, '')
        if len(data) != length:
            length = len(data)
        else:
            break
    return len(data)


def part_1(data):
    return reduce(data)


def part_2(data):
    return min([reduce(data.replace(chr(i), '').replace(chr(i + 32), '')) for i in range(65, 65 + 26)])


if __name__ == '__main__':
    print(f"Part 1: {part_1(data)}")
    print(f"Part 2: {part_2(data)}")
