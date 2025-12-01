use advent::prelude::*;

type Input = Vec<(bool, i64, i64)>;

fn parse_input(input: &str) -> Input {
    let mut results = Vec::new();
    let re = regex::Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    let mut enabled = true;
    for i in 0..input.len() {
        if input[i..].starts_with("do()") {
            enabled = true;
        } else if input[i..].starts_with("don't()") {
            enabled = false;
        } else if input[i..].starts_with("mul(")
            && let Some(captures) = re.captures(&input[i..])
        {
            let (_, [a, b]) = captures.extract();
            results.push((enabled, a.parse().unwrap(), b.parse().unwrap()));
        }
    }

    results
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2024 / 03));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    input.iter().map(|(_enabled, a, b)| a * b).sum()
}

fn part2(input: Input) -> i64 {
    input
        .iter()
        .map(|(enabled, a, b)| if *enabled { a * b } else { 0 })
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
        assert_eq!(part1(input.clone()), 183669043);
        assert_eq!(part2(input), 59097164);
    }

    #[test]
    fn examples() {
        let input =
            parse_input("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(part1(input.clone()), 161);
        let input = parse_input(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(part2(input), 48);
    }
}
