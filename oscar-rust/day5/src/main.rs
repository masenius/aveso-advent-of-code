use std::collections::HashMap;

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    println!("Part 1, test");
    let test_puzzle = "abc";
    let expected = "18f47a30";
    let computed = find_password(&test_puzzle, true).expect("yolo");
    assert_eq!(computed, expected);

    println!("Part 1, real data");
    let puzzle = "reyedfim";
    let expected = "f97c354d";
    let computed = find_password(&puzzle, true).expect("yolo");
    assert_eq!(computed, expected);

    println!("Part 2, test");
    let expected = "05ace8e3";
    let computed = find_password(&test_puzzle, false).expect("yolo");
    assert_eq!(computed, expected);

    println!("Part 2, real data");
    let unordered = find_password(&puzzle, false).expect("yolo");
    let expected = "863dde27";
    assert_eq!(unordered, expected);
}

// https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
fn find_password(door_id: &str, in_order: bool) -> Option<String> {
    // Couldn't think of any easier data structure to use for this.
    let mut password: HashMap<u32, char> = HashMap::with_capacity(8);
    let mut hasher = Md5::new();
    let key = door_id.as_bytes();

    // First part of the quiz, data is expected to arrive in order.
    let mut counter = 0;

    // For the first part of the quiz when relevant char is the 6 th character,
    // and for the second part of the quiz the relevant char is the 7 th 
    // character.
    let nth_pos = match in_order {
        true => 5,
        false => 6,
    };

    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        // Thank you, internetz!
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;

        if first_five == 0 {
            let result = hasher.result_str();

            let _position = result.chars().nth(5).expect("Could not fetch the relevant character for the password");
            let mut position: u32 = match _position.to_digit(10) {
                Some(num) => num,
                None => 255,
            };

            if in_order {
                position = counter;
                counter += 1;
            }

            if position < 8 {
                let val = result.chars().nth(nth_pos).unwrap();

                if !password.contains_key(&position) {
                    password.insert(position, val);
                }
            }
        }

        // Reset before next iteration
        hasher.reset();

        if password.len() == 8 {
            let mut output = String::with_capacity(8);
            for num in 0..8 {
                let c = password.get(&num).expect(format!("Could not fetch {} from hashmap", &num)
                                                      .as_str());
                output.push(*c);
            }
            return Some(output);
        }
    }
    None
}
