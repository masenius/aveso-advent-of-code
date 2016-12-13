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
}

fn error_correct_message(input: &str) -> String {
    let mut letter_maps = BTreeMap::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let mut freq_map = letter_maps.entry(i).or_insert(FreqMap::new());
            freq_map.update(c);
        }
    }

    letter_maps.values()
        .map(|fm| fm.most_common_letter)
        .collect()
}

fn main() {
    let input = include_str!("input");
    println!("The corrected message is {}", error_correct_message(input));
}

#[cfg(test)]
mod test {
    use super::error_correct_message;

    #[test]
    fn tc_1() {
        let message = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        assert_eq!(error_correct_message(message), "easter".to_string());
    }
}
