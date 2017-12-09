fn increment(num: &mut isize) {
    *num += 1;
}

fn part2_offset(num: &mut isize) {
    if *num >= 3 {
        *num -= 1;
    }
    else {
        *num += 1;
    }
}

fn run_instructions(instructions: &mut [isize], offset_f: fn(&mut isize)) -> u32 {
    let mut i = 0;
    let mut steps = 0;
    while let Some(mut num) = instructions.get_mut(i) {
        steps += 1;

        let signed_i = i as isize + *num;
        if signed_i < 0 {
            break;
        }

        i = signed_i as usize;
        offset_f(&mut num);
    }
    steps
}

fn main() {
    let input = include_str!("input");
    let instructions = input.lines()
        .filter_map(|l| l.parse().ok())
        .collect::<Vec<isize>>();

    println!("Part 1: Number of steps: {}", run_instructions(&mut instructions.clone(), increment));
    println!("Part 2: Number of steps: {}", run_instructions(&mut instructions.clone(), part2_offset));
}
