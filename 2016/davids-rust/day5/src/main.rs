extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::{self, Write};

struct Md5Hasher {
    hasher: Md5
}

impl Md5Hasher {
    fn new() -> Md5Hasher {
        Md5Hasher {
            hasher: Md5::new()
        }
    }

    fn md5sum(&mut self, input: &str) -> String {
        self.hasher.input_str(input);
        let res = self.hasher.result_str();
        self.hasher.reset();
        res
    }
}

fn find_password(input: &str, password_length: usize) -> String {
    let mut hasher = Md5Hasher::new();
    let mut password = String::with_capacity(8);
    let mut test_input = input.to_string();
    let mut counter = 0u64;
    while password.len() < password_length {
        test_input.push_str(&counter.to_string());
        let hash = hasher.md5sum(&test_input);
        if hash.starts_with("00000") {
            password.push(hash.chars().nth(5).unwrap());
        }
        test_input.truncate(input.len());
        counter += 1;
    }

    password
}

fn find_password_v2(input: &str, password_length: usize) -> String {
    let mut hasher = Md5Hasher::new();
    let mut password_chars: Vec<Option<char>> = vec![None; password_length];
    let mut test_input = input.to_string();
    let mut counter = 0u64;
    let mut chars_filled = 0;
    while chars_filled < password_length {
        test_input.push_str(&counter.to_string());
        let hash = hasher.md5sum(&test_input);
        if hash.starts_with("00000") {
            let mut hash_chars = hash.chars().skip(5);
            if let Some(p) = hash_chars.next().unwrap().to_digit(10) {
                let pos = p as usize;
                if pos < password_length && password_chars[pos] == None {
                    password_chars[pos] = Some(hash_chars.next().unwrap());
                    chars_filled += 1;

                    // Very cinematic
                    print!("{}\r", password_chars.iter()
                             .map(|&o| match o {
                                 Some(c) => c,
                                 None => ' '
                             })
                           .collect::<String>());
                    io::stdout().flush().unwrap();
                }
            }
        }
        test_input.truncate(input.len());
        counter += 1;
    }

    password_chars.iter()
        .map(|o| o.unwrap())
        .collect()
}

fn main() {
    let input = "cxdnnyjw";
    println!("Part 1");
    println!("Password: {}", find_password(input, 8));
    println!("Part 2");
    println!("Password: {}", find_password_v2(input, 8));
}

#[cfg(test)]
mod test {
    use super::{find_password, find_password_v2};

    #[test]
    fn tc_1() {
        let input = "abc";
        assert_eq!(find_password(input, 8), "18f47a30".to_string());
    }

    #[test]
    fn tc_2() {
        let input = "abc";
        assert_eq!(find_password_v2(input, 8), "05ace8e3".to_string());
    }
}
