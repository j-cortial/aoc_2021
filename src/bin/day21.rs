use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn solve_part1(start: (usize, usize)) -> usize {
    let die = (1..=100).cycle().enumerate();
    let chunks = die.chunks(3);
    let moves = chunks
        .into_iter()
        .map(|c| c.into_iter().fold((0, 0), |acc, x| (x.0 + 1, acc.1 + x.1)));
    let mut changes = (0..=1).cycle().zip(moves);
    let (throws, score, _) = changes
        .fold_while(
            (0, [0, 0], [start.0, start.1]),
            |(_, mut score, mut pos), (p, (t, m))| {
                pos[p] = 1 + (pos[p] + m - 1) % 10;
                score[p] += pos[p];
                if score[p] >= 1000 {
                    Done((t, score, pos))
                } else {
                    Continue((t, score, pos))
                }
            },
        )
        .into_inner();
    score.into_iter().min().unwrap() * throws
}

fn parse_input(input: &str) -> (usize, usize) {
    let mut iter = input.lines().map(|line| {
        line.split(": ")
            .skip(1)
            .next()
            .map(|s| s.parse().unwrap())
            .unwrap()
    });
    (iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
    let input = include_str!("../../data/day21.txt");
    let start = parse_input(input);
    let answer1 = solve_part1(start);
    println!("The answer to part 1 is {answer1}");
}
