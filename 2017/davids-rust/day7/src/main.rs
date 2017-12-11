#[macro_use]
extern crate nom;

use std::collections::HashSet;

mod parser;

use parser::parse_discs;

fn main() {
    let input = include_str!("input");
    let mut non_root_discs: HashSet<&str> = HashSet::new();
    let mut all_discs = HashSet::new();
    let discs = parse_discs(input).to_result().unwrap();
    for disc in discs.iter() {
        all_discs.insert(disc.name);
        if let Some(ref c) = disc.children {
            non_root_discs.extend(c);
        }
    }

    let root_discs = all_discs.difference(&non_root_discs)
        .map(|s| *s)
        .collect::<Vec<&str>>();
    assert_eq!(root_discs.len(), 1);
    println!("The name of the bottom program is {}", root_discs[0]);
}
