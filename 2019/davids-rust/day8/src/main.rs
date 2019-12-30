struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Image {
    fn new(data: Vec<u8>, width: usize, height: usize) -> Self {
        Image {
            data,
            width,
            height,
        }
    }

    fn layers(&self) -> impl Iterator<Item = &[u8]> {
        self.data.chunks(self.width * self.height)
    }

    fn pixel_color(&self, pos: usize) -> Color {
        assert!(pos < self.width * self.height);
        for pixel in self.data.iter().skip(pos).step_by(self.width * self.height) {
            match pixel {
                0 => return Color::Black,
                1 => return Color::White,
                _ => {}
            }
        }
        Color::Transparent
    }
}

#[derive(Debug, PartialEq)]
enum Color {
    Black,
    White,
    Transparent,
}

fn count_zeros(series: &[u8]) -> usize {
    series.iter().filter(|&n| *n == 0).count()
}

fn ones_multiplied_by_twos(series: &[u8]) -> usize {
    let (ones, twos) = series.iter().fold((0, 0), |(ones, twos), &n| {
        if n == 1 {
            (ones + 1, twos)
        } else if n == 2 {
            (ones, twos + 1)
        } else {
            (ones, twos)
        }
    });
    ones * twos
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let input = include_str!("input")
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect::<Vec<u8>>();
    let image = Image::new(input, WIDTH, HEIGHT);
    let least_zeros = image
        .layers()
        .min_by_key(|s| count_zeros(s))
        .expect("Series was empty");
    let answer = ones_multiplied_by_twos(least_zeros);
    println!("Part 1: {}", answer);

    for pixel in 0..(WIDTH * HEIGHT) {
        if pixel % WIDTH == 0 && pixel != 0 {
            print!("\n");
        }
        match image.pixel_color(pixel) {
            // Should preferably be the other way around,
            // but the letters are impossible to discern then
            Color::Black => print!(" "),
            Color::White => print!("#"),
            Color::Transparent => panic!("Transparency not supported"),
        }
    }
    print!("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_layers() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let image = Image::new(data, 3, 2);
        let mut layers = image.layers();
        assert_eq!(layers.next(), Some(&[1u8, 2, 3, 4, 5, 6] as &[u8]));
        assert_eq!(layers.next(), Some(&[7u8, 8, 9, 0, 1, 2] as &[u8]));
        assert_eq!(layers.next(), None);
    }

    #[test]
    fn test_pixel_color() {
        let data = vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        let image = Image::new(data, 2, 2);
        assert_eq!(image.pixel_color(0), Color::Black, "0");
        assert_eq!(image.pixel_color(1), Color::White, "1");
        assert_eq!(image.pixel_color(2), Color::White, "2");
        assert_eq!(image.pixel_color(3), Color::Black, "3");
    }

    #[test]
    fn test_ones_and_twos() {
        let data = [1, 2, 3, 4, 1, 2, 1];
        assert_eq!(ones_multiplied_by_twos(&data), 6);
    }
}
