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
    let data = parse_input(input);
    dbg!(data);
}
