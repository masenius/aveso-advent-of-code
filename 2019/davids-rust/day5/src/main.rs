use intcode::{IntcodeMachine, MachineState};

fn run_and_print(mut machine: IntcodeMachine) {
    loop {
        match machine.step() {
            MachineState::Output(n) => println!("{}", n),
            MachineState::Halted => break,
            _ => {}
        }
    }
}

fn main() {
    let input = include_str!("input");
    let parsed: Result<Vec<isize>, _> = input.split(",").map(str::parse::<isize>).collect();

    let mem = parsed.expect("Failed to parse program");
    let mut machine = IntcodeMachine::load(mem.clone());
    machine.set_input(1);
    println!("Part 1");
    run_and_print(machine);

    let mut machine = IntcodeMachine::load(mem);
    machine.set_input(5);
    println!("Part 2");
    run_and_print(machine);
}
