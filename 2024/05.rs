use advent::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Input {
    rules: Vec<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

fn parse_input(input: &str) -> Input {
    enum Mode {
        Rules,
        Updates,
    }
    let mut mode = Mode::Rules;
    let mut data = Input {
        rules: Vec::new(),
        updates: Vec::new(),
    };
    for line in input.lines() {
        match mode {
            Mode::Rules => {
                if line == "" {
                    mode = Mode::Updates;
                } else {
                    let rule: Vec<_> = line.split("|").map(|x| i64::from_str(x).unwrap()).collect();
                    data.rules.push((rule[0], rule[1]));
                }
            }
            Mode::Updates => {
                let page_nums: Vec<_> =
                    line.split(",").map(|x| i64::from_str(x).unwrap()).collect();
                data.updates.push(page_nums);
            }
        }
    }

    data
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 05))
}

fn part1(input: Input) -> i64 {
    input
        .updates
        .into_iter()
        .filter(|update| update_valid(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn update_valid(update: &Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
    for next in 1..update.len() {
        for prev in 0..next {
            if rules
                .iter()
                .filter(|(dep, num)| (*dep, *num) == (update[next], update[prev]))
                .next()
                .is_some()
            {
                return false;
            }
        }
    }

    true
}

fn fix_invalid_update(update: &Vec<i64>, rules: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut update = update.clone();

    let mut updated = true;
    while updated {
        updated = false;

        for next in 1..update.len() {
            for prev in 0..next {
                if rules
                    .iter()
                    .filter(|(dep, num)| (*dep, *num) == (update[next], update[prev]))
                    .next()
                    .is_some()
                {
                    update.swap(next, prev);
                    updated = true;
                }
            }
        }
    }

    update
}

fn part2(input: Input) -> i64 {
    input
        .updates
        .into_iter()
        .filter(|update| !update_valid(update, &input.rules))
        .map(|update| fix_invalid_update(&update, &input.rules))
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
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
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
