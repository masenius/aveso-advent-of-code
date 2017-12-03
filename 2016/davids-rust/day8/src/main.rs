use std::fmt;

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize)
}

struct Screen {
    cells: [[bool; 50]; 6],
    lit_pixels: usize
}

impl Screen {
    fn new() -> Screen {
        Screen {
            cells: [[false; 50]; 6],
            lit_pixels: 0
        }
    }

    fn rect(&mut self, to_x: usize, to_y: usize) {
        for x in 0..to_y {
            for y in 0..to_x {
                self.set_pixel(x, y, true);
            }
        }
    }

    fn rotate_col(&mut self, col_no: usize, amount: usize) {
        let mut new_col = vec![false; self.cells.len()];
        for row in 0..self.cells.len() {
            let to = (row + amount) % self.cells.len();
            new_col[to] = self.cells[row][col_no];
        }
        for (i, val) in new_col.iter().enumerate() {
            self.set_pixel(i, col_no, *val);
        }
    }

    fn rotate_row(&mut self, row_no: usize, amount: usize) {
        let mut new_row = vec![false; self.cells[row_no].len()];
        for col in 0..self.cells[row_no].len() {
            let to = (col + amount) % self.cells[row_no].len();
            new_row[to] = self.cells[row_no][col];
        }
        for (i, val) in new_row.iter().enumerate() {
            self.set_pixel(row_no, i, *val);
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, to: bool) {
        match (self.cells[x][y], to) {
            (false, true) => self.lit_pixels += 1,
            (true, false) => self.lit_pixels -= 1,
            _ => {}
        }
        self.cells[x][y] = to;
    }

    fn lit_pixels(&self) -> usize {
        self.lit_pixels
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::with_capacity(50*6);
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                if self.cells[x][y] {
                    out.push('#');
                }
                else {
                    out.push(' ');
                }
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

fn parse_rect<'a, I>(token_iter: &'a mut I) -> Result<Instruction, ()>
    where I: Iterator<Item=&'a str> {
    let area = try!(token_iter.next().ok_or(()));
    let x_y: Vec<_> = try!(area.split("x")
                           .map(|n| n.parse::<usize>().or(Err(())))
                           .collect());
    if x_y.len() == 2 {
        Ok(Instruction::Rect(x_y[0], x_y[1]))
    }
    else {
        Err(())
    }
}

fn parse_rotate<'a, I>(token_iter: &'a mut I) -> Result<Instruction, ()>
    where I: Iterator<Item=&'a str> {
    let what = try!(token_iter.next().ok_or(()));
    let index: usize = try!(token_iter.next().ok_or(())
                            .and_then(|s| {
                                let num: &str = try!(s.split("=")
                                                     .last()
                                                     .ok_or(()));

                                num.parse::<usize>()
                                    .or(Err(()))
                            }));

    let how_much = try!(token_iter.nth(1)
                        .ok_or(())
                        .and_then(|s: &str| s.parse::<usize>().or(Err(()))));
    match what {
        "row" => Ok(Instruction::RotateRow(index, how_much)),
        "column" => Ok(Instruction::RotateColumn(index, how_much)),
        _ => Err(())
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, ()> {
    input.lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            let cmd = words.next().unwrap();
            match cmd {
                "rect" => parse_rect(&mut words),
                "rotate" => parse_rotate(&mut words),
                _ => Err(())
            }
        })
        .collect()
}

fn follow_instructions(instructions: &[Instruction], screen: &mut Screen) {
    for inst in instructions {
        match *inst {
            Instruction::Rect(x, y) => screen.rect(x, y),
            Instruction::RotateColumn(col, amt) => screen.rotate_col(col, amt),
            Instruction::RotateRow(row, amt) => screen.rotate_row(row, amt),
        }
    }
}

fn main() {
    let input = include_str!("input");
    let instructions = parse_instructions(input).unwrap();
    let mut screen = Screen::new();
    follow_instructions(&instructions, &mut screen);
    println!("There are {} lit pixels", screen.lit_pixels());
    println!("{}", screen);
}
