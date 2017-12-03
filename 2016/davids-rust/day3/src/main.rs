type Triangle = (u16, u16, u16);

fn valid_triangles_by_line(input: &str) -> Result<Vec<Triangle>, ()> {
    input.lines()
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<u16>().or(Err(())))
                .collect::<Result<Vec<_>, ()>>()
                .and_then(|v| {
                    if v.len() == 3 {
                        Ok((v[0], v[1], v[2]))
                    }
                    else {
                        Err(())
                    }
                })
        })
        .filter(|r| {
            r.as_ref().map(valid_triangle).unwrap_or(true)
        })
        .collect()
}


fn valid_triangles_by_column(input: &str) -> Result<Vec<Triangle>, ()> {
    let numbers = try!(input.split_whitespace()
                       .map(|d| d.parse::<u16>().or(Err(())))
                       .collect::<Result<Vec<_>, ()>>());

    Ok(numbers.chunks(9)
       .flat_map(|nums| {
           nums.iter().take(3)
               .zip(nums.iter().skip(3).take(3))
               .zip(nums.iter().skip(6).take(3))
       })
       .map(|((a,b),c)| (*a,*b,*c))
       .filter(valid_triangle)
       .collect::<Vec<_>>())
}


fn valid_triangle(triangle: &Triangle) -> bool {
    let mut sides = vec![triangle.0, triangle.1, triangle.2];
    sides.sort();
    sides[0] + sides[1] > sides[2]
}

fn part_one(input: &str) -> usize {
    valid_triangles_by_line(input)
        .expect("Malformatted input!")
        .into_iter()
        .count()
}

fn part_two(input: &str) -> usize {
    valid_triangles_by_column(input)
        .expect("Malformatted input!")
        .into_iter()
        .count()
}

fn main() {
    let input = include_str!("input");
    println!("Part one:");
    println!("Number of valid triangles: {}", part_one(input));
    println!("Part two:");
    println!("Actual number of valid triangles: {}", part_two(input));
}
