use std::collections::HashMap;

fn part_1(instructions: &[isize]) -> u32 {
    let mut i = 0;
    let mut steps = 0;
    let mut offsets = HashMap::new();
    while let Some(num) = instructions.get(i) {
        let offset = offsets.entry(i).or_insert(0);
        i = i + *offset;
        *offset += 1;
        steps += 1;

        let signed_i = i as isize + num;
        if signed_i < 0 {
            break;
        }

        i = signed_i as usize;
    }
    steps
}

fn main() {
    let input = include_str!("input");
    let instructions = input.lines()
        .filter_map(|l| l.parse().ok())
        .collect::<Vec<isize>>();

    println!("Part 1: Number of steps: {}", part_1(&instructions));
}
