use advent::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Input {
    rules: [bool; 128 * 128],
    updates: Vec<Vec<i32>>,
}

fn parse_input(input: &str) -> Input {
    #[derive(Debug, Copy, Clone)]
    enum Mode {
        Rules,
        Updates,
    }
    let mut mode = Mode::Rules;
    let mut data = Input {
        rules: [false; 128 * 128],
        updates: Vec::new(),
    };
    for line in input.lines() {
        match mode {
            Mode::Rules => {
                if line.is_empty() {
                    mode = Mode::Updates;
                } else {
                    let rule: Vec<_> = line.split("|").map(|x| i32::from_str(x).unwrap()).collect();
                    data.rules[(rule[1] * 128 + rule[0]) as usize] = true;
                }
            }
            Mode::Updates => {
                let page_nums: Vec<_> =
                    line.split(",").map(|x| i32::from_str(x).unwrap()).collect();
                data.updates.push(page_nums);
            }
        }
    }

    data
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 05))
}

fn part1(input: Input) -> i32 {
    input
        .updates
        .into_iter()
        .filter(|update| update_valid(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn update_valid(update: &Vec<i32>, rules: &[bool]) -> bool {
    update.is_sorted_by(|a, b| !rules[(a * 128 + b) as usize])
}

fn fix_invalid_update(mut update: Vec<i32>, rules: &[bool]) -> Vec<i32> {
    update.sort_by(|a, b| {
        if rules[(a * 128 + b) as usize] {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    update
}

fn part2(input: Input) -> i32 {
    input
        .updates
        .into_iter()
        .filter(|update| !update_valid(update, &input.rules))
        .map(|update| fix_invalid_update(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4905);
    assert_eq!(part2(input), 6204);
}

#[test]
fn examples() {
    let input = parse_input(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );
    assert_eq!(part1(input.clone()), 143);
    assert_eq!(part2(input), 123);
}
