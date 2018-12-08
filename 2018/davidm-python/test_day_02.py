import day_02


def test_part_1():
    ids = [
        'abcdef',
        'bababc',
        'abbcde',
        'abcccd',
        'aabcdd',
        'abcdee',
        'ababab'
    ]
    expected_checksum = 12
    assert day_02.part_1(ids) == 12


def test_part_2():
    ids = [
        'abcde',
        'fghij',
        'klmno',
        'pqrst',
        'fguij',
        'axcye',
        'wvxyz'
    ]
    expected_characters = ['f', 'g', 'i', 'j']
    assert day_02.part_2(ids) == expected_characters
