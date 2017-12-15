fn captcha_sum(digits: &[u8]) -> u32 {
    let mut iter = digits.iter().chain(digits.iter().take(1)).peekable();
    let mut sum = 0;
    while let Some(d) = iter.next() {
        if let Some(next_d) = iter.peek() {
            if d == *next_d {
                sum += *d as u32;
            }
        }
    }
    sum
}

fn captcha_sum_v2(digits: &[u8]) -> u32 {
    assert!(digits.len() % 2 == 0);
    let mut sum = 0;
    for (i, &d) in digits.iter().enumerate() {
        if d == digits[(i + digits.len() / 2) % digits.len()] {
            sum += d as u32
        }
    }
    sum
}

fn main() {
    let input = include_str!("input");
    let digits = input.chars()
        .filter_map(|c| c.to_digit(10).map(|c| c as u8))
        .collect::<Vec<u8>>();
    println!("Part1: The sum is {}", captcha_sum(&digits));
    println!("Part2: The sum is {}", captcha_sum_v2(&digits));
}

#[cfg(test)]
mod test {
    use super::{captcha_sum, captcha_sum_v2};

    #[test]
    fn test_captcha_sum() {
        assert_eq!(captcha_sum(&[1,1,2,2]), 3);
        assert_eq!(captcha_sum(&[1,1,1,1]), 4);
        assert_eq!(captcha_sum(&[1,2,3,4]), 0);
        assert_eq!(captcha_sum(&[9,1,2,1,2,1,2,9]), 9);
    }

    #[test]
    fn test_captcha_sum_v2() {
        assert_eq!(captcha_sum_v2(&[1,2,1,2]), 6);
        assert_eq!(captcha_sum_v2(&[1,2,2,1]), 0);
        assert_eq!(captcha_sum_v2(&[1,2,3,4,2,5]), 4);
        assert_eq!(captcha_sum_v2(&[1,2,1,3,1,4,1,5]), 4);
    }
}
