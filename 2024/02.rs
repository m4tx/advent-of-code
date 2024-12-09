use advent::prelude::*;

type Input = Vec<Vec<i64>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 02))
}

fn part1(input: Input) -> i64 {
    let mut count = 0;
    for line in input {
        if is_safe(&line) {
            count += 1;
        }
    }

    count
}

fn is_safe(line: &Vec<i64>) -> bool {
    let increasing = line[1] > line[0];
    for i in 1..line.len() {
        if (increasing && line[i - 1] > line[i])
            || (!increasing && line[i - 1] < line[i])
            || !(1i64..=3i64).contains(&(line[i - 1] - line[i]).abs())
        {
            return false;
        }
    }

    true
}

fn part2(input: Input) -> i64 {
    let mut count = 0;
    for line in input {
        if is_safe(&line) {
            count += 1;
        } else {
            for i in 0..line.len() {
                let mut new_line = line.clone();
                new_line.remove(i);

                if is_safe(&new_line) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 639);
    assert_eq!(part2(input), 674);
}

#[test]
fn examples() {
    let input = parse_input(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    );
    assert_eq!(part1(input.clone()), 2);
    assert_eq!(part2(input), 4);
}
