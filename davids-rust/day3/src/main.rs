type Triangle = (u16, u16, u16);

fn read_input(input: &str) -> Result<Vec<Triangle>, ()> {
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
        }).collect()
}

fn valid_triangle(triangle: &(u16, u16, u16)) -> bool {
    let mut sides = vec![triangle.0, triangle.1, triangle.2];
    sides.sort();
    sides[0] + sides[1] > sides[2]
}

fn main() {
    let input = include_str!("input");
    let triangles = read_input(input).expect("Malformatted input!");
    let valid_triangle_count = triangles.into_iter().filter(valid_triangle).count();
    println!("Number of valid triangles: {}", valid_triangle_count);
}
