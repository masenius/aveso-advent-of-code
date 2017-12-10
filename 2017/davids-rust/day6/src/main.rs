use std::collections::HashSet;

fn index_of_max(banks: &[u32]) -> usize {
    let mut max = 0;
    let mut max_i = 0;
    for (i, &n) in banks.iter().enumerate() {
        if n > max {
            max_i = i;
            max = n;
        }
    }
    max_i
}

fn run_cycle(banks: &mut [u32]) {
    let max_i = index_of_max(&banks);
    let mut i = max_i + 1;
    let max_value = banks[max_i];
    banks[max_i] = 0;
    for _ in 0..max_value {
        if i >= banks.len() {
            i = 0;
        }
        banks[i] += 1;
        i += 1;
    }
}

fn cycles_until_loop(banks: &mut [u32]) -> u32 {
    let mut seen = HashSet::new();
    let mut cycles = 0;
    loop {
        if seen.contains(banks) {
            break;
        }
        seen.insert(banks.to_vec());
        run_cycle(banks);
        cycles += 1;
    }
    cycles
}

fn main() {
    let input = include_str!("input");
    let mut banks = input.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<u32>>();
    assert_eq!(banks.len(), 16);
    println!("Part 1: Cycles until loop: {}",
             cycles_until_loop(banks.as_mut_slice()));
    println!("Part 2: Cycles in loop: {}",
             cycles_until_loop(banks.as_mut_slice()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_cycle() {
        let mut banks: Vec<u32> = vec![0,2,7,0];
        run_cycle(banks.as_mut_slice());
        assert_eq!(banks, vec![2,4,1,2]);
        run_cycle(banks.as_mut_slice());
        assert_eq!(banks, vec![3,1,2,3]);
    }

    #[test]
    fn test_index_of_max() {
        let banks = vec![1,2,4,3];
        assert_eq!(index_of_max(&banks), 2);
        let banks = vec![1,4,4,2];
        assert_eq!(index_of_max(&banks), 1);
    }
}
