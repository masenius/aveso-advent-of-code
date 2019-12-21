#[derive(Debug, PartialEq)]
pub enum MachineState {
    Running,
    AwaitingInput,
    Halted,
    Output(isize),
}

pub struct IntcodeMachine {
    pc: usize,
    state: Vec<isize>,
    input: Option<isize>,
}

impl IntcodeMachine {
    pub fn load(mem: Vec<isize>) -> IntcodeMachine {
        IntcodeMachine {
            pc: 0,
            state: mem,
            input: None,
        }
    }

    pub fn state(&self, index: usize) -> isize {
        self.state[index]
    }

    pub fn step(&mut self) -> MachineState {
        let (opcode, mode) = read_opcode(self.state[self.pc]);
        match opcode {
            1 => {
                self.op_add(
                    self.get_value(mode, 1),
                    self.get_value(mode, 2),
                    self.state[self.pc + 3] as usize,
                );
                self.pc += 4;
            }
            2 => {
                self.op_mul(
                    self.get_value(mode, 1),
                    self.get_value(mode, 2),
                    self.state[self.pc + 3] as usize,
                );
                self.pc += 4;
            }
            3 => {
                if let Some(input) = self.input {
                    self.op_input(input, self.state[self.pc + 1] as usize);
                    self.input = None;
                    self.pc += 2;
                } else {
                    return MachineState::AwaitingInput;
                }
            }
            4 => {
                let output = self.get_value(mode, 1);
                self.pc += 2;
                return MachineState::Output(output);
            }
            5 => self.op_jump_if_true(self.get_value(mode, 1), self.get_value(mode, 2) as usize),
            6 => self.op_jump_if_false(self.get_value(mode, 1), self.get_value(mode, 2) as usize),
            7 => {
                self.op_less_than(
                    self.get_value(mode, 1),
                    self.get_value(mode, 2),
                    self.state[self.pc + 3] as usize,
                );
                self.pc += 4;
            }
            8 => {
                self.op_equals(
                    self.get_value(mode, 1),
                    self.get_value(mode, 2),
                    self.state[self.pc + 3] as usize,
                );
                self.pc += 4;
            }

            99 => return MachineState::Halted,
            n => panic!("Invalid opcode {}", n),
        }
        MachineState::Running
    }

    pub fn set_input(&mut self, input: isize) {
        self.input = Some(input);
    }

    fn get_value(&self, mode: isize, arg_n: usize) -> isize {
        let arg = self.state[self.pc + arg_n];
        match mode / 10isize.pow((arg_n - 1) as u32) % 10 {
            0 => self.state[arg as usize],
            _ => arg,
        }
    }

    fn op_add(&mut self, val1: isize, val2: isize, dst: usize) {
        self.state[dst] = val1 + val2;
    }

    fn op_mul(&mut self, val1: isize, val2: isize, dst: usize) {
        self.state[dst] = val1 * val2;
    }

    fn op_input(&mut self, input: isize, dst: usize) {
        self.state[dst] = input;
    }

    fn op_jump_if_true(&mut self, val1: isize, dst: usize) {
        if val1 != 0 {
            self.pc = dst;
        } else {
            self.pc += 3;
        }
    }

    fn op_jump_if_false(&mut self, val1: isize, dst: usize) {
        if val1 == 0 {
            self.pc = dst;
        } else {
            self.pc += 3;
        }
    }

    fn op_less_than(&mut self, val1: isize, val2: isize, dst: usize) {
        let res = if val1 < val2 { 1 } else { 0 };
        self.state[dst] = res;
    }

    fn op_equals(&mut self, val1: isize, val2: isize, dst: usize) {
        let res = if val1 == val2 { 1 } else { 0 };
        self.state[dst] = res;
    }
}

fn read_opcode(value: isize) -> (isize, isize) {
    (value % 10 + (value / 10) % 10 * 10, value / 100)
}

#[cfg(test)]
mod test {
    use super::{MachineState::*, *};

    #[test]
    fn test_read_opcode_no_param() {
        let (opcode, mode) = read_opcode(1);
        assert_eq!(opcode, 1);
        assert_eq!(mode, 0);
        let (opcode, mode) = read_opcode(99);
        assert_eq!(opcode, 99);
        assert_eq!(mode, 0);
    }

    #[test]
    fn test_read_opcode_param() {
        let (opcode, mode) = read_opcode(1101);
        assert_eq!(opcode, 1);
        assert_eq!(mode, 11);
    }

    #[test]
    fn test_add() {
        let mem = vec![1, 4, 5, 3, 5, 10];
        let mut machine = IntcodeMachine::load(mem);
        let res = machine.step();
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![1, 4, 5, 15, 5, 10]);
        assert_eq!(res, Running);
    }

    #[test]
    fn test_add_param() {
        let mem = vec![1101, 4, 5, 3, 5, 10];
        let mut machine = IntcodeMachine::load(mem);
        let res = machine.step();
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![1101, 4, 5, 9, 5, 10]);
        assert_eq!(res, Running);
    }

    #[test]
    fn test_mul() {
        let mem = vec![2, 4, 5, 3, 5, 10];
        let mut machine = IntcodeMachine::load(mem);
        let res = machine.step();
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![2, 4, 5, 50, 5, 10]);
        assert_eq!(res, Running);
    }

    #[test]
    fn test_mul_param() {
        let mem = vec![1002, 4, 5, 3, 5, 10];
        let mut machine = IntcodeMachine::load(mem);
        let res = machine.step();
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![1002, 4, 5, 25, 5, 10]);
        assert_eq!(res, Running);
    }

    #[test]
    fn test_halt() {
        let mem = vec![99];
        let mut machine = IntcodeMachine::load(mem);
        let res = machine.step();
        assert_eq!(machine.pc, 0);
        assert_eq!(res, Halted);
    }

    #[test]
    fn test_multi() {
        let mem = vec![1, 9, 10, 11, 2, 10, 11, 12, 99, 5, 10, 0, 0];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.step(), Halted);
        assert_eq!(machine.pc, 8);
        assert_eq!(
            machine.state,
            vec![1, 9, 10, 11, 2, 10, 11, 12, 99, 5, 10, 15, 150]
        );
    }

    #[test]
    fn test_input() {
        let mem = vec![3, 2, 0];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), AwaitingInput);
        assert_eq!(machine.pc, 0);
        machine.set_input(5);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.pc, 2);
        assert_eq!(machine.state, vec![3, 2, 5]);
    }

    #[test]
    fn test_output() {
        let mem = vec![4, 2, 5];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), Output(5));
        assert_eq!(machine.pc, 2);
        assert_eq!(machine.state, vec![4, 2, 5]);
    }

    #[test]
    fn test_jump_if_true() {
        let mem = vec![5, 5, 4, 5, 1, 5];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.pc, 1);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.pc, 5);
    }

    #[test]
    fn test_jump_if_false() {
        let mem = vec![6, 3, 4, 0, 1, 5];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.pc, 1);
    }

    #[test]
    fn test_less_than() {
        let mem = vec![7, 1, 2, 4, 5];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![7, 1, 2, 4, 1]);

        let mem = vec![7, 4, 5, 0, 1, 0];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![0, 4, 5, 0, 1, 0]);
    }

    #[test]
    fn test_equals() {
        let mem = vec![8, 1, 1, 4, 5];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![8, 1, 1, 4, 1]);

        let mem = vec![8, 4, 5, 0, 1, 0];
        let mut machine = IntcodeMachine::load(mem);
        assert_eq!(machine.step(), Running);
        assert_eq!(machine.pc, 4);
        assert_eq!(machine.state, vec![0, 4, 5, 0, 1, 0]);
    }
}
