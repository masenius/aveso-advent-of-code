#[macro_use] extern crate lazy_static;
#[macro_use] extern crate itertools;
extern crate regex;

use regex::Regex;
use std::fmt;
use itertools::Itertools;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;

lazy_static! {
    static ref CHECKSUM_SPLIT:Regex = Regex::new("(?x)
(?P<payload>[a-z-]+)-
(?P<numbers>[0-9]+)
\\[(?P<checksum>[a-z]+)\\]").unwrap();
}

struct CodeParts {
    payload: Vec<char>,
    checksum: Vec<char>,
    numbers: i32,
}

impl CodeParts {
    fn new<S>(room_code: S) -> CodeParts where S: Into<String> {
        let _room: &str = &room_code.into();
        let caps = CHECKSUM_SPLIT.captures(_room).unwrap();
        let payload: &str = &caps["payload"];
        let mut checksum: Vec<char> = caps["checksum"].chars().collect();
        checksum.sort_by(|a, b| a.cmp(&b));

        CodeParts {
            payload: payload.chars().filter(|a| a.is_alphanumeric()).collect::<Vec<char>>(),
            checksum: checksum,
            numbers: caps["numbers"].parse::<i32>().unwrap(),
        }
    }
}

impl fmt::Debug for CodeParts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "CodeParts{{payload: {p}, checksum: {c}, numbers: {n}}}",
               p = self.payload.iter().join(""),
               c = self.checksum.iter().join(""),
               n = self.numbers)
    }
}

fn main() {
    let valids = ["aaaaa-bbb-z-y-x-123[abxyz]", // true
                  "a-b-c-d-e-f-g-h-987[abcde]", // true
                  "not-a-real-room-404[oarel]", // true
                  "totally-real-room-200[decoy]" // false
    ];

    let mut index = 0;
    let mut num_correct = 0;
    let mut total_sum = 0;
    let size = valids.len();
    loop {
        let parts = CodeParts::new(valids[index]);
        let checksum_ok: bool = control_checksum2(&parts);

        if checksum_ok {
            num_correct += 1;
            total_sum += parts.numbers;
        }

        index += 1;
        if index == size {
            break;
        }
    }

    assert!(num_correct == 3, "The number of correct test data is 3, not {}.", num_correct);
    assert!(total_sum == 1514, "The total sum id isn't correct ({} != 1514)", total_sum);

    let path = Path::new("src/input.txt");
    if !path.exists() {
        panic!("Input path does not exist! {}", path.display());
    }

    let input_file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Could not open input.txt! {}", e);
        }
    };

    let bufreader = BufReader::new(&input_file);
    let mut num_correct = 0;
    let mut total_sum = 0;
    for line in bufreader.lines() {
        let parts = CodeParts::new(line.unwrap());
        let checksum_correct: bool = control_checksum2(&parts);

        if checksum_correct {
            num_correct += 1;
            total_sum += parts.numbers;
        }
    }

    println!("num_correct: {}", num_correct);
    println!("total sum: {}", total_sum);

}

fn control_checksum2(parts: &CodeParts) -> bool {
    // http://stackoverflow.com/questions/34555837/sort-hashmap-data-by-value
    let mut count: HashMap<char, u32> = HashMap::new();

    for letter in &parts.payload {
        *count.entry(*letter).or_insert(0) += 1;
    }

    let mut count_vec: Vec<_> = count.iter().collect();
    count_vec.sort_by(|a, b| {
        match a.1.cmp(b.1).reverse() {
            Ordering::Equal => a.0.cmp(b.0),
            other => other,
        }
    });

    let mut calculated_checksum = count_vec[0..5]
        .iter()
        .map(|&(letter, _)| letter)
        .collect::<Vec<&char>>();
    calculated_checksum.sort();


    let actual_checksum = &parts.checksum;

    // http://stackoverflow.com/a/29504547/285103
    let matching = calculated_checksum
        .iter()
        .zip(actual_checksum.iter())
        .filter(|&(calculated_checksum, actual_checksum)|
                *calculated_checksum == actual_checksum)
        .count();

    if matching == 5 {
        true
    } else {
        false
    }
}
