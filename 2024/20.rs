use advent::prelude::*;
use std::ops::{Add, Sub};

type Input = Vec<Vec<Item>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Item {
    Start,
    End,
    Wall,
    Empty,
}

fn parse_input(input: &str) -> Input {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row: Vec<_> = line
            .chars()
            .map(|c| match c {
                '.' => Item::Empty,
                '#' => Item::Wall,
                'S' => Item::Start,
                'E' => Item::End,
                _ => panic!("invalid item: {}", c),
            })
            .collect();
        grid.push(row);
    }

    grid
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

    pub fn manhattan_distance(&self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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

impl Sub<Self> for Point2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn find_start(grid: &[Vec<Item>]) -> Point2D {
    for (y, row) in grid.iter().enumerate() {
        for (x, &item) in row.iter().enumerate() {
            if item == Item::Start {
                return Point2D::new(x as i64, y as i64);
            }
        }
    }

    panic!("start not found")
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 20))
}

const DIRECTIONS: &[Point2D; 4] = &[
    Point2D::new(0, 1),
    Point2D::new(1, 0),
    Point2D::new(0, -1),
    Point2D::new(-1, 0),
];

fn part1(input: Input) -> i64 {
    calc(input, 2, 100)
}

fn part2(input: Input) -> i64 {
    calc(input, 20, 100)
}

fn calc(input: Input, max_distance: i64, min_save: i64) -> i64 {
    let position = find_start(&input);

    let mut q = VecDeque::new();
    q.push_back(PointInQueue {
        distance: 0,
        position,
    });

    let mut distances = HashMap::new();
    while let Some(PointInQueue { distance, position }) = q.pop_front() {
        if distances.contains_key(&position) {
            continue;
        }
        distances.insert(position, distance);

        if input[position.y as usize][position.x as usize] == Item::End {
            continue;
        }

        for &direction in DIRECTIONS {
            let new_position = position + direction;
            if input[new_position.y as usize][new_position.x as usize] == Item::Wall {
                continue;
            }
            q.push_back(PointInQueue {
                distance: distance + 1,
                position: new_position,
            });
        }
    }

    let size_y = input.len() as i64;
    let size_x = input[0].len() as i64;
    let mut count = 0;

    let it = (0..size_y).cartesian_product(0..size_x);
    for ((y, x), (dy, dx)) in it.cartesian_product(
        (-max_distance..=max_distance).cartesian_product(-max_distance..=max_distance),
    ) {
        let before_position = Point2D::new(x, y);
        let after_position = Point2D::new(x + dx, y + dy);
        let dist = after_position.manhattan_distance(before_position);

        if dist > max_distance {
            continue;
        }

        if let (Some(&before_dist), Some(&after_dist)) = (
            distances.get(&before_position),
            distances.get(&after_position),
        ) {
            if after_dist - before_dist >= min_save + dist {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PointInQueue {
    distance: i64,
    position: Point2D,
}

impl PartialOrd for PointInQueue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointInQueue {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1485);
    assert_eq!(part2(input), 1027501);
}

#[test]
fn examples() {
    let input = parse_input(
        "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
    );
    assert_eq!(calc(input.clone(), 2, 60), 1);
    assert_eq!(calc(input, 20, 70), 41);
}
