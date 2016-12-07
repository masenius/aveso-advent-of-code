#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right
}

struct MoveInstruction {
    blocks: i16,
    direction: Direction
}

impl MoveInstruction {
    fn from_string(s: &str) -> Result<MoveInstruction, ()> {
        let mut chars = s.chars();
        let letter = try!(chars.nth(0).ok_or(()));
        let number = try!(chars.collect::<String>().parse::<i16>().or(Err(())));
        let direction = match letter {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => { return Err(()); }
        };
        Ok(MoveInstruction{blocks: number, direction: direction})
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Heading {
    North,
    South,
    East,
    West
}

impl Heading {
    fn turn(&self, direction: Direction) -> Heading {
        use Heading::*;
        match direction {
            Direction::Right => match *self {
                North => East,
                East => South,
                South => West,
                West => North
            },
            Direction::Left => match *self {
                North => West,
                East => North,
                South => East,
                West => South
            }
        }
    }
}

struct Position {
    blocks_north: i16,
    blocks_east: i16,
    heading: Heading
}

impl Position {
    fn new() -> Position {
        Position {
            blocks_north: 0,
            blocks_east: 0,
            heading: Heading::North
        }
    }

    fn turn_and_move(&mut self, instruction: &MoveInstruction) {
        use Heading::*;
        self.heading = self.heading.turn(instruction.direction);
        match self.heading {
            North => self.blocks_north += instruction.blocks,
            East => self.blocks_east += instruction.blocks,
            South => self.blocks_north -= instruction.blocks,
            West => self.blocks_east -= instruction.blocks
        };
    }

    fn blocks_away(&self) -> u16 {
        (self.blocks_north.abs() + self.blocks_east.abs()) as u16
    }
}

fn read_instructions(instructions: &str) -> Vec<MoveInstruction> {
    instructions.split(", ")
        .map(|s| MoveInstruction::from_string(s).expect("Bad input list"))
        .collect()
}

fn follow_instructions(instructions: &[MoveInstruction]) -> Position {
    let mut pos = Position::new();
    for inst in instructions {
        pos.turn_and_move(&inst);
    }
    pos
}

fn main() {
    let input = include_str!("input");
    let instructions = read_instructions(input);
    let pos = follow_instructions(&instructions);
    println!("The Easter Bunny's HQ is {} blocks away", pos.blocks_away());
}

#[cfg(test)]
mod test {
    use super::{read_instructions, follow_instructions, MoveInstruction};
    use super::Direction::*;

    #[test]
    fn tc_1() {
        let instructions = read_instructions("R2, L3");
        let pos = follow_instructions(&instructions);
        assert_eq!(pos.blocks_away(), 5);
    }

    #[test]
    fn tc_2() {
        let instructions = read_instructions("R2, R2, R2");
        let pos = follow_instructions(&instructions);
        assert_eq!(pos.blocks_away(), 2);
    }

    #[test]
    fn tc_3() {
        let instructions = read_instructions("R5, L5, R5, R3");
        let pos = follow_instructions(&instructions);
        assert_eq!(pos.blocks_away(), 12);
    }
}
