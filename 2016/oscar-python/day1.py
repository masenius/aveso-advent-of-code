class Agent:
    N = "North"
    S = "South"
    W = "West"
    E = "East"

    def __init__(self):
        self.direction = self.N
        self.x = 0
        self.y = 0
        self.visited = {(0, 0)}
        self.secret = False

    def get_direction_from_instruction(self, instruction):
        direction, length = instruction[0], int(instruction[1:])
        assert direction in ("L", "R")

        possibilities = {
            ("North", "R"): self.move_east,
            ("North", "L"): self.move_west,

            ("South", "R"): self.move_west,
            ("South", "L"): self.move_east,

            ("West", "R"): self.move_north,
            ("West", "L"): self.move_south,

            ("East", "R"): self.move_south,
            ("East", "L"): self.move_north
        }

        movement = possibilities.get((self.direction, direction))

        for step in range(length):
            movement(1)
            _pos = self.get_current_position()
            _len = len(self.visited)
            self.visited.add((self.x, self.y))

            if len(self.visited) == _len and not self.secret:
                # a-ha!
                print("Secret base distance: ", self.get_manhattan_distance())
                self.secret = _pos

        if movement == self.move_north:
            self.direction = self.N
        elif movement == self.move_south:
            self.direction = self.S
        elif movement == self.move_west:
            self.direction = self.W
        elif movement == self.move_east:
            self.direction = self.E

    def move_north(self, length):
        self.y += length

    def move_south(self, length):
        self.y -= length

    def move_west(self, length):
        self.x -= length

    def move_east(self, length):
        self.x += length

    def get_current_position(self):
        return (self.x, self.y)

    def get_manhattan_distance(self):
        return abs(self.x) + abs(self.y)


if __name__ == '__main__':
    instructions = ["R2", "L3"]
    agent = Agent()
    for instruction in instructions:
        agent.get_direction_from_instruction(instruction)
    assert agent.get_current_position() == (2, 3)
    assert agent.get_manhattan_distance() == 5

    instructions = ["R2", "R2", "R2"]
    agent = Agent()
    for instruction in instructions:
        agent.get_direction_from_instruction(instruction)
    assert agent.get_current_position() == (0, -2)
    assert agent.get_manhattan_distance() == 2

    instructions = ["R5", "L5", "R5", "R3"]
    agent.x = 0
    agent.y = 0
    for instruction in instructions:
        agent.get_direction_from_instruction(instruction)
    assert agent.get_manhattan_distance() == 12

    instructions = []
    with open("day1_input.txt", "r") as fobj:
        lines = fobj.readlines()[0]
        lines = lines.strip()
        parts = lines.split(", ")
        assert len(parts) == 138
        instructions = parts[:]

    agent = Agent()
    for instruction in instructions:
        agent.get_direction_from_instruction(instruction)
    print(agent.get_current_position())
    print(agent.get_manhattan_distance())
