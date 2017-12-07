use std::collections::HashSet;

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

fn valid_passphrases(input: &str) -> usize {
    input.lines()
        .filter(|&l| !has_duplicates(l))
        .count()
}

fn main() {
    let input = include_str!("input");
    println!("Part 1: Number of valid passphrases: {}", valid_passphrases(input));
}
