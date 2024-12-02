use advent::prelude::*;

type Input = Vec<i64>;

fn parse_input(input: &str) -> Input {
    todo!("parsing")
}

fn default_input() -> Input {
    parse_input(include_input!({ year } / { day }))
}

fn part1(input: Input) -> i64 {
    todo!("part 1")
}

fn part2(input: Input) -> i64 {
    todo!("part 2")
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
    let input = parse_input("");
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}
