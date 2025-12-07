use advent::prelude::*;

type Input = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|s| s.as_bytes().to_vec()).collect()
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 07));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    solve(input).splits
}

fn part2(input: Input) -> i64 {
    solve(input).paths
}

#[derive(Debug, Copy, Clone)]
struct Solution {
    splits: i64,
    paths: i64,
}

fn solve(input: Input) -> Solution {
    let start_pos = input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == b'S')
                .map(|x| (x as i64, y as i64))
        })
        .unwrap();

    let mut beams = HashMap::new();
    beams.insert(start_pos, 1);
    let mut splits = 0;

    loop {
        let mut new_beams = HashMap::new();

        for (&(x, y), &ways) in &beams {
            let y = y + 1;
            if y >= input.len() as i64 {
                return Solution {
                    splits,
                    paths: beams.values().sum(),
                };
            }

            match input[y as usize][x as usize] {
                b'.' => {
                    *new_beams.entry((x, y)).or_default() += ways;
                }
                b'^' => {
                    splits += 1;
                    *new_beams.entry((x - 1, y)).or_default() += ways;
                    *new_beams.entry((x + 1, y)).or_default() += ways;
                }
                _ => {
                    panic!("invalid cell")
                }
            }
        }

        beams = new_beams;
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn default() {
        let input = default_input();
        assert_eq!(part1(input.clone()), 1649);
        assert_eq!(part2(input), 16937871060075);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
",
        );
        assert_eq!(part1(input.clone()), 21);
        assert_eq!(part2(input), 40);
    }
}
