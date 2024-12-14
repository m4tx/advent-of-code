use advent::prelude::*;
use std::ops::{Add, AddAssign, Mul};

type Input = Vec<Robot>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Robot {
    position: Point2D,
    velocity: Point2D,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    pub const fn new(x: i64, y: i64) -> Self {
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

impl AddAssign<Self> for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

const SIZE: Point2D = Point2D::new(101, 103);

fn parse_input(input: &str) -> Input {
    let button_a_re = regex::Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();

    let mut robots = Input::new();

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let captures = button_a_re.captures(line).unwrap();
        let a_x: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let a_y: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
        let v_x: i64 = captures.get(3).unwrap().as_str().parse().unwrap();
        let v_y: i64 = captures.get(4).unwrap().as_str().parse().unwrap();

        robots.push(Robot {
            position: Point2D::new(a_x, a_y),
            velocity: Point2D::new(v_x, v_y),
        });
    }

    robots
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 14))
}

fn part1(mut input: Input) -> usize {
    simulate(&mut input, 100, SIZE);
    score(&input, SIZE)
}

fn simulate(input: &mut Input, seconds: i64, size: Point2D) {
    for _ in 0..seconds {
        for robot in input.iter_mut() {
            robot.position += robot.velocity;
            while robot.position.x < 0 {
                robot.position.x += size.x;
            }
            while robot.position.y < 0 {
                robot.position.y += size.y;
            }
            robot.position.x %= size.x;
            robot.position.y %= size.y;
        }
    }
}

fn score(input: &Input, size: Point2D) -> usize {
    let q1 = robots_between(
        input,
        Point2D::new(0, 0),
        Point2D::new(size.x / 2, size.y / 2),
    );
    let q2 = robots_between(
        input,
        Point2D::new(size.x / 2 + 1, 0),
        Point2D::new(size.x, size.y / 2),
    );
    let q3 = robots_between(
        input,
        Point2D::new(0, size.y / 2 + 1),
        Point2D::new(size.x / 2, size.y),
    );
    let q4 = robots_between(
        input,
        Point2D::new(size.x / 2 + 1, size.y / 2 + 1),
        Point2D::new(size.x, size.y),
    );

    q1 * q2 * q3 * q4
}

fn robots_between(input: &Input, start: Point2D, end: Point2D) -> usize {
    input
        .iter()
        .filter(|robot| {
            robot.position.x >= start.x
                && robot.position.y >= start.y
                && robot.position.x < end.x
                && robot.position.y < end.y
        })
        .count()
}

fn part2(mut input: Input) -> i64 {
    for i in 0..SIZE.x * SIZE.y {
        simulate(&mut input, 1, SIZE);
        if are_halves_similar(&input, SIZE) {
            return i + 1;
        }
    }

    i64::MAX
}

fn are_halves_similar(input: &Input, size: Point2D) -> bool {
    let left_half = input
        .iter()
        .filter(|robot| robot.position.x < size.x / 2)
        .map(|robot| robot.position);
    let right_half: HashSet<_> = input
        .iter()
        .copied()
        .filter(|robot| robot.position.x > size.x / 2)
        .map(|robot| Point2D {
            x: size.x - 1 - robot.position.x,
            y: robot.position.y,
        })
        .collect();

    let score = left_half.filter(|point| right_half.contains(point)).count();

    score > 100
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 221616000);
    assert_eq!(part2(input), 7572);
}

#[test]
fn examples() {
    let input = parse_input(
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    );
    assert_eq!(part1(input), 21);
}
