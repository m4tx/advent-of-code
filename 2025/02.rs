use advent::prelude::*;

type Input = Vec<(i64, i64)>;

fn parse_input(input: &str) -> Input {
    input
        .split(",")
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start: i64 = parts[0].trim().parse().unwrap();
            let end: i64 = parts[1].trim().parse().unwrap();
            (start, end)
        })
        .collect()
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 02));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    check_repeated_impl(input, Policy::DivideInHalf)
}

fn part2(input: Input) -> i64 {
    check_repeated_impl(input, Policy::DivideAny)
}

fn check_repeated_impl(input: Input, policy: Policy) -> i64 {
    let mut counter = 0;

    for (start, end) in input {
        for num in start..=end {
            let num_str = num.to_string();

            let sizes = match policy {
                Policy::DivideInHalf => {
                    if num_str.len() % 2 != 0 {
                        continue;
                    }
                    let half = num_str.len() / 2;
                    half..half + 1
                }
                Policy::DivideAny => 1..num_str.len() / 2 + 1,
            };
            for i in sizes {
                if is_repeated(&num_str, i) {
                    counter += num;
                    break;
                }
            }
        }
    }

    counter
}

fn is_repeated(s: &str, split_at: usize) -> bool {
    if !s.len().is_multiple_of(split_at) {
        return false;
    }

    let parts = s.len() / split_at;
    for part in 1..parts {
        if s[0..split_at] != s[part * split_at..(part + 1) * split_at] {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, Copy)]
enum Policy {
    DivideInHalf,
    DivideAny,
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
        assert_eq!(part1(input.clone()), 24157613387);
        assert_eq!(part2(input), 33832678380);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(part1(input.clone()), 1227775554);
        assert_eq!(part2(input), 4174379265);
    }
}
