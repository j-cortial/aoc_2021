use itertools::Itertools;
use nalgebra::*;

type Scanner = Matrix3xX<i64>;

fn parse_input(input: &str) -> Vec<Scanner> {
    let data = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    line.split(',')
                        .map(|token| token.parse().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();
    data.into_iter()
        .map(|array| {
            Scanner::from_iterator(array.len(), array.into_iter().flat_map(|v| v.into_iter()))
        })
        .collect()
}

fn main() {
    let input = include_str!("../../data/day19.txt");
    let data = parse_input(input);
    dbg!(&data);
}
