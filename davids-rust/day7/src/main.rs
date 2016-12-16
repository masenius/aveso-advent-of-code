use std::collections::{VecDeque, HashSet};
use std::ops::Index;
use std::iter::FromIterator;

fn is_abba<T>(chars: &T) -> bool where T: Index<usize, Output=char> {
    chars[0] == chars[3] && chars[1] == chars[2] && chars[0] != chars[1]
}

fn is_aba<T>(chars: &T) -> bool where T: Index<usize, Output=char> {
    chars[0] == chars[2] && chars[1] != chars[0]
}

fn bab_to_aba<T>(chars: &T) -> [char;3] where T: Index<usize, Output=char> {
    [chars[1], chars[0], chars[1]]
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
            else if within_brackets && is_abba(&buffer) {
                return false;
            }
            buffer.pop_front();
        }
    }
    abba
}

fn supports_ssl(addr: &str) -> bool {
    let mut buffer = VecDeque::new();
    let mut within_brackets = false;
    let mut abas = HashSet::new();
    let mut abas_for_babs = HashSet::new();

    for c in addr.chars() {
        if c == '[' {
            within_brackets = true;
        }
        else if c == ']' && within_brackets {
            within_brackets = false
        }

        buffer.push_back(c);
        if buffer.len() == 3 {
            if is_aba(&buffer) {
                if !within_brackets {
                    abas.insert(String::from_iter(buffer.clone().into_iter()));
                }
                else {
                    let aba = bab_to_aba(&buffer);
                    abas_for_babs.insert(String::from_iter(aba.iter().map(|c| *c).into_iter()));
                }
            }
            buffer.pop_front();
        }
    }
    !abas.is_disjoint(&abas_for_babs)
}

fn main() {
    let input = include_str!("input");
    let count = input.lines()
        .filter(|s| supports_tls(*s))
        .count();
    println!("Part one:");
    println!("{} lines support TLS", count);
    println!("Part two:");
    let count = input.lines()
        .filter(|s| supports_ssl(*s))
        .count();
    println!("{} lines support SSL", count);
}

#[cfg(test)]
mod test {
    use super::{supports_tls, supports_ssl};

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

    // Part 2

    #[test]
    fn tc_5() {
        assert!(supports_ssl("aba[bab]xyz"));
    }

    #[test]
    fn tc_6() {
        assert!(!supports_ssl("xyx[xyx]xyx"));
    }

    #[test]
    fn tc_7() {
        assert!(supports_ssl("aaa[kek]eke"));
    }

    #[test]
    fn tc_8() {
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }
}
