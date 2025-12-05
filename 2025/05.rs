use advent::prelude::*;

#[derive(Clone)]
struct Input {
    fresh_ranges: Vec<(i64, i64)>,
    available: Vec<i64>,
}

fn parse_input(input: &str) -> Input {
    let mut sections = input.split("\n\n");

    let fresh_ranges = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: i64 = parts.next().unwrap().trim().parse().unwrap();
            let end: i64 = parts.next().unwrap().trim().parse().unwrap();
            (start, end)
        })
        .collect();

    let available = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    Input {
        fresh_ranges,
        available,
    }
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 05));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    let mut counter = 0;

    for ingredient in &input.available {
        for (start, end) in &input.fresh_ranges {
            if ingredient >= start && ingredient <= end {
                counter += 1;
                break;
            }
        }
    }

    counter
}

fn part2(mut input: Input) -> i64 {
    for i in 0..input.fresh_ranges.len() {
        for j in 0..input.fresh_ranges.len() {
            if i == j {
                continue;
            }

            let (s2, e2) = input.fresh_ranges[j];
            let (s1, _e1) = &mut input.fresh_ranges[i];

            if e2 >= *s1 && s2 <= *s1 {
                *s1 = e2 + 1;
            }
        }
    }

    input
        .fresh_ranges
        .into_iter()
        .map(|(start, end)| (end - start + 1).max(0))
        .sum()
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
        assert_eq!(part1(input.clone()), 643);
        assert_eq!(part2(input), 342018167474526);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(part1(input.clone()), 3);
        assert_eq!(part2(input), 14);
    }
}
