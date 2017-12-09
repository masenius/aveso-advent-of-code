use std::collections::HashSet;

fn is_anagram(first: &str, second: &str) -> bool {
    let not_whitespace = |&c: &char| !c.is_whitespace();
    let mut first_chars = first.chars()
        .filter(&not_whitespace)
        .collect::<Vec<char>>();
    let mut second_chars = second.chars()
        .filter(&not_whitespace)
        .collect::<Vec<char>>();

    if first_chars.len() != second_chars.len() {
        return false;
    }

    first_chars.sort();
    second_chars.sort();

    first_chars == second_chars
}

fn has_duplicates(line: &str) -> bool {
    let mut map = HashSet::new();
    for word in line.split_whitespace() {
        if map.contains(word) {
            return true
        }
        map.insert(word);
    }
    false
}

fn has_anagrams(line: &str) -> bool {
    let words = line.split_whitespace().collect::<Vec<&str>>();
    for (i, word) in words.iter().enumerate() {
        for word2 in words.iter().skip(i+1) {
            if is_anagram(word, word2) {
                return true;
            }
        }
    }
    false
}

fn valid_passphrases(input: &str, validator: fn(&str) -> bool) -> usize {
    input.lines()
        .filter(|&l| !validator(l))
        .count()
}

fn main() {
    let input = include_str!("input");
    println!("Part 1: Number of valid passphrases: {}", valid_passphrases(input, has_duplicates));
    println!("Part 2: Number of valid passphrases: {}", valid_passphrases(input, has_anagrams));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("nag a ram", "anagram"));
        assert!(!is_anagram("abcd", "abce"));
        assert!(!is_anagram("abcd", "abcde"));
    }
}
