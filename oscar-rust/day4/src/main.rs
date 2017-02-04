#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;

lazy_static! {
    static ref CHECKSUM_SPLIT:Regex = Regex::new("(?x)
(?P<payload>[a-z-]+)-
(?P<shift>[0-9]+)
\\[(?P<checksum>[a-z]+)\\]").unwrap();
}

#[derive(Debug)]
struct CodeParts<'a> {
    payload: Vec<&'a str>,
    checksum: Vec<&'a str>,
    shift: i32,
    calc_checksum: Vec<&'a str> //HashMap<&'a str, i32>
}

impl<'a> CodeParts<'a> {
    fn new(room_code: &str) -> CodeParts {
        let caps: Vec<_> = CHECKSUM_SPLIT.captures(room_code).into_iter().collect();

        let payload: Vec<&str> = caps[0].name("payload")
            .expect("No payload matched")
            .as_str()
            .split("-")
            .collect();

        let shift: i32 = caps[0].name("shift")
            .expect("No shift matched")
            .as_str()
            .parse()
            .expect("Could not parse shift as digit");


        let checksum: Vec<&str> = caps[0].name("checksum")
            .expect("No checksum match")
            .as_str()
            .split("")
            .filter(|c| !c.is_empty())
            .take(5)
            .collect();

        let mut occurences: HashMap<&str, u32> = HashMap::new();
        for part in &payload {
            let parts: Vec<&str> = part.split("").filter(|p| !p.is_empty()).collect();
            for letter in &parts {
                *occurences.entry(letter).or_insert(0) += 1;
            }
        }
        let calc_checksum = calculate_checksum(occurences);

        CodeParts {
            payload: payload,
            shift: shift,
            checksum: checksum,
            calc_checksum: calc_checksum
        }
    }

    fn is_valid(&self) -> bool {
        self.calc_checksum == self.checksum
    }

    fn unshift(&self) -> String {
        let shift: u8 = (self.shift % 26) as u8;
        let case = 'a' as u8;

        // Det här måste gå att skriva mer lättförståerligt utan att
        // behöva skapa mellansträngen...
        self.payload.iter().map(|part| {
            part.chars().map(|c| {
                (((c as u8 - case + shift) % 26) + case) as char
            }).collect::<String>()
        }).collect::<Vec<String>>().join(" ")
    }
}


fn calculate_checksum<'a>(occurences: HashMap<&'a str, u32>) -> Vec<&'a str> {
    let mut count_vec: Vec<_> = occurences
        .iter()
        .collect();

    // Sortera först på antalet, om lika på bokstaven
    count_vec.sort_by(|a, b| {
        match a.1.cmp(b.1).reverse() {
            Ordering::Equal => a.0.cmp(b.0),
            other => other,
        }
    });

    // Kasta siffrorna och ta bara de fem första
    count_vec
        .iter()
        .map(|&(letter, _)| *letter)
        .take(5)
        .collect::<Vec<&str>>()
}

fn part1() {
    // let valids = ["aaaaa-bbb-z-y-x-123[abxyz]", // true
    //               "a-b-c-d-e-f-g-h-987[abcde]", // true
    //               "not-a-real-room-404[oarel]", // true
    //               "totally-real-room-200[decoy]" // false
    // ];

    let bufreader = get_lines();
    let mut sector_sum = 0;
    let mut north_pole = 0;

    for bufline in bufreader.lines() {
        let line = bufline.expect("Something wrong with this line");
        let codeparts = CodeParts::new(&line);

        if codeparts.is_valid() {
            sector_sum += codeparts.shift
        }

        // What is the sector ID of the room where North Pole objects
        // are stored?
        let unshifted = codeparts.unshift();
        if unshifted.contains("north") {
            north_pole = codeparts.shift
        }
    }

    // What is the sum of the sector IDs of the real rooms?
    assert_eq!(sector_sum, 185371);
    println!("Sector sum: {}", sector_sum);
    println!("Stuff stored in: {}", north_pole);
}

fn part2() {
    let encrypted = CodeParts::new("qzmt-zixmtkozy-ivhz-343[zimth]");
    assert_eq!(encrypted.unshift(), "very encrypted name");
}

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
fn get_lines() -> BufReader<File> {
    let path = Path::new("src/input.txt");
    if !path.exists() {
        panic!("Input path does not exist! {}", path.display());
    }

    let input_file = File::open(path).expect("Could not open input.txt!");

    BufReader::new(input_file)
}
