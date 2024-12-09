use advent::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Operation {
    result: i64,
    values: Vec<i64>,
}

type Input = Vec<Operation>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let line_parsed: Vec<_> = line.split(": ").collect();
            let result = i64::from_str(line_parsed[0]).unwrap();
            let values = line_parsed[1]
                .split(" ")
                .map(|x| i64::from_str(x).unwrap())
                .collect();
            Operation { result, values }
        })
        .collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 07))
}

fn part1(input: Input) -> i64 {
    input
        .into_iter()
        .filter(can_solve)
        .map(|operation| operation.result)
        .sum()
}

fn can_solve(operation: &Operation) -> bool {
    can_solve_impl(operation, 0, MathOp::Add, 0, false)
}

fn can_solve_concat(operation: &Operation) -> bool {
    can_solve_impl(operation, 0, MathOp::Add, 0, true)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MathOp {
    Add,
    Mul,
    Concat,
}

fn can_solve_impl(operation: &Operation, index: usize, op: MathOp, acc: i64, with_concat: bool) -> bool {
    let val = match op {
        MathOp::Add => acc + operation.values[index],
        MathOp::Mul => acc * operation.values[index],
        MathOp::Concat => {
            let mut val = operation.values[index];
            let mut mult = 1;
            while val > 0 {
                mult *= 10;
                val /= 10;
            }
            acc * mult + operation.values[index]
        }
    };

    if index == operation.values.len() - 1 {
        val == operation.result
    } else {
        can_solve_impl(operation, index + 1, MathOp::Add, val, with_concat)
            || can_solve_impl(operation, index + 1, MathOp::Mul, val, with_concat)
            || (with_concat && can_solve_impl(operation, index + 1, MathOp::Concat, val, with_concat))
    }
}

fn part2(input: Input) -> i64 {
    input
        .into_iter()
        .filter(can_solve_concat)
        .map(|operation| operation.result)
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 465126289353);
    assert_eq!(part2(input), 70597497486371);
}

#[test]
fn examples() {
    let input = parse_input(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    );
    assert_eq!(part1(input.clone()), 3749);
    assert_eq!(part2(input), 11387);
}
