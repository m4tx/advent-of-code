use advent::prelude::*;

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 08))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Antenna {
    frequency: char,
    x: i64,
    y: i64,
}

fn part1(input: Input) -> usize {
    calc(input, false)
}

fn part2(input: Input) -> usize {
    calc(input, true)
}

fn calc(input: Input, with_mult: bool) -> usize {
    let mut antennas: Vec<Antenna> = Vec::new();
    let len_x = input[0].len() as i64;
    let len_y = input.len() as i64;

    for x in 0..len_x {
        for y in 0..len_y {
            let frequency = input[y as usize][x as usize];
            if frequency != '#' && frequency != '.' {
                antennas.push(Antenna { frequency, x, y });
            }
        }
    }

    let mut antinodes = HashSet::new();
    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i == j || antennas[i].frequency != antennas[j].frequency {
                continue;
            }

            let Antenna { x: x1, y: y1, .. } = antennas[i];
            let Antenna { x: x2, y: y2, .. } = antennas[j];
            let (x_diff, y_diff) = (x2 - x1, y2 - y1);

            let mut mult = if with_mult { 0 } else { 1 };
            loop {
                let (node_x, node_y) = (x1 - x_diff * mult, y1 - y_diff * mult);

                if node_x >= 0 && node_x < len_x && node_y >= 0 && node_y < len_y {
                    antinodes.insert((node_x, node_y));
                } else {
                    break;
                }
                mult += 1;

                if !with_mult {
                    break;
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 396);
    assert_eq!(part2(input), 1200);
}

#[test]
fn examples() {
    let input = parse_input(
        "##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##",
    );
    assert_eq!(part1(input.clone()), 14);
    assert_eq!(part2(input), 34);
}

#[test]
fn examples_simple() {
    let input = parse_input(
        "..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........",
    );
    assert_eq!(part1(input), 2);
}

#[test]
fn examples_part2() {
    let input = parse_input(
        "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........",
    );
    assert_eq!(part2(input), 9);
}
