use advent::prelude::*;
use std::ops::Add;

const DEFAULT_SIZE: usize = 71;
const DEFAULT_CUTOFF: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    corrupted: Vec<Point2D>,
    size: usize,
    cutoff: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Item {
    Safe,
    Corrupted,
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

fn parse_input(input: &str) -> Input {
    let corrupted = input
        .lines()
        .map(|line| {
            let mut vals = line.split(",");
            let x = vals.next().unwrap().parse::<usize>().unwrap();
            let y = vals.next().unwrap().parse::<usize>().unwrap();
            Point2D::new(x as i64, y as i64)
        })
        .collect();

    Input {
        corrupted,
        size: DEFAULT_SIZE,
        cutoff: DEFAULT_CUTOFF,
    }
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 18))
}

fn part1(input: Input) -> i64 {
    calc(&input.corrupted[0..input.cutoff], input.size).unwrap()
}

fn calc(corrupted: &[Point2D], size: usize) -> Option<i64> {
    let mut grid = vec![vec![Item::Safe; size]; size];
    for point in corrupted {
        grid[point.y as usize][point.x as usize] = Item::Corrupted;
    }

    let mut visited = vec![vec![false; size]; size];

    let mut q = VecDeque::new();
    q.push_back((Point2D::new(0, 0), 0));

    while let Some((p, dist)) = q.pop_front() {
        if p.x < 0
            || p.y < 0
            || p.x >= size as i64
            || p.y >= size as i64
            || grid[p.y as usize][p.x as usize] == Item::Corrupted
        {
            continue;
        }

        if p == Point2D::new(size as i64 - 1, size as i64 - 1) {
            return Some(dist);
        }

        if visited[p.y as usize][p.x as usize] {
            continue;
        }
        visited[p.y as usize][p.x as usize] = true;

        q.push_back((p + Point2D::new(1, 0), dist + 1));
        q.push_back((p + Point2D::new(-1, 0), dist + 1));
        q.push_back((p + Point2D::new(0, 1), dist + 1));
        q.push_back((p + Point2D::new(0, -1), dist + 1));
    }

    None
}

fn part2(input: Input) -> String {
    for i in (input.cutoff + 1)..input.corrupted.len() {
        let result = calc(&input.corrupted[0..i], input.size);
        if result.is_none() {
            let last = input.corrupted[i - 1];
            return format!("{},{}", last.x, last.y);
        }
    }

    panic!("no solution found");
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 310);
    assert_eq!(part2(input), "16,46");
}

#[test]
fn examples() {
    let mut input = parse_input(
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
    );
    input.size = 7;
    input.cutoff = 12;
    assert_eq!(part1(input.clone()), 22);
    assert_eq!(part2(input), "6,1");
}
