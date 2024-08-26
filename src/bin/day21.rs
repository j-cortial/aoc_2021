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

fn solve_part2(start: (usize, usize)) -> usize {
    let mut universes = [[[0; 10]; 21]; 2];
    universes[0][0][start.0 - 1] = 1;
    universes[1][0][start.1 - 1] = 1;
    let mut wins = [0; 2];
    for p in (0..=1).cycle() {
        let m = universes[1 - p]
            .iter()
            .map(|scores| scores.iter().sum::<usize>())
            .sum::<usize>();
        if m == 0 {
            break;
        }
        let mut new_universe = [[0; 10]; 21];
        for (delta, freq) in (3..=9).zip([1, 3, 6, 7, 6, 3, 1].into_iter()) {
            for s in 0..21 {
                for l in 0..10 {
                    new_universe[s][(l + delta) % 10] += universes[p][s][l] * freq;
                }
            }
        }
        for s in (0..21).rev() {
            for l in 0..10 {
                let t = s + l + 1;
                let f = new_universe[s][l];
                new_universe[s][l] = 0;
                if t >= 21 {
                    wins[p] += f * m;
                } else {
                    new_universe[t][l] = f;
                }
            }
        }
        universes[p] = new_universe;
    }
    wins.into_iter().max().unwrap()
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
    let answer2 = solve_part2(start);
    println!("The answer to part 2 is {answer2}");
}

#[cfg(test)]
mod tests {
    use crate::solve_part2;

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2((4, 8)), 444356092776315);
    }
}
