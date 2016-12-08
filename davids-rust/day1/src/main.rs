use std::collections::HashSet;
use std::cmp::{max, min};
use std::iter::repeat;

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

    fn from_point(point: (i16, i16)) -> Position {
        Position {
            blocks_north: point.0,
            blocks_east: point.1,
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

    fn location(&self) -> (i16, i16) {
        (self.blocks_north, self.blocks_east)
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

fn points_between(p1: (i16, i16), p2: (i16, i16)) -> HashSet<(i16, i16)> {
    if p1.0 == p2.0 {
        repeat(p1.0).zip((min(p1.1, p2.1)..(max(p1.1, p2.1) + 1))).collect()
    }
    else if p1.1 == p2.1 {
        (min(p1.0, p2.0)..(max(p1.0, p2.0) + 1)).zip(repeat(p1.1)).collect()
    }
    else {
        HashSet::new()
    }
}

fn move_until_returned(instructions: &[MoveInstruction]) -> Option<(i16, i16)> {
    let mut pos = Position::new();
    let mut locs = HashSet::new();
    let mut prev_loc = pos.location();
    for inst in instructions {
        pos.turn_and_move(&inst);
        let mut visited_points = points_between(prev_loc, pos.location());
        visited_points.remove(&prev_loc);
        for p in visited_points.drain() {
            if !locs.insert(p) {
                return Some(p);
            }
        }
        prev_loc = pos.location()
    }
    None
}

fn part_one(instructions: &[MoveInstruction]) {
    let pos = follow_instructions(&instructions);
    println!("The Easter Bunny's HQ is {} blocks away", pos.blocks_away());
}

fn part_two(instructions: &[MoveInstruction]) {
    let point = move_until_returned(&instructions).unwrap();
    let pos = Position::from_point(point);
    println!("Actually, the Easter Bunny's HQ is {} blocks away", pos.blocks_away());
}

fn main() {
    let input = include_str!("input");
    let instructions = read_instructions(input);
    println!("Part One:");
    part_one(&instructions);
    println!("Part Two:");
    part_two(&instructions);
}

#[cfg(test)]
mod test {
    use super::{read_instructions, follow_instructions, move_until_returned, Position};

    // Part 1
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

    // Part two
    #[test]
    fn tc_4() {
        let instructions = read_instructions("R8, R4, R4, R8");
        let point = move_until_returned(&instructions).unwrap();
        let pos = Position::from_point(point);
        assert_eq!(pos.blocks_away(), 4);
    }
}
