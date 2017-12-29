fn reverse_section(list: &mut [u16], start: usize, length: usize) {
    let end = start + length - 1;
    for (i, list_i) in (start..(start + (length / 2))).enumerate() {
        let list_i = list_i % list.len();
        let opposite_i = (end - i) % list.len();
        list.swap(list_i, opposite_i);
    }
}

fn hash(list: &mut [u16], lengths: &[usize], rounds: usize) {
    let mut skip_size = 0;
    let mut pos = 0;
    for _ in 0..rounds {
        for length in lengths.iter() {
            reverse_section(list, pos, *length);
            pos += length + skip_size;
            pos = pos % list.len();
            skip_size += 1;
        }
    }
}

fn hash_v1(list: &mut [u16], lengths: &[usize]) -> u16 {
    hash(list, lengths, 1);
    list[0] * list[1]
}

fn hash_v2(list: &mut [u16], lengths: &[usize]) -> String {
    hash(list, lengths, 64);
    list.chunks(16)
        .map(|c| c.iter().fold(0, |res, &num| res ^ num))
        .map(|n| format!("{:02x}", n))
        .collect::<String>()
}

fn main() {
    let input = include_str!("input");
    let lengths_v1 = input.split(",")
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<usize>>();
    let list = (0..256).collect::<Vec<u16>>();
    println!("Part 1: Result: {}", hash_v1(list.clone().as_mut_slice(), &lengths_v1));
    let mut lengths_v2 = input.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c as usize)
        .collect::<Vec<usize>>();
    lengths_v2.extend_from_slice(&[17, 31, 73, 47, 23]);
    println!("Part 2: Result: {}", hash_v2(list.clone().as_mut_slice(), &lengths_v2));
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
