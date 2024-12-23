use advent::prelude::*;
use itertools::iproduct;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

type Input = Vec<i64>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 22))
}

fn part1(input: Input) -> i64 {
    input
        .iter()
        .map(|x| {
            let mut val = *x;
            for _ in 0..2000 {
                val = calc(val);
            }
            val
        })
        .sum()
}

fn part2(input: Input) -> i64 {
    let histories = input.iter().map(|x| get_bananas(*x)).collect::<Vec<_>>();

    let range = -9..=9;
    let possibilities: Vec<_> =
        iproduct!(range.clone(), range.clone(), range.clone(), range).collect();
    possibilities
        .into_par_iter()
        .map(|(a, b, c, d)| {
            let expected_changes = [a, b, c, d];
            histories
                .iter()
                .map(|hist| hist.get_price_for(expected_changes))
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct History {
    changes: Vec<i64>,
    prices: Vec<i64>,
}

impl History {
    fn get_price_for(&self, expected_changes: [i64; 4]) -> i64 {
        for i in 0..self.changes.len() - 3 {
            if &self.changes[i..i + 4] == expected_changes {
                return self.prices[i + 3];
            }
        }

        0
    }
}

fn get_bananas(mut num: i64) -> History {
    let mut price = num % 10;
    let mut changes = Vec::new();
    let mut prices = Vec::new();

    for _ in 0..2000 {
        num = calc(num);
        let new_price = num % 10;
        let diff = new_price - price;
        price = new_price;

        changes.push(diff);
        prices.push(price);
    }

    History { changes, prices }
}

fn calc(num: i64) -> i64 {
    let num = (num ^ (num * 64)) % 16777216;
    let num = (num ^ (num / 32)) % 16777216;

    (num ^ (num * 2048)) % 16777216
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 14392541715);
    assert_eq!(part2(input), 1628);
}

#[test]
fn examples() {
    let input = parse_input(
        "1
10
100
2024",
    );
    assert_eq!(part1(input.clone()), 37327623);

    let input = parse_input(
        "1
2
3
2024",
    );
    assert_eq!(part2(input), 23);
}
