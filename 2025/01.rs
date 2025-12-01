use advent::prelude::*;

type Input = Vec<i64>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let num = &line[1..].parse::<i64>().unwrap();
            if line.starts_with('L') { -num } else { *num }
        })
        .collect()
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 01));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    let mut rot = 50;
    let mut count = 0;

    for step in input {
        rot += step;
        rot %= 100;
        if rot == 0 {
            count += 1;
        }
    }

    count
}

fn part2(input: Input) -> i64 {
    let mut rot = 50;
    let mut count = 0;

    for step in input {
        rot += step;

        if rot >= 100 {
            count += rot / 100;
        } else if rot < 0 {
            count += rot / -100;
            if rot - step > 0 {
                count += 1;
            }
        } else if rot == 0 {
            count += 1;
        }
        rot += 1000;
        rot %= 100;
    }

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
        assert_eq!(part1(input.clone()), 1059);
        assert_eq!(part2(input), 6305);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(part1(input.clone()), 3);
        assert_eq!(part2(input), 6);
    }
}
