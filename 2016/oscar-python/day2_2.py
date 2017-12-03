class Parser:
    pad = ((False, False, 1, False, False),
           (False, 2, 3, 4, False),
           (5, 6, 7, 8, 9),
           (False, "A", "B", "C"),
           (False, False, "D", False, False))

    def __init__(self):
        self.position = [2, 0]

    def parse(self, instruction):
        parts = []
        for row in instruction:
            for element in row:
                self.move(element)
            y, x = self.position
            parts.append(self.pad[y][x])
            print("Added: ", self.pad[y][x])

        return "".join(str(x) for x in parts)

    def move(self, d):
        y, x = self.position
        _len = len(self.pad)

        if d == "U" and y > 0:
            if self._try_value(x, y-1):
                self.position[0] -= 1

        elif d == "D" and y < _len:
            if self._try_value(x, y+1):
                self.position[0] += 1

        elif d == "L" and x > 0:
            if self._try_value(x-1, y):
                self.position[1] -= 1

        elif d == "R" and x < _len:
            if self._try_value(x+1, y):
                self.position[1] += 1

    def _try_value(self, x, y):
        try:
            value = self.pad[y][x]
        except IndexError:
            return False
        return value

if __name__ == '__main__':
    inpt = ["ULL",
            "RRDDD",
            "LURDL",
            "UUUUD"]

    p = Parser()
    code = p.parse(inpt)
    print(code)
    assert code == "5DB3"

    with open("day2_input.txt", "r") as fobj:
        lines = fobj.readlines()

    parser = Parser()
    _lines = [l.strip() for l in lines]

    code = parser.parse(_lines)
    print(code)
