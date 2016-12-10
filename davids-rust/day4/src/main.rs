use std::collections::{HashMap, HashSet};
use std::cmp::min;

#[derive(Debug)]
struct Room {
    letters: Vec<char>,
    section: u32,
    checksum: Vec<char>
}

impl Room {
    fn new(letters: Vec<char>, section: u32, checksum: Vec<char>) -> Room {
        Room {
            letters: letters,
            section: section,
            checksum: checksum
        }
    }

    fn letter_freq(&self) -> HashMap<char, usize> {
        self.letters.iter()
            .fold(HashMap::new(), |mut map, &c| {
                {
                    let counter = map.entry(c).or_insert(0);
                    *counter += 1;
                }
                map
            })
    }

    fn valid(&self) -> bool {
        if self.checksum.len() != 5 {
            return false;
        }

        let freq_map = self.letter_freq();
        let mut sorted = self.letters.clone();
        sorted.sort_by_key(|c| freq_map.get(&c).unwrap());
        sorted.reverse();

        let mut prev_freq = freq_map.get(&sorted[0]).unwrap();
        let mut grouped = Vec::new();
        let mut curr_group = HashSet::new();
        for c in sorted {
            let freq = freq_map.get(&c).unwrap();
            if freq != prev_freq {
                grouped.push(curr_group);
                curr_group = HashSet::new()
            }
            prev_freq = freq;
            curr_group.insert(c);
        }
        grouped.push(curr_group);

        let mut checksum_i = 0;
        for g in grouped.iter() {

            let last = min(checksum_i + g.len(), self.checksum.len());
            for c in self.checksum[checksum_i..last].iter() {
                if !g.contains(c) {
                    return false;
                }
            }
            checksum_i = checksum_i + g.len();
            if checksum_i >= self.checksum.len() {
                break;
            }
        }

        true
    }
}

fn take_until_filter<F, S>(input: &[char], filter: F,
                           stop: S) -> (Vec<char>, usize)
    where F: Fn(char) -> bool, S: Fn(char) -> bool {
    let mut out = Vec::new();
    let mut counter = 0;
    for c in input {
        if stop(*c) {
            break;
        }
        if filter(*c) {
            out.push(*c);
        }
        counter += 1;
    }
    (out, counter)
}

fn parse_room(input: &str) -> Room {
    let chars = input.chars().collect::<Vec<char>>();
    let mut consumed = 0;
    let (letters, chars_read) = take_until_filter(&chars,
                                                  |c| c.is_alphabetic(),
                                                  |c| c.is_numeric());

    consumed += chars_read;
    let (number_chars, chars_read) = take_until_filter(&chars[consumed..],
                                                       |c| c.is_numeric(),
                                                       |c| !c.is_numeric());

    let number: u32 = number_chars.into_iter().collect::<String>().parse().unwrap();

    consumed += chars_read;
    let (checksum, _) = take_until_filter(&chars[consumed..],
                                          |c| c.is_alphabetic(),
                                          |_| false);

    Room::new(letters, number, checksum)
}

fn main() {
    let input = include_str!("input");
    let section_count = input.lines()
        .map(parse_room)
        .filter(|r| r.valid())
        .fold(0, |sum, r| {
            sum + r.section
        });

    println!("Sum of sections of valid rooms is {}", section_count);
}

#[cfg(test)]
mod test {
    use super::{parse_room};

    #[test]
    fn tc_1() {
        assert!(parse_room("aaaaa-bbb-z-y-x-123[abxyz]").valid());
    }

    #[test]
    fn tc_2() {
        assert!(parse_room("a-b-c-d-e-f-g-h-987[abcde]").valid());
    }

    #[test]
    fn tc_3() {
        assert!(parse_room("not-a-real-room-404[oarel]").valid());
    }

    #[test]
    fn tc_4() {
        assert!(!parse_room("totally-real-room-200[decoy]").valid());
    }

}
