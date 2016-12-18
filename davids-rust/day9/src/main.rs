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
    let mut marker = String::new();
    let mut consumed = 0;
    for c in chars.iter() {
        consumed += 1;
        match *c {
            '(' => continue,
            ')' => break,
            ch => marker.push(ch)
        }
    }
    let nums: Vec<_> = try!(marker.split('x')
                            .map(|s| s.parse::<usize>().or(Err("Failed to parse number in marker")))
                            .collect());
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

fn decompress(input: &str) -> Result<Vec<char>, &'static str> {
    let mut state = ParseState::Data;
    let mut decompressed = Vec::with_capacity(input.len() * 3);
    let chars = input.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();
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

fn main() {
    let input = include_str!("input");
    let decompressed = decompress(input).unwrap();
    println!("Decompressed length {}", decompressed.len());
}

#[cfg(test)]
mod test {
    use super::decompress;

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
}
