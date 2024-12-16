use advent::prelude::*;
use std::ops::Add;

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
    parse_input(include_input!(2024 / 16))
}

fn part1(input: Input) -> i64 {
    calc(input).0
}

fn part2(input: Input) -> usize {
    calc(input).1
}

fn calc(input: Input) -> (i64, usize) {
    let position = find_start(&input);

    let mut pq = BinaryHeap::new();
    let mut distances = HashMap::new();
    pq.push(PointInQueue {
        distance: 0,
        position,
        direction: Direction::Left,
    });

    let mut end_pos = Point2D::new(0, 0);
    let mut best_distance = i64::MAX;
    while let Some(p) = pq.pop() {
        let PointInQueue {
            distance,
            position,
            direction,
        } = p;

        if distances.contains_key(&(position, direction)) {
            continue;
        }
        distances.insert((position, direction), distance);

        if input[position.y as usize][position.x as usize] == Item::End {
            if distance < best_distance {
                best_distance = distance;
                end_pos = position;
            }
            continue;
        }

        for (dx, dy, new_direction) in [
            (0, 1, Direction::Down),
            (0, -1, Direction::Up),
            (1, 0, Direction::Right),
            (-1, 0, Direction::Left),
        ] {
            let new_position = Point2D::new(position.x + dx, position.y + dy);
            if input[new_position.y as usize][new_position.x as usize] == Item::Wall {
                continue;
            }

            let new_distance = distance + 1 + if new_direction == direction { 0 } else { 1000 };
            pq.push(PointInQueue {
                distance: new_distance,
                position: new_position,
                direction: new_direction,
            });
        }
    }

    let mut all_points: HashSet<Point2D> = HashSet::new();
    for direction in &[
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        if let Some(&d) = distances.get(&(end_pos, *direction)) {
            if d == best_distance {
                backtrack(&input, &distances, &mut all_points, end_pos, *direction, d);
            }
        }
    }

    (best_distance, all_points.len())
}

fn backtrack(
    input: &Input,
    distances: &HashMap<(Point2D, Direction), i64>,
    all_points: &mut HashSet<Point2D>,
    pos: Point2D,
    dir: Direction,
    distance: i64,
) {
    all_points.insert(pos);
    if input[pos.y as usize][pos.x as usize] == Item::Start {
        return;
    }

    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        for new_direction in Direction::all() {
            let new_pos = Point2D::new(pos.x - dx, pos.y - dy);
            let new_distance = distance - 1 - if new_direction == dir { 0 } else { 1000 };

            if let Some(&d) = distances.get(&(new_pos, new_direction)) {
                if d == new_distance {
                    backtrack(
                        input,
                        distances,
                        all_points,
                        new_pos,
                        new_direction,
                        new_distance,
                    );
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PointInQueue {
    distance: i64,
    position: Point2D,
    direction: Direction,
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
    assert_eq!(part1(input.clone()), 89460);
    assert_eq!(part2(input), 504);
}

#[test]
fn examples() {
    let input = parse_input(
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
",
    );
    assert_eq!(part1(input.clone()), 7036);
    assert_eq!(part2(input), 45);

    let input = parse_input(
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    );
    assert_eq!(part1(input.clone()), 11048);
    assert_eq!(part2(input), 64);
}
