enum ParseState {
    Marker,
    Data,
    Repeated(Marker)
}

#[derive(Clone, Copy)]
struct Marker {
    repeat_count: usize,
    chars: usize
}

impl Marker {
    fn new(chars: usize, repeat_count: usize) -> Marker {
        Marker {
            repeat_count: repeat_count,
            chars: chars
        }
    }
}

fn parse_data(chars: &[char], out_buffer: &mut Vec<char>) -> usize {
    let mut consumed = 0;
    for c in chars.iter().take_while(|&c| *c != '(') {
        out_buffer.push(*c);
        consumed += 1;
    }
    consumed
}

fn parse_marker(chars: &[char]) -> Result<(usize, Marker), &'static str> {
    let mut num = String::new();
    let mut nums = Vec::new();
    let mut consumed = 0;
    for c in chars.iter() {
        consumed += 1;
        match *c {
            '(' => continue,
            'x' | ')' => {
                nums.push(try!(num.parse::<usize>().or(Err("Failed to parse number in marker"))));
                if *c == ')' {
                    break
                }
                num.clear();
            },
            ch => num.push(ch)
        }
    }
    if nums.len() == 2 {
        Ok((consumed, Marker::new(nums[0], nums[1])))
    }
    else {
        Err("Found more than two numbers in marker")
    }
}

fn repeat_chars(chars: &[char], marker: Marker, out_buffer: &mut Vec<char>) -> Result<usize, &'static str> {
    let seq = chars.iter()
        .take(marker.chars)
        .collect::<Vec<_>>();
    if seq.len() != marker.chars {
        return Err("Can't repeat more characters than remaining in input");
    }
    for _ in 0..marker.repeat_count {
        for c in seq.iter() {
            out_buffer.push(**c);
        }
    }
    Ok(marker.chars)
}

fn repeat_chars2(chars: &[char], marker: Marker) -> Result<(usize, usize), &'static str> {
    let mut decompressed_len = 0;
    decompressed_len += try!(decompress_chars_v2(&chars[..marker.chars])) * marker.repeat_count;
    Ok((marker.chars, decompressed_len))
}

fn decompress_chars(chars: &[char]) -> Result<Vec<char>, &'static str> {
    let mut state = ParseState::Data;
    let mut decompressed = Vec::with_capacity(chars.len() * 3);
    let mut pos = 0;
    loop {
        match state {
            ParseState::Data => {
                let consumed = parse_data(&chars[pos..], &mut decompressed);
                pos += consumed;
                state = ParseState::Marker;
            },
            ParseState::Marker => {
                let (consumed, marker) = try!(parse_marker(&chars[pos..]));
                pos += consumed;
                state = ParseState::Repeated(marker)
            },
            ParseState::Repeated(marker) => {
                let consumed = try!(repeat_chars(&chars[pos..], marker, &mut decompressed));
                pos += consumed;
                state = ParseState::Data;
            }
        }
        if pos >= chars.len() {
            break;
        }
    }
    Ok(decompressed)
}

fn decompress_chars_v2(chars: &[char]) -> Result<usize, &'static str> {
    let mut state = ParseState::Data;
    let mut pos = 0;
    let mut decompressed_len = 0;
    loop {
        match state {
            ParseState::Data => {
                let consumed = chars[pos..].iter().take_while(|&c| *c != '(').count();
                pos += consumed;
                decompressed_len += consumed;
                state = ParseState::Marker;
            },
            ParseState::Marker => {
                let (consumed, marker) = try!(parse_marker(&chars[pos..]));
                pos += consumed;
                state = ParseState::Repeated(marker)
            },
            ParseState::Repeated(marker) => {
                let (consumed, decomp_len) = try!(repeat_chars2(&chars[pos..], marker));
                pos += consumed;
                decompressed_len += decomp_len;
                state = ParseState::Data;
            }
        }
        if pos >= chars.len() {
            break;
        }
    }
    Ok(decompressed_len)
}

fn decompress(input: &str) -> Result<Vec<char>, &'static str> {
    let chars = input.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();
    decompress_chars(&chars)
}

fn decompress_v2(input: &str) -> Result<usize, &'static str> {
    let chars = input.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();
    decompress_chars_v2(&chars)
}

fn main() {
    let input = include_str!("input");
    let decompressed = decompress(input).unwrap();
    println!("Part 1:");
    println!("Decompressed length {}", decompressed.len());
    let v2_decompressed_len = decompress_v2(input).unwrap();
    println!("Part 2:");
    println!("Decompressed length {}", v2_decompressed_len);
}

#[cfg(test)]
mod tests {
    use super::{decompress, decompress_v2};

    fn decompress_to_str(input: &str) -> String {
        decompress(input).unwrap().iter().cloned().collect()
    }

    fn test_decompress(input: &str, expected_output: &str) {
        assert_eq!(decompress_to_str(input), expected_output.to_string());
    }

    #[test]
    fn tc_1() {
        test_decompress("ADVENT", "ADVENT");
    }

    #[test]
    fn tc_2() {
        test_decompress("A(1x5)BC", "ABBBBBC");
    }

    #[test]
    fn tc_3() {
        test_decompress("(3x3)XYZ", "XYZXYZXYZ");
    }

    #[test]
    fn tc_4() {
        test_decompress("A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG");
    }

    #[test]
    fn tc_5() {
        test_decompress("(6x1)(1x3)A", "(1x3)A");
    }

    #[test]
    fn tc_6() {
        test_decompress("X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn tc_7() {
        test_decompress("A(1x5)  BC", "ABBBBBC");
    }

    // Part 2
    #[test]
    fn tc_8() {
        assert_eq!(decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A").unwrap(), 241920);
    }
}
