struct IntcodeMachine {
    pc: usize,
    state: Vec<usize>
}

impl IntcodeMachine {
    pub fn load(mem: Vec<usize>) -> IntcodeMachine {
        IntcodeMachine {
            pc: 0,
            state: mem
        }
    }

    pub fn step(&mut self) -> Option<()> {
        let opcode = self.state[self.pc];
        match opcode {
            1 => {
                self.add(self.state[self.pc + 1],
                         self.state[self.pc + 2],
                         self.state[self.pc + 3]);
                self.pc += 4;
            },
            2 => {
                self.mul(self.state[self.pc + 1],
                         self.state[self.pc + 2],
                         self.state[self.pc + 3] as usize);
                self.pc += 4;
            },
            99 => return None,
            n => panic!("Invalid opcode {}", n)
        }
        Some(())
    }

    pub fn run_to_end(&mut self) {
        while let Some(_) = self.step() {}
    }

    fn add(&mut self, src1: usize, src2: usize, dst: usize) {
        self.state[dst] = self.state[src1] + self.state[src2];
    }

    fn mul(&mut self, src1: usize, src2: usize, dst: usize) {
        self.state[dst] = self.state[src1] * self.state[src2];
    }
}

fn part1(rom: &[usize]) {
    let mut program = rom.to_vec();
    program[1] = 12;
    program[2] = 2;

    let mut machine = IntcodeMachine::load(program);
    machine.run_to_end();
    println!("Part 1: value after run: {}", machine.state[0]);
}

fn part2(rom: &[usize]) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = rom.to_vec();
            program[1] = noun;
            program[2] = verb;

            let mut machine = IntcodeMachine::load(program);
            machine.run_to_end();

            if machine.state[0] == 19690720 {
                println!("Part2: noun: {}, verb: {}, final: {}",
                         noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}

fn main() {
    let input = include_str!("input");
    let parsed: Result<Vec<usize>, _> = input.split(",")
        .map(str::parse::<usize>)
        .collect();

    let rom = parsed.expect("Failed to parse program");

    part1(&rom);
    part2(&rom);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mem = vec![1,4,5,3,5,10];
        let mut machine = IntcodeMachine::load(mem);
        let res = machine.step();
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![1,4,5,15,5,10]);
        assert_eq!(res, Some(()));
    }

    #[test]
    fn test_mul() {
        let mem = vec![2,4,5,3,5,10];
        let mut machine = IntcodeMachine::load(mem);
        let res = machine.step();
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![2,4,5,50,5,10]);
        assert_eq!(res, Some(()));
    }

    #[test]
    fn test_halt() {
        let mem = vec![99];
        let mut machine = IntcodeMachine::load(mem);
        let res = machine.step();
        assert_eq!(machine.pc, 0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_multi() {
        let mem = vec![1,9,10,11,2,10,11,12,99,5,10,0,0];
        let mut machine = IntcodeMachine::load(mem);
        machine.step();
        machine.step();
        let res = machine.step();
        assert_eq!(res, None);
        assert_eq!(machine.pc, 8);
        assert_eq!(machine.state,
                   vec![1,9,10,11,2,10,11,12,99,5,10,15,150]);
    }
}
