import re

import day_03


def test_part_1():
    data = [
        '#1 @ 1,3: 4x4',
        '#2 @ 3,1: 4x4',
        '#3 @ 5,5: 2x2',
    ]
    data = [tuple(map(int, re.findall('\d+', input))) for input in data]
    expected_result = 4
    assert day_03.part_1(data) == expected_result


def test_part_2():
    data = [
        '#1 @ 1,3: 4x4',
        '#2 @ 3,1: 4x4',
        '#3 @ 5,5: 2x2',
    ]
    data = [tuple(map(int, re.findall('\d+', input))) for input in data]
    expected_result = 3
    assert day_03.part_2(data) == expected_result
