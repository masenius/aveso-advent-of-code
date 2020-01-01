use intcode::{IntcodeMachine, MachineState};

fn run_until_output(machine: &mut IntcodeMachine) -> isize {
    loop {
        match machine.step() {
            MachineState::Output(val) => return val,
            MachineState::Halted => panic!("Unexpected halt"),
            MachineState::AwaitingInput => panic!("Unexpected state (awaiting input)"),
            _ => {}
        }
    }
}

fn main() {
    let input = include_str!("input");
    let parsed: Result<Vec<isize>, _> = input.split(",").map(str::parse::<isize>).collect();

    let mem = parsed.expect("Failed to parse program");

    let mut machine = IntcodeMachine::load(mem.clone());
    machine.add_memory(1024);
    machine.set_input(1);
    println!("Part 1: {}", run_until_output(&mut machine));

    let mut machine = IntcodeMachine::load(mem.clone());
    machine.add_memory(1024);
    machine.set_input(2);
    println!("Part 2: {}", run_until_output(&mut machine));
}
