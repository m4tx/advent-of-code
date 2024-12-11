use advent::prelude::*;

type Input = Vec<i64>;

fn parse_input(input: &str) -> Input {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 11))
}

fn part1(input: Input) -> i64 {
    blink(&input, 25)
}

fn part2(input: Input) -> i64 {
    blink(&input, 75)
}

fn blink(input: &Input, times: i64) -> i64 {
    let mut map = HashMap::new();
    input.iter().map(|x| num_after(*x, times, &mut map)).sum()
}

fn num_after(num: i64, steps: i64, map: &mut HashMap<(i64, i64), i64>) -> i64 {
    if steps == 0 {
        return 1;
    }
    if let Some(&x) = map.get(&(num, steps)) {
        return x;
    }

    let result = if num == 0 {
        num_after(1, steps - 1, map)
    } else {
        let x_str = num.to_string();
        if x_str.len() % 2 == 0 {
            let x_1 = x_str[..x_str.len() / 2].parse().unwrap();
            let x_2 = x_str[x_str.len() / 2..].parse().unwrap();
            num_after(x_1, steps - 1, map) + num_after(x_2, steps - 1, map)
        } else {
            num_after(num * 2024, steps - 1, map)
        }
    };

    map.insert((num, steps), result);
    result
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 190865);
    assert_eq!(part2(input), 225404711855335);
}

#[test]
fn examples() {
    let input = parse_input("0 1 10 99 999");
    assert_eq!(blink(&input, 1), 7);
    let input = parse_input("125 17");
    assert_eq!(blink(&input, 6), 22);
    assert_eq!(part1(input), 55312);
}
