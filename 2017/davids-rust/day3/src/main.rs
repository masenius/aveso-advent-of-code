use std::collections::HashMap;

fn circle_of_num(num: u32) -> u32 {
    let mut last_in_prev_circle = 1;
    for i in 1.. {
        let first = last_in_prev_circle + 1;
        let last = first + circle_size(i) - 1;
        if num >= first && num <= last {
            return i
        }
        last_in_prev_circle = last;
    }
    unreachable!();
}

fn circle_side_len(circle_n: u32) -> u32 {
    circle_n * 2 + 1
}

fn circle_size(circle_n: u32) -> u32 {
    circle_n * 8
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Top,
    Bottom,
    Left,
    Right
}

fn number_of_center(circle_n: u32, side: Side) -> u32 {
    use Side::*;
    let base_distance = match side {
        Right => 1,
        Top => 3,
        Left => 5,
        Bottom => 7
    };

    let mut num = 1;
    for i in 0..circle_n {
        num += base_distance + 8 * i
    }
    num
}

fn offset_from_side_center(num: u32, circle_n: u32, side: Side) -> Option<u32> {
    let center_num = number_of_center(circle_n, side);
    let side_len = circle_side_len(circle_n);
    if num >= center_num - side_len / 2 && num <= center_num + side_len / 2 {
        return Some((num as i64 - center_num as i64).abs() as u32);
    }
    None
}

fn part_1(input: u32) -> u32 {
    let outwards_distance = circle_of_num(input);
    let sideways_distance = offset_from_side_center(input, outwards_distance, Side::Right)
        .or_else(|| offset_from_side_center(input, outwards_distance, Side::Top))
        .or_else(|| offset_from_side_center(input, outwards_distance, Side::Bottom))
        .or_else(|| offset_from_side_center(input, outwards_distance, Side::Left))
        .expect("Number wasn't found in the expected circle");
    outwards_distance + sideways_distance
}

fn next_pos(cur_pos: (i32, i32), side: Side) -> (i32, i32) {
    use Side::*;
    match side {
        Bottom => (cur_pos.0 + 1, cur_pos.1),
        Right => (cur_pos.0, cur_pos.1 + 1),
        Left => (cur_pos.0, cur_pos.1 - 1),
        Top => (cur_pos.0 - 1, cur_pos.1)
    }
}

fn moves_per_side(side: Side, circle_n: u32) -> u32 {
    use Side::*;

    let side_len = circle_side_len(circle_n);
    match side {
        Bottom => side_len,
        Top | Left => side_len - 1,
        Right => side_len - 2
    }
}

fn turn(side: Side) -> Side {
    use Side::*;
    match side {
        Bottom => Right,
        Right => Top,
        Top => Left,
        Left => Bottom
    }
}

fn sum_adjacent_values(pos: (i32, i32), visited: &HashMap<(i32,i32), u32>) -> u32 {
    let mut sum = 0;
    for x in -1..2 {
        for y in -1..2 {
            if (x, y) == (0,0) { continue }
            sum += visited.get(&(pos.0+x, pos.1+y)).unwrap_or(&0);
        }
    }
    sum
}

fn part_2(input: u32) -> u32 {
    let mut visited = HashMap::new();
    let mut pos = (0, 0);
    let mut circle_n = 0;
    let mut side = Side::Bottom;

    visited.insert((0,0), 1);
    let mut cur_num = 1;

    let mut moves_before_turn = moves_per_side(side, circle_n);
    while cur_num <= input {
        if moves_before_turn > 0 {
            pos = next_pos(pos, side);
            cur_num = sum_adjacent_values(pos, &visited);
            visited.insert(pos, cur_num);

            moves_before_turn -= 1;
        }
        else {
            side = turn(side);
            if let Side::Right = side {
                circle_n += 1;
            }
            moves_before_turn = moves_per_side(side, circle_n);
        }
    }
    cur_num
}

fn main() {
    const INPUT: u32 = 265149;

    println!("Part 1: Distance is {}", part_1(INPUT));
    println!("Part 2: First larger than input is {}", part_2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_circle_of_num() {
        assert_eq!(circle_of_num(2), 1);
        assert_eq!(circle_of_num(17), 2);
        assert_eq!(circle_of_num(26), 3);
    }

    #[test]
    fn test_circle_side_len() {
        assert_eq!(circle_side_len(0), 1);
        assert_eq!(circle_side_len(1), 3);
        assert_eq!(circle_side_len(2), 5);
        assert_eq!(circle_side_len(4), 9);
    }

    #[test]
    fn test_circle_size() {
        assert_eq!(circle_size(1), 8);
        assert_eq!(circle_size(2), 16);
        assert_eq!(circle_size(3), 24);
    }

    #[test]
    fn test_number_of_center() {
        assert_eq!(number_of_center(1, Side::Top), 4);
        assert_eq!(number_of_center(2, Side::Right), 11);
        assert_eq!(number_of_center(3, Side::Bottom), 46);
    }

    #[test]
    fn test_offset_from_side_center() {
        assert_eq!(offset_from_side_center(30, 3, Side::Right), Some(2));
        assert_eq!(offset_from_side_center(18, 2, Side::Top), None);
    }
}
