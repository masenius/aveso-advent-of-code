use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

enum Direction {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Direction::*;
        let num = value[1..].parse::<u16>().map_err(|_| ())?;
        let letter = value.chars().nth(0).ok_or(())?;
        match letter {
            'U' => Ok(Up(num)),
            'D' => Ok(Down(num)),
            'L' => Ok(Left(num)),
            'R' => Ok(Right(num)),
            _ => Err(()),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn origo() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }
}

fn create_path(trail: &[Direction]) -> Vec<Coordinate> {
    use Direction::*;

    let mut current_x = 0;
    let mut current_y = 0;
    let mut path = Vec::new();
    for c in trail {
        match *c {
            Up(n) => {
                path.extend(
                    (current_y + 1..=(current_y + n as i32))
                        .map(|y| Coordinate { x: current_x, y }),
                );
                current_y += n as i32;
            }
            Down(n) => {
                path.extend(
                    ((current_y - n as i32)..=(current_y - 1 as i32))
                        .rev()
                        .map(|y| Coordinate { x: current_x, y }),
                );
                current_y -= n as i32;
            }
            Right(n) => {
                path.extend(
                    (current_x + 1..=(current_x + n as i32))
                        .map(|x| Coordinate { x, y: current_y }),
                );
                current_x += n as i32;
            }
            Left(n) => {
                path.extend(
                    (current_x - n as i32..=(current_x - 1))
                        .rev()
                        .map(|x| Coordinate { x, y: current_y }),
                );
                current_x -= n as i32;
            }
        };
    }
    path
}

fn manhattan_distance(p1: Coordinate, p2: Coordinate) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn main() -> Result<(), ()> {
    let input = include_str!("input")
        .lines()
        .map(|l| {
            l.split(",")
                .map(Direction::try_from)
                .collect::<Result<Vec<Direction>, ()>>()
        })
        .collect::<Result<Vec<Vec<Direction>>, ()>>()?;
    assert_eq!(input.len(), 2);

    let first_path = create_path(&input[0]);
    let second_path = create_path(&input[1]);

    let intersections: HashSet<Coordinate> = first_path
        .iter()
        .cloned()
        .collect::<HashSet<Coordinate>>()
        .intersection(&second_path.iter().cloned().collect::<HashSet<Coordinate>>())
        .cloned()
        .collect();

    let nearest_intersection = intersections
        .iter()
        .map(|&c| manhattan_distance(c, Coordinate::origo()))
        .min()
        .unwrap();
    println!(
        "Part 1: Manhattan distance of closest intersection {}",
        nearest_intersection
    );

    let mut path_distances_first = HashMap::new();
    for (i, c) in first_path.iter().enumerate() {
        if intersections.contains(c) {
            path_distances_first.entry(c).or_insert(i + 1);
        }
    }

    let mut closest = None;
    for (i, c) in second_path.iter().enumerate() {
        if intersections.contains(c) {
            let dist = i + 1 + path_distances_first[c];
            if let Some(n) = closest {
                closest = Some(cmp::min(dist, n));
            } else {
                closest = Some(dist);
            }
        }
    }

    println!(
        "Part 2: combined steps to nearest by path: {}",
        closest.unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Direction::*, *};

    macro_rules! coord_vec {
        ( $( ($x:expr, $y:expr) ),* ) => {{
            let mut temp = Vec::new();
            $(
                temp.push(Coordinate { x: $x, y: $y });
            )*
                temp
        }};
    }

    #[test]
    fn test_create_path_1() {
        let directions = vec![Up(3), Right(2)];
        let coordinates = create_path(&directions);
        let expected = coord_vec!((0, 1), (0, 2), (0, 3), (1, 3), (2, 3));
        assert_eq!(coordinates, expected)
    }

    #[test]
    fn test_create_path_2() {
        let directions = vec![Left(3), Down(2)];
        let coordinates = create_path(&directions);
        let expected = coord_vec!((-1, 0), (-2, 0), (-3, 0), (-3, -1), (-3, -2));
        assert_eq!(coordinates, expected)
    }
}
