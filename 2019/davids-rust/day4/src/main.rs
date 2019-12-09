fn split_digits(number: u32) -> Vec<u8> {
    let mut remaining = number;
    let mut digits = Vec::new();
    while remaining > 0 {
        digits.push((remaining % 10) as u8);
        remaining /= 10;
    }
    digits.reverse();
    digits
}

fn has_repeating_digits(digits: &[u8]) -> bool {
    digits
        .iter()
        .zip(digits.iter().skip(1))
        .any(|(p, n)| p == n)
}

fn has_double_digits(digits: &[u8]) -> bool {
    let mut prev = digits[0];
    let mut count = 1;
    for n in &digits[1..] {
        if *n == prev {
            count += 1;
        } else if count == 2 {
            return true;
        } else {
            count = 1;
        }
        prev = *n;
    }
    count == 2
}

fn digits_only_increase(digits: &[u8]) -> bool {
    digits
        .iter()
        .zip(digits.iter().skip(1))
        .all(|(p, n)| n >= p)
}

fn valid_number_part1(digits: &[u8]) -> bool {
    has_repeating_digits(digits) && digits_only_increase(digits)
}

fn valid_number_part2(digits: &[u8]) -> bool {
    has_double_digits(digits) && digits_only_increase(digits)
}

fn main() {
    let input = 171309..=643603;
    let valid_count_part1 = input
        .clone()
        .map(split_digits)
        .filter(|v| valid_number_part1(v.as_slice()))
        .count();
    println!("Part1: valid combinations: {}", valid_count_part1);

    let valid_count_part2 = input
        .map(split_digits)
        .filter(|v| valid_number_part2(v.as_slice()))
        .count();
    println!("Part2: valid combinations: {}", valid_count_part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_digits() {
        let expected = vec![1, 0, 2, 4];
        let actual = split_digits(1024);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_has_repeating_digits() {
        assert!(has_repeating_digits(&vec![1, 2, 2, 3, 4, 5]));
        assert!(!has_repeating_digits(&vec![1, 2, 3, 7, 8, 9]));
    }

    #[test]
    fn test_digits_only_increase() {
        assert!(digits_only_increase(&vec![1, 2, 3, 4, 5, 6]));
        assert!(digits_only_increase(&vec![1, 1, 1, 1, 1, 1]));
        assert!(!digits_only_increase(&vec![2, 2, 3, 4, 5, 0]));
    }

    #[test]
    fn test_valid_number_part1() {
        assert!(valid_number_part1(&vec![1, 1, 1, 1, 2, 3]));
        assert!(!valid_number_part1(&vec![2, 2, 3, 4, 5, 0]));
        assert!(!valid_number_part1(&vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn test_has_double_digits() {
        assert!(has_double_digits(&vec![1, 1, 2, 2, 3, 3]));
        assert!(has_double_digits(&vec![1, 1, 1, 1, 2, 2]));
        assert!(!has_double_digits(&vec![1, 1, 1, 1, 2, 3]));
        assert!(!has_double_digits(&vec![1, 2, 3, 4, 4, 4]));
    }

    #[test]
    fn test_valid_number_part2() {
        assert!(valid_number_part2(&vec![1, 1, 2, 3, 4, 5, 6]));
        assert!(!valid_number_part2(&vec![1, 1, 1, 1, 2, 3]));
        assert!(!valid_number_part2(&vec![1, 2, 3, 4, 5, 6]));
    }
}
