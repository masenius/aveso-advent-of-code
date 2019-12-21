use intcode::{IntcodeMachine, MachineState};

struct Permutations<T> {
    values: Vec<T>,
    i: usize,
    values_len: usize,
    cycles: Vec<usize>,
    first: bool,
}

impl<T> Permutations<T>
where
    T: Clone,
{
    fn new(values: &[T]) -> Self {
        let mut n = values.len();
        let mut cycles = Vec::with_capacity(values.len());
        while n > 0 {
            cycles.push(n);
            n -= 1;
        }
        Permutations {
            values: values.into(),
            i: values.len() - 1,
            values_len: values.len(),
            cycles,
            first: true,
        }
    }
}

impl<T> Iterator for Permutations<T>
where
    T: Clone + Copy + Sized,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values_len == 0 {
            return None;
        } else if self.first {
            self.first = false;
            return Some(self.values.clone());
        }

        loop {
            self.cycles[self.i] -= 1;
            if self.cycles[self.i] == 0 {
                if self.i == 0 {
                    return None;
                }
                let temp = self.values[self.i];
                self.values
                    .copy_within((self.i + 1)..self.values_len, self.i);
                *self.values.last_mut().unwrap() = temp;
                self.cycles[self.i] = self.values_len - self.i;
                self.i -= 1;
            } else {
                let j = self.cycles[self.i];
                self.values.swap(self.i, self.values_len - j);
                self.i = self.values_len - 1;

                return Some(self.values.clone());
            }
        }
    }
}

fn run(variation: &[u8], mem: &[isize], feedback: bool) -> isize {
    let mut machines: Vec<IntcodeMachine> = variation
        .iter()
        .map(|p| {
            let mut machine = IntcodeMachine::load(mem.to_vec());
            machine.set_input(*p as isize);
            machine
        })
        .collect();

    let mut last_output = 0;

    'feedback: loop {
        for machine in machines.iter_mut() {
            loop {
                match machine.step() {
                    MachineState::AwaitingInput => {
                        machine.set_input(last_output);
                    }
                    MachineState::Output(output) => {
                        last_output = output;
                        break;
                    }
                    MachineState::Halted => break 'feedback,
                    _ => {}
                }
            }
        }
        if !feedback {
            break;
        }
    }
    last_output
}

fn part_1(mem: &[isize]) {
    let permutations = Permutations::new(&[0, 1, 2, 3, 4]);
    let largest_output = permutations.map(|p| run(&p, &mem, false)).max();
    println!("Part 1: largest output: {}", largest_output.unwrap());
}

fn part_2(mem: &[isize]) {
    let permutations = Permutations::new(&[5, 6, 7, 8, 9]);
    let largest_output = permutations.map(|p| run(&p, &mem, true)).max();
    println!("Part 2: largest output: {}", largest_output.unwrap());
}

fn main() {
    let input = include_str!("input");
    let parsed: Result<Vec<isize>, _> = input.split(",").map(str::parse::<isize>).collect();

    let mem = parsed.expect("Failed to parse program");

    part_1(&mem);
    part_2(&mem);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permuations() {
        let permutations = Permutations::new(&[1, 2, 3])
            .take(10)
            .collect::<Vec<Vec<i32>>>();
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        for test in expected {
            // Let's be lazy and not make guarantees about ordering
            assert!(
                permutations.contains(&test),
                format!("{:?} not in result", test)
            );
        }
    }
}
