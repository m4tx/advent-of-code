use advent::prelude::*;

type Input = Vec<Vec<i64>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|x| x as i64 - '0' as i64).collect())
        .collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 10))
}

fn part1(input: Input) -> i64 {
    trailheads(&input)
        .map(|(x, y)| reachable_peaks_num(&input, x, y, 0).len() as i64)
        .sum()
}

fn part2(input: Input) -> i64 {
    trailheads(&input)
        .map(|(x, y)| unique_paths_num(&input, x, y, 0))
        .sum()
}

fn trailheads(input: &Input) -> impl Iterator<Item = (i64, i64)> {
    let y_len = input.len() as i64;
    let x_len = input[0].len() as i64;

    (0..x_len)
        .cartesian_product(0..y_len)
        .filter(|&(x, y)| input[y as usize][x as usize] == 0)
}

fn reachable_peaks_num(input: &Input, x: i64, y: i64, expected: i64) -> HashSet<(i64, i64)> {
    if invalid_input(input, x, y, expected) {
        return HashSet::new();
    }

    if input[y as usize][x as usize] == 9 {
        return HashSet::from_iter([(x, y)]);
    }

    let mut set = reachable_peaks_num(input, x - 1, y, expected + 1);
    set.extend(reachable_peaks_num(input, x + 1, y, expected + 1));
    set.extend(reachable_peaks_num(input, x, y - 1, expected + 1));
    set.extend(reachable_peaks_num(input, x, y + 1, expected + 1));

    set
}

fn unique_paths_num(input: &Input, x: i64, y: i64, expected: i64) -> i64 {
    if invalid_input(input, x, y, expected) {
        return 0;
    }

    if input[y as usize][x as usize] == 9 {
        return 1;
    }

    let mut count = 0;
    count += unique_paths_num(input, x - 1, y, expected + 1);
    count += unique_paths_num(input, x + 1, y, expected + 1);
    count += unique_paths_num(input, x, y - 1, expected + 1);
    count += unique_paths_num(input, x, y + 1, expected + 1);

    count
}

fn invalid_input(input: &Input, x: i64, y: i64, expected: i64) -> bool {
    x < 0
        || y < 0
        || x >= input[0].len() as i64
        || y >= input.len() as i64
        || input[y as usize][x as usize] != expected
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 841);
    assert_eq!(part2(input), 1875);
}

#[test]
fn examples() {
    let input = parse_input(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    );
    assert_eq!(part1(input.clone()), 36);
    assert_eq!(part2(input), 81);
}
