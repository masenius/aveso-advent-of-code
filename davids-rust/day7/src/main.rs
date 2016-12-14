use std::collections::VecDeque;

fn is_abba(chars: &VecDeque<char>) -> bool {
    assert!(chars.len() == 4);
    chars[0] == chars[3] && chars[1] == chars[2] && chars[0] != chars[1]
}

fn supports_tls(addr: &str) -> bool {
    let mut buffer = VecDeque::new();
    let mut abba = false;
    let mut within_brackets = false;

    for c in addr.chars() {
        if c == '[' {
            within_brackets = true;
        }
        else if c == ']' && within_brackets {
            within_brackets = false
        }

        buffer.push_back(c);
        if buffer.len() == 4 {
            if !abba && !within_brackets {
                abba = is_abba(&buffer);
            }
            else if within_brackets {
                if is_abba(&buffer) {
                    return false;
                }
            }
            buffer.pop_front();
        }
    }
    abba
}

fn main() {
    let input = include_str!("input");
    let count = input.lines()
        .filter(|s| supports_tls(*s))
        .count();
    println!("{} lines support TLS", count);
}

#[cfg(test)]
mod test {
    use super::supports_tls;

    #[test]
    fn tc_1() {
        assert!(supports_tls("abba[mnop]qrst"));
    }

    #[test]
    fn tc_2() {
        assert!(!supports_tls("abcd[bddb]xyyx"));
    }

    #[test]
    fn tc_3() {
        assert!(!supports_tls("aaaa[qwer]tyui"));
    }

    #[test]
    fn tc_4() {
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }
}
