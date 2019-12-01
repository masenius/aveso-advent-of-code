fn required_fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn required_fuel_accumulated(mass: u32) -> u32 {
    let mut sum = 0;

    let mut fuel_req = required_fuel(mass);
    while fuel_req > 0 {
        sum += fuel_req;
        fuel_req = required_fuel(fuel_req);
    }

    sum
}


fn main() {
    let input: Vec<u32> = include_str!("input")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let res1: u32 = input
        .iter()
        .map(|d| required_fuel(*d))
        .sum();

    let res2: u32 = input
        .iter()
        .map(|d| required_fuel_accumulated(*d))
        .sum();

    println!("Part 1: total required fuel: {}", res1);
    println!("Part 2: total required fuel: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_required_fuel() {
        let test_vector = vec![
            (12, 2),
            (14, 2),
            (1969, 654),
            (100756, 33583),
            (3, 0)
        ];

        for test in test_vector {
            assert_eq!(required_fuel(test.0), test.1);
        }
    }

    #[test]
    fn test_required_fuel_accumulated() {
        let test_vector = vec![
            (14, 2),
            (1969, 966),
            (100756, 50346)
        ];

        for test in test_vector {
            assert_eq!(required_fuel_accumulated(test.0), test.1);
        }
    }
}
