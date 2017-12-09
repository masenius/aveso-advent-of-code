fn part_1(instructions: &mut [isize]) -> u32 {
    let mut i = 0;
    let mut steps = 0;
    while let Some(num) = instructions.get_mut(i) {
        steps += 1;

        let signed_i = i as isize + *num;
        if signed_i < 0 {
            break;
        }

        i = signed_i as usize;
        *num += 1;
    }
    steps
}

fn main() {
    let input = include_str!("input");
    let instructions = input.lines()
        .filter_map(|l| l.parse().ok())
        .collect::<Vec<isize>>();

    println!("Part 1: Number of steps: {}", part_1(&mut instructions.clone()));
}
