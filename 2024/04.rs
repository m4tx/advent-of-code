use advent::prelude::*;

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 04))
}

fn part1(input: Input) -> i64 {
    let mut count = 0;

    let x_len = input[0].len();
    let y_len = input.len();
    for x in 0..x_len {
        for y in 0..y_len {
            let horizontal = if x + 3 < x_len {
                format!(
                    "{}{}{}{}",
                    input[y][x],
                    input[y][x + 1],
                    input[y][x + 2],
                    input[y][x + 3]
                )
            } else {
                String::new()
            };
            let vertical = if y + 3 < y_len {
                format!(
                    "{}{}{}{}",
                    input[y][x],
                    input[y + 1][x],
                    input[y + 2][x],
                    input[y + 3][x]
                )
            } else {
                String::new()
            };
            let diagonal = if x + 3 < x_len && y + 3 < y_len {
                format!(
                    "{}{}{}{}",
                    input[y][x],
                    input[y + 1][x + 1],
                    input[y + 2][x + 2],
                    input[y + 3][x + 3]
                )
            } else {
                String::new()
            };
            let diagonal_2 = if x >= 3 && y + 3 < y_len {
                format!(
                    "{}{}{}{}",
                    input[y][x],
                    input[y + 1][x - 1],
                    input[y + 2][x - 2],
                    input[y + 3][x - 3]
                )
            } else {
                String::new()
            };

            if horizontal == "XMAS" {
                count += 1;
            }
            if vertical == "XMAS" {
                count += 1;
            }
            if diagonal == "XMAS" {
                count += 1;
            }
            if diagonal_2 == "XMAS" {
                count += 1;
            }
            if horizontal == "SAMX" {
                count += 1;
            }
            if vertical == "SAMX" {
                count += 1;
            }
            if diagonal == "SAMX" {
                count += 1;
            }
            if diagonal_2 == "SAMX" {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: Input) -> i64 {
    let mut count = 0;

    let x_len = input[0].len();
    let y_len = input.len();
    for x in 0..x_len {
        for y in 0..y_len {
            let diagonal = if x + 2 < x_len && y + 2 < y_len {
                format!(
                    "{}{}{}",
                    input[y][x],
                    input[y + 1][x + 1],
                    input[y + 2][x + 2]
                )
            } else {
                String::new()
            };
            let diagonal_2 = if x + 2 < x_len && y + 2 < y_len {
                format!(
                    "{}{}{}",
                    input[y][x + 2],
                    input[y + 1][x + 1],
                    input[y + 2][x]
                )
            } else {
                String::new()
            };

            if (diagonal == "MAS" || diagonal == "SAM")
                && (diagonal_2 == "MAS" || diagonal_2 == "SAM")
            {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}

#[test]
fn examples() {
    let input = parse_input(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    );
    assert_eq!(part1(input.clone()), 18);
    assert_eq!(part2(input), 9);
}
