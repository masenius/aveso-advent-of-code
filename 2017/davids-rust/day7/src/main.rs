#[macro_use]
extern crate nom;

use std::collections::{HashSet, HashMap};

mod parser;

use parser::parse_discs;

fn get_root_disc<'b>(discs: &[parser::Disc<'b>]) -> &'b str {
    let mut non_root_discs: HashSet<&str> = HashSet::new();
    let mut all_discs = HashSet::new();

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
    root_discs[0]
}

fn weight_of_subtree(discs: &HashMap<&str, &parser::Disc>, root_disc_name: &str) -> u32 {
    let root_disc = discs.get(root_disc_name).unwrap();
    let mut weight = root_disc.weight;
    if let Some(ref children) = root_disc.children {
        for child_name in children.iter() {
            weight += weight_of_subtree(discs, child_name);
        }
    }
    weight
}

fn explore_tree(discs: &HashMap<&str, &parser::Disc>, root_disc_name: &str, tree_unbalance: i64) {
    let root_disc = discs.get(root_disc_name).unwrap();
    let mut weights = HashMap::new();
    let children = root_disc.children.as_ref().unwrap();
    for c in children {
        let disc = discs.get(c).unwrap();
        let subtree_weight = weight_of_subtree(&discs, disc.name);
        let names = weights.entry(subtree_weight).or_insert(Vec::new());
        names.push(disc.name);
    }
    if children.len() == 2 {
        panic!("Uh-oh, never handled this annoying case...");
    }
    assert!(weights.len() < 3);
    if weights.len() <= 1 {
        println!("Found the unbalanced disc, name: {}", root_disc.name);
        println!("Disc has the weight {}, should be {}", root_disc.weight, root_disc.weight as i64 + tree_unbalance);
    }
    else {
        let mut expected_weight = 0;
        let mut unbalanced_weight = 0;
        let mut unbalanced_name = None;
        for (k, v) in weights.iter() {
            if v.len() == 1 {
                unbalanced_weight = *k;
                unbalanced_name = Some(v[0]);
            }
            else {
                expected_weight = *k;
            }
        }
        let difference = expected_weight as i64 - unbalanced_weight as i64;
        explore_tree(discs, unbalanced_name.unwrap(), difference);
    }
}

fn part2(discs: &[parser::Disc], root_disc_name: &str) {
    let mut all_discs = HashMap::new();
    for disc in discs.iter() {
        all_discs.insert(disc.name, disc);
    }
    explore_tree(&all_discs, root_disc_name, 0);
}

fn main() {
    let input = include_str!("input");
    let discs = parse_discs(input).to_result().unwrap();
    let root_disc_name = get_root_disc(&discs);
    println!("The name of the bottom program is {}", root_disc_name);
    part2(&discs, root_disc_name);
}
