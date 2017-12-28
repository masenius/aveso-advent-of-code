fn reverse_section(list: &mut [u16], start: usize, length: usize) {
    let end = start + length - 1;
    for (i, list_i) in (start..(start + (length / 2))).enumerate() {
        let list_i = list_i % list.len();
        let opposite_i = (end - i) % list.len();
        list.swap(list_i, opposite_i);
    }
}

fn part1(list: &mut [u16], lengths: &[usize]) -> u16 {
    let mut skip_size = 0;
    let mut pos = 0;
    for length in lengths.iter() {
        reverse_section(list, pos, *length);
        pos += length + skip_size;
        pos = pos % list.len();
        skip_size += 1;
    }
    list[0] * list[1]
}

fn main() {
    let input = include_str!("input");
    let lengths = input.split(",")
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<usize>>();
    let list = (0..256).collect::<Vec<u16>>();
    println!("Part 1: Result: {}", part1(list.clone().as_mut_slice(), &lengths));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_section() {
        // Even length
        let mut input = [1, 2, 3, 4];
        reverse_section(&mut input[..], 0, 4);
        assert_eq!(input, [4, 3, 2, 1]);

        // Odd length
        let mut input = [1, 2, 3, 4, 5];
        reverse_section(&mut input[..], 0, 5);
        assert_eq!(input, [5, 4, 3, 2, 1]);

        // Wrapping
        let mut input = [1, 2, 3, 4, 5];
        reverse_section(&mut input[..], 2, 4);
        assert_eq!(input, [3, 2, 1, 5, 4]);
    }

}
