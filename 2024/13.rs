use advent::prelude::*;
use std::ops::{Add, Mul};

type Input = Vec<Machine>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Machine {
    button_a: Point2D,
    button_b: Point2D,
    prize: Point2D,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add<Self> for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i64> for Point2D {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn parse_input(input: &str) -> Input {
    let button_a_re = regex::Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b_re = regex::Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = regex::Regex::new(r"^Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut machines = Input::new();

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let captures = button_a_re.captures(line).unwrap();
        let a_x: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let a_y: i64 = captures.get(2).unwrap().as_str().parse().unwrap();

        let captures = button_b_re.captures(lines.next().unwrap()).unwrap();
        let b_x: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let b_y: i64 = captures.get(2).unwrap().as_str().parse().unwrap();

        let captures = prize_re.captures(lines.next().unwrap()).unwrap();
        let prize_x: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let prize_y: i64 = captures.get(2).unwrap().as_str().parse().unwrap();

        machines.push(Machine {
            button_a: Point2D::new(a_x, a_y),
            button_b: Point2D::new(b_x, b_y),
            prize: Point2D::new(prize_x, prize_y),
        });

        let next_line = lines.next();
        if next_line.is_none() {
            break;
        }
    }

    machines
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 13))
}

fn part1(input: Input) -> i64 {
    input.into_iter().map(get_price).sum()
}

fn part2(mut input: Input) -> i64 {
    input
        .into_iter()
        .map(|machine| Machine {
            prize: machine.prize + Point2D::new(10000000000000, 10000000000000),
            ..machine
        })
        .map(get_price)
        .sum()
}

fn get_price(machine: Machine) -> i64 {
    let d = machine.button_a.x * machine.button_b.y - machine.button_a.y * machine.button_b.x;

    let n = machine.prize.x * machine.button_b.y - machine.prize.y * machine.button_b.x;
    let m = machine.button_a.x * machine.prize.y - machine.button_a.y * machine.prize.x;
    if n % d != 0 || m % d != 0 {
        return 0;
    }
    let n = n / d;
    let m = m / d;

    3 * n + m
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 32026);
    assert_eq!(part2(input), 89013607072065);
}

#[test]
fn examples() {
    let input = parse_input(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    );
    assert_eq!(part1(input.clone()), 480);
    assert_eq!(part2(input), 875318608908);
}
