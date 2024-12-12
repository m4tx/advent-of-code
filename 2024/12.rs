use advent::prelude::*;
use std::ops::Add;

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    let mut input: Input = input.lines().map(|line| line.chars().collect()).collect();
    let len_x = input[0].len();
    for line in &mut input {
        line.insert(0, '#');
        line.push('#');
    }
    input.insert(0, vec!['#'; len_x + 2]);
    input.push(vec!['#'; len_x + 2]);
    input
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 12))
}

fn part1(input: Input) -> i64 {
    get_regions(input)
        .iter()
        .map(|region| region.area * region.perimeter)
        .sum()
}

fn part2(input: Input) -> i64 {
    get_regions(input)
        .iter()
        .map(|region| region.area * region.sides)
        .sum()
}

fn get_regions(input: Input) -> Vec<Region> {
    let len_x = input[0].len();
    let len_y = input.len();

    let mut visited = vec![vec![false; len_x]; len_y];

    (0..len_x)
        .cartesian_product(0..len_y)
        .map(|(x, y)| process_region(&input, x, y, &mut visited))
        .filter(|&region| region != Region::zero())
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Region {
    area: i64,
    perimeter: i64,
    sides: i64,
}

impl Region {
    const fn zero() -> Self {
        Self {
            area: 0,
            perimeter: 0,
            sides: 0,
        }
    }
}

impl Add for Region {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            area: self.area + rhs.area,
            perimeter: self.perimeter + rhs.perimeter,
            sides: self.sides + rhs.sides,
        }
    }
}

fn process_region(input: &Input, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> Region {
    if input[y][x] == '#' || visited[y][x] {
        return Region::zero();
    }
    visited[y][x] = true;

    let region_id = input[y][x];

    let mut region = Region {
        area: 1,
        perimeter: 4,
        sides: 4,
    };
    let neighbours = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
    for (nx, ny) in neighbours.iter() {
        if input[*ny][*nx] == region_id {
            let neighbour = process_region(input, *nx, *ny, visited);
            region = region + neighbour;
            region.perimeter -= 1;
        }
    }

    // top side
    if (input[y][x - 1] == region_id && input[y - 1][x - 1] != region_id)
        || input[y - 1][x] == region_id
    {
        region.sides -= 1;
    }
    // bottom side
    if (input[y][x + 1] == region_id && input[y + 1][x + 1] != region_id)
        || input[y + 1][x] == region_id
    {
        region.sides -= 1;
    }
    // right side
    if (input[y - 1][x] == region_id && input[y - 1][x + 1] != region_id)
        || input[y][x + 1] == region_id
    {
        region.sides -= 1;
    }
    // left side
    if (input[y + 1][x] == region_id && input[y + 1][x - 1] != region_id)
        || input[y][x - 1] == region_id
    {
        region.sides -= 1;
    }

    region
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1344578);
    assert_eq!(part2(input), 814302);
}

#[test]
fn examples() {
    let input = parse_input(
        "AAAA
BBCD
BBCC
EEEC",
    );
    assert_eq!(part1(input.clone()), 140);
    assert_eq!(part2(input), 80);
    let input = parse_input(
        "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
    );
    assert_eq!(part1(input.clone()), 772);
    assert_eq!(part2(input), 436);
    let input = parse_input(
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    );
    assert_eq!(part1(input.clone()), 1930);
    assert_eq!(part2(input), 1206);
}
