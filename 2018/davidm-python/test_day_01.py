import day_01
import pytest


@pytest.mark.parametrize('test_case', [
    (1, -1, 0),
    (3, 3, 4, -2, -4, 10),
    (-6, 3, 8, 5, -6, 5),
    (7, 7, -2, -7, -4, 14)
])
def test_part_2(test_case):
    assert day_01.part_2(test_case[:-1]) == test_case[-1]
