use std::usize;
use std::collections::{HashMap, BTreeMap};

#[derive(Debug)]
struct FreqMap {
    map: HashMap<char, usize>,
    most_common_letter: char,
    highest_count: usize
}

impl FreqMap {
    fn new() -> FreqMap {
        FreqMap {
            map: HashMap::new(),
            most_common_letter: 'a',
            highest_count: 0
        }
    }

    fn update(&mut self, c: char) {
        let count = self.map.entry(c).or_insert(0);
        *count += 1;
        if *count > self.highest_count {
            self.most_common_letter = c;
            self.highest_count = *count;
        }
    }

    fn least_common_letter(&self) -> char {
        let mut least_common_letter = 'a';
        let mut lowest = usize::MAX;
        for (&letter, &count) in &self.map {
            if count < lowest {
                least_common_letter = letter;
                lowest = count;
            }
        }

        least_common_letter
    }
}

fn error_correct_message<F>(input: &str, map_func: F) -> String
    where F: Fn(&FreqMap) -> char {
    let mut letter_maps = BTreeMap::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let mut freq_map = letter_maps.entry(i).or_insert(FreqMap::new());
            freq_map.update(c);
        }
    }

    letter_maps.values()
        .map(map_func)
        .collect()
}

fn error_correct_most_common(input: &str) -> String {
    let most_common_func = |fm: &FreqMap| fm.most_common_letter;
    error_correct_message(input, most_common_func)
}

fn error_correct_least_common(input: &str) -> String {
    let most_common_func = |fm: &FreqMap| fm.least_common_letter();
    error_correct_message(input, most_common_func)
}

fn main() {
    let input = include_str!("input");
    println!("Error corrected using most frequent letter:");
    println!("The corrected message is {}", error_correct_most_common(input));
    println!("Error corrected using least frequent letter:");
    println!("The corrected message is {}", error_correct_least_common(input));
}

#[cfg(test)]
mod test {
    use super::{error_correct_most_common, error_correct_least_common};

    #[test]
    fn tc_1() {
        let message = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        assert_eq!(error_correct_most_common(message), "easter".to_string());
    }

    #[test]
    fn tc_2() {
        let message = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        assert_eq!(error_correct_least_common(message), "advent".to_string());
    }
}
