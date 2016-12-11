extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

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

fn main() {
    let input = "cxdnnyjw";
    println!("Password: {}", find_password(input, 8));
}

#[cfg(test)]
mod test {
    use super::find_password;

    #[test]
    fn tc_1() {
        let input = "abc";
        assert_eq!(find_password(input, 8), "18f47a30".to_string());
    }
}
