use intcode::{IntcodeMachine, MachineState};

fn run_to_end(machine: &mut IntcodeMachine) {
    loop {
        if let MachineState::Halted = machine.step() {
            break;
        }
    }
}

fn part1(rom: &[isize]) {
    let mut program = rom.to_vec();
    program[1] = 12;
    program[2] = 2;

    let mut machine = IntcodeMachine::load(program);
    run_to_end(&mut machine);
    println!("Part 1: value after run: {}", machine.state(0));
}

fn part2(rom: &[isize]) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = rom.to_vec();
            program[1] = noun;
            program[2] = verb;

            let mut machine = IntcodeMachine::load(program);
            run_to_end(&mut machine);

            if machine.state(0) == 19690720 {
                println!(
                    "Part2: noun: {}, verb: {}, final: {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return;
            }
        }
    }
}

fn main() {
    let input = include_str!("input");
    let parsed: Result<Vec<isize>, _> = input.split(",").map(str::parse::<isize>).collect();

    let rom = parsed.expect("Failed to parse program");

    part1(&rom);
    part2(&rom);
}
