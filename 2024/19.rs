use advent::prelude::*;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    towels: Vec<String>,
    requests: Vec<String>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::to_string)
        .collect();
    lines.next();

    let requests = lines.map(str::to_string).collect();

    Input { towels, requests }
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2024 / 19));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> usize {
    input
        .requests
        .into_iter()
        .filter(|request| count_combinations(&input.towels, request) > 0)
        .count()
}

fn part2(input: Input) -> u64 {
    input
        .requests
        .into_iter()
        .map(|request| count_combinations(&input.towels, &request))
        .sum()
}

fn count_combinations(towels: &[String], request: &str) -> u64 {
    let mut checked = HashMap::new();
    count_combinations_impl(towels, &mut checked, request)
}

fn count_combinations_impl<'a>(
    towels: &[String],
    checked: &mut HashMap<&'a str, u64>,
    request: &'a str,
) -> u64 {
    if request.is_empty() {
        return 1;
    }
    if let Some(value) = checked.get(request) {
        return *value;
    }

    let count = towels
        .iter()
        .filter_map(|towel| request.strip_prefix(towel))
        .map(|remaining| count_combinations_impl(towels, checked, remaining))
        .sum();

    checked.insert(request, count);
    count
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
        assert_eq!(part1(input.clone()), 322);
        assert_eq!(part2(input), 715514563508258);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        );
        assert_eq!(part1(input.clone()), 6);
        assert_eq!(part2(input), 16);
    }
}
