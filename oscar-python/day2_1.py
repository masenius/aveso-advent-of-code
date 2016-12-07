class Parser:
    "Feature: x & y are mixed up!"
    pad = ((1, 2, 3),
           (4, 5, 6),
           (7, 8, 9))

    def __init__(self):
        self.position = [1, 1] # 5

    def parse(self, instruction):
        parts = []
        for row in instruction:
            for element in row:
                self.move(element)
            x, y = self.position
            parts.append(self.pad[x][y])
            print("Added: ", self.pad[x][y])

        return "".join(str(x) for x in parts)

    def move(self, direction):
        x, y = self.position

        if direction == "U" and x > 0:
            self.position[0] -= 1
        elif direction == "D" and x < 2:
            self.position[0] += 1
        elif direction == "L" and y > 0:
            self.position[1] -= 1
        elif direction == "R" and y < 2:
            self.position[1] += 1

if __name__ == '__main__':
    inpt = ["ULL",
            "RRDDD",
            "LURDL",
            "UUUUD"]

    p = Parser()
    code = p.parse(inpt)
    print(code)
    assert code == "1985"

    with open("day2_input.txt", "r") as fobj:
        lines = fobj.readlines()

    parser = Parser()
    _lines = [l.strip() for l in lines]

    code = parser.parse(_lines)
    print(code)
