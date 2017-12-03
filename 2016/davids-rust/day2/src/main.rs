#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, ()> {
        use Direction::*;
        match c {
            'U' => Ok(Up),
            'R' => Ok(Right),
            'D' => Ok(Down),
            'L' => Ok(Left),
            _ => Err(())
        }
    }
}

enum KeypadKey {
    Key(&'static str),
    Blank
}

use KeypadKey::*;

type Layout = Vec<Vec<KeypadKey>>;
type Instructions = Vec<Vec<Direction>>;
type Pos = (usize, usize);

struct Keypad {
    buttons: Layout,
    pos: (usize, usize)
}

impl Keypad {
    fn new(layout: Layout, start_pos: Pos) -> Keypad {
        Keypad {
            buttons: layout,
            pos: start_pos
        }
    }

    fn move_finger(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Up => {
                if self.pos.0 > 0 {
                    let new_pos = (self.pos.0 - 1, self.pos.1);
                    self.try_move(new_pos)
                }
            },
            Right => {
                if self.pos.1 < self.buttons[self.pos.0].len() - 1  {
                    let new_pos = (self.pos.0, self.pos.1 + 1);
                    self.try_move(new_pos);
                }
            },
            Down => {
                if self.pos.0 < self.buttons.len() - 1 {
                    let new_pos = (self.pos.0 + 1, self.pos.1);
                    self.try_move(new_pos);
                }
            },
            Left => {
                if self.pos.1 > 0 {
                    let new_pos = (self.pos.0, self.pos.1 - 1);
                    self.try_move(new_pos)
                }
            }
        }
    }

    fn try_move(&mut self, new_pos: Pos) {
        if let Key(_) = self.buttons[new_pos.0][new_pos.1] {
            self.pos = new_pos
        }
    }

    fn current_key(&self) -> String {
        if let Key(k) = self.buttons[self.pos.0][self.pos.1] {
            k.to_string()
        }
        else {
            panic!("Should not be possible");
        }
    }
}

fn read_instructions(input: &str) -> Instructions {
    input.lines()
        .map(|l| l.chars().map(|c| Direction::from_char(c).unwrap()).collect())
        .collect()
}

fn get_code(instructions: &Instructions, layout: Layout, start_pos: Pos) -> Vec<String> {
    let mut numbers = Vec::new();
    let mut keypad = Keypad::new(layout, start_pos);
    for line in instructions {
        for dir in line {
            keypad.move_finger(*dir);
        }
        numbers.push(keypad.current_key());
    }
    numbers
}

fn part_one(instructions: &Instructions) -> String {
    let layout = vec![
        vec![Key("1"),Key("2"), Key("3")],
        vec![Key("4"),Key("5"), Key("6")],
        vec![Key("7"),Key("8"), Key("9")]
    ];
    let code = get_code(instructions, layout, (1, 1));
    code.into_iter().collect()
}

fn part_two(instructions: &Instructions) -> String {
    let layout = vec![
        vec![Blank,    Blank,    Key("1"), Blank,    Blank],
        vec![Blank,    Key("2"), Key("3"), Key("4"), Blank],
        vec![Key("5"), Key("6"), Key("7"), Key("8"), Key("9")],
        vec![Blank,    Key("A"), Key("B"), Key("C"), Blank],
        vec![Blank,    Blank,    Key("D"), Blank,    Blank]
    ];
    let code = get_code(instructions, layout, (2, 0));
    code.into_iter().collect()
}

fn main() {
    let input = include_str!("input");
    let instructions = read_instructions(input);
    println!("Part one:");
    println!("The code is {}", part_one(&instructions));
    println!("Part two:");
    println!("Actually, the code is {}", part_two(&instructions));
}

#[cfg(test)]
mod test {
    use super::{read_instructions, get_code};
    use super::KeypadKey::*;

    #[test]
    fn tc_1() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let instructions = read_instructions(input);
        let layout = vec![
            vec![Key("1"),Key("2"), Key("3")],
            vec![Key("4"),Key("5"), Key("6")],
            vec![Key("7"),Key("8"), Key("9")]
        ];
        let start_pos = (1, 1);
        assert_eq!(get_code(&instructions, layout, start_pos), vec!["1","9","8","5"].iter()
                   .map(|s| s.to_string()).collect::<Vec<String>>());
    }

    // Part 2
    #[test]
    fn tc_2() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let instructions = read_instructions(input);
        let layout = vec![
            vec![Blank,    Blank,    Key("1"), Blank,    Blank],
            vec![Blank,    Key("2"), Key("3"), Key("4"), Blank],
            vec![Key("5"), Key("6"), Key("7"), Key("8"), Key("9")],
            vec![Blank,    Key("A"), Key("B"), Key("C"), Blank],
            vec![Blank,    Blank,    Key("D"), Blank,    Blank]
        ];
        let start_pos = (2, 0);
        assert_eq!(get_code(&instructions, layout, start_pos), vec!["5", "D", "B", "3"].iter()
                   .map(|s| s.to_string()).collect::<Vec<String>>());
    }
}
