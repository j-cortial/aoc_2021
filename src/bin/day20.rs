use std::{collections::HashMap, hash::Hash};

use nalgebra::RowVector2;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum PixelValue {
    #[default]
    Dark,
    Light,
}

impl TryFrom<char> for PixelValue {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(PixelValue::Dark),
            '#' => Ok(PixelValue::Light),
            _ => Err(()),
        }
    }
}

impl PixelValue {
    fn value(&self) -> u8 {
        match &self {
            PixelValue::Dark => 0,
            PixelValue::Light => 1,
        }
    }
}

type Pixel = RowVector2<isize>;

#[derive(Debug, Clone)]
struct Image {
    center: HashMap<Pixel, PixelValue>,
    outside: PixelValue,
}

impl Image {
    fn new(center: HashMap<Pixel, PixelValue>) -> Self {
        Self {
            center,
            outside: PixelValue::default(),
        }
    }

    fn pixel_value(&self, pixel: Pixel) -> PixelValue {
        *self.center.get(&pixel).unwrap_or(&self.outside)
    }

    fn bounds(&self) -> ((isize, isize), (isize, isize)) {
        self.center.keys().fold(
            ((isize::MAX, isize::MIN), (isize::MAX, isize::MIN)),
            |acc, x| {
                (
                    (acc.0 .0.min(x[0]), acc.0 .1.max(x[0])),
                    (acc.1 .0.min(x[1]), acc.1 .1.max(x[1])),
                )
            },
        )
    }

    fn lit_pixel_count(&self) -> Option<usize> {
        match self.outside {
            PixelValue::Dark => Some(
                self.center
                    .values()
                    .filter(|&&v| v == PixelValue::Light)
                    .count(),
            ),
            PixelValue::Light => None,
        }
    }

    fn enhance(&self, algorithm: &[PixelValue]) -> Self {
        let new_outside = match self.outside {
            PixelValue::Dark => algorithm[0],
            PixelValue::Light => algorithm[511],
        };
        let stencil = [
            Pixel::new(-1, -1),
            Pixel::new(0, -1),
            Pixel::new(1, -1),
            Pixel::new(-1, 0),
            Pixel::new(0, 0),
            Pixel::new(1, 0),
            Pixel::new(-1, 1),
            Pixel::new(0, 1),
            Pixel::new(1, 1),
        ];
        let ((x_min, x_max), (y_min, y_max)) = self.bounds();
        let mut new_center = HashMap::new();
        for x in x_min - 1..=x_max + 1 {
            for y in y_min - 1..=y_max + 1 {
                let p = Pixel::new(x, y);
                let v = algorithm[stencil
                    .iter()
                    .map(|d| self.pixel_value(p + d).value())
                    .fold(0usize, |acc, x| (acc * 2) + x as usize)];
                new_center.insert(p, v);
            }
        }
        Self {
            center: new_center,
            outside: new_outside,
        }
    }
}

fn solve_part1(input_image: &Image, algorithm: &[PixelValue]) -> usize {
    input_image
        .enhance(algorithm)
        .enhance(algorithm)
        .lit_pixel_count()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<PixelValue>, Image) {
    let mut data = input.split("\n\n");
    let algorithm = data
        .next()
        .unwrap()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect();
    let image_center = data
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Pixel::new(x as isize, y as isize), c.try_into().unwrap()))
        })
        .collect();
    (algorithm, Image::new(image_center))
}

fn main() {
    let input = include_str!("../../data/day20.txt");
    let (algorithm, input_image) = parse_input(input);
    let answer1 = solve_part1(&input_image, &algorithm);
    println!("The answer to part 1 is {answer1}");
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part1};

    #[test]
    fn test_part1() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        let (algorithm, input_image) = parse_input(input);
        assert_eq!(input_image.lit_pixel_count(), Some(10));
        assert_eq!(input_image.enhance(&algorithm).lit_pixel_count(), Some(24));
        assert_eq!(solve_part1(&input_image, &algorithm), 35);
    }
}
