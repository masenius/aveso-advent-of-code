use std::collections::{HashMap, HashSet};
use std::cmp::min;

#[derive(Debug)]
struct Room {
    room_name: Vec<char>,
    section: u32,
    checksum: Vec<char>
}

impl Room {
    fn new(room_name: Vec<char>, section: u32, checksum: Vec<char>) -> Room {
        Room {
            room_name: room_name,
            section: section,
            checksum: checksum
        }
    }

    fn room_letters(&self) -> Vec<char> {
        self.room_name.clone().into_iter().filter(|&c| c.is_alphabetic()).collect()
    }

    fn letter_freq(&self) -> HashMap<char, usize> {
        self.room_letters().iter()
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
        let mut sorted = self.room_letters().clone();
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

    fn decrypt(&self) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
        self.room_name.iter()
            .map(|&c| {
                if c == '-' {
                    ' '
                }
                else {
                    let letter_pos = alphabet.iter().position(|&l| l == c).unwrap();
                    *alphabet.iter()
                        .cycle()
                        .skip(letter_pos)
                        .nth(self.section as usize)
                        .unwrap()
                }
            }).collect()
    }
}

fn take_until_filter<F, S>(input: &[char],
                           filter: F,
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
    let (mut room_name, chars_read) = take_until_filter(&chars,
                                                    |c| c.is_alphabetic() || c == '-',
                                                    |c| c.is_numeric());
    if room_name[room_name.len() - 1] == '-' {
        room_name.pop();
    }

    consumed += chars_read;
    let (number_chars, chars_read) = take_until_filter(&chars[consumed..],
                                                       |c| c.is_numeric(),
                                                       |c| !c.is_numeric());

    let number: u32 = number_chars.into_iter().collect::<String>().parse().unwrap();

    consumed += chars_read;
    let (checksum, _) = take_until_filter(&chars[consumed..],
                                          |c| c.is_alphabetic(),
                                          |_| false);

    Room::new(room_name, number, checksum)
}

fn main() {
    let input = include_str!("input");
    let valid_rooms = input.lines()
        .map(parse_room)
        .filter(|r| r.valid())
        .collect::<Vec<_>>();

    let section_count = valid_rooms.iter()
        .fold(0, |sum, r| {
            sum + r.section
        });
    println!("Sum of sections of valid rooms is {}", section_count);

    let decrypted = valid_rooms.iter()
        .map(|r| {
            let mut s = r.decrypt();
            s.push_str(&format!(" {}\n", r.section));
            s
        })
        .collect::<String>();
    println!("Decrypted: ");
    println!("{}", decrypted);
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

    // Part 2

    #[test]
    fn tc_5() {
        assert_eq!(parse_room("qzmt-zixmtkozy-ivhz-343").decrypt(),
                   "very encrypted name".to_string());
    }
}
