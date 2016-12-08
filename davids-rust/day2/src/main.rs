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

struct Keypad {
    buttons: [[u8; 3]; 3],
    pos: (usize, usize)
}

impl Keypad {
    fn new() -> Keypad {
        Keypad {
            buttons: [
                [1,2,3],
                [4,5,6],
                [7,8,9]
            ],
            pos: (1,1)
        }
    }

    fn move_finger(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Up => {
                if self.pos.0 > 0 {
                    self.pos = (self.pos.0 - 1, self.pos.1)
                }
            },
            Right => {
                if self.pos.1 < 2 {
                    self.pos = (self.pos.0, self.pos.1 + 1)
                }
            },
            Down => {
                if self.pos.0 < 2 {
                    self.pos = (self.pos.0 + 1, self.pos.1)
                }
            },
            Left => {
                if self.pos.1 > 0 {
                    self.pos = (self.pos.0, self.pos.1 - 1)
                }
            }
        }
    }

    fn current_key(&self) -> u8 {
        self.buttons[self.pos.0][self.pos.1]
    }
}

fn read_instructions(input: &str) -> Vec<Vec<Direction>> {
    input.lines()
        .map(|l| l.chars().map(|c| Direction::from_char(c).unwrap()).collect())
        .collect()
}

fn get_code(instructions: Vec<Vec<Direction>>) -> Vec<u8> {
    let mut numbers = Vec::new();
    let mut keypad = Keypad::new();
    for line in instructions {
        for dir in line {
            keypad.move_finger(dir);
        }
        numbers.push(keypad.current_key());
    }
    numbers
}

fn main() {
    let input = include_str!("input");
    let instructions = read_instructions(input);
    let code = get_code(instructions);
    println!("{:?}", code)
}

#[cfg(test)]
mod test {
    use super::{read_instructions, get_code};

    #[test]
    fn tc_1() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let instructions = read_instructions(input);
        assert_eq!(get_code(instructions), vec![1,9,8,5]);
    }
}
