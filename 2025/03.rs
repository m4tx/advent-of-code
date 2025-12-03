use advent::prelude::*;

type Input = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect()
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 03));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    solve(input, 2)
}

fn part2(input: Input) -> i64 {
    solve(input, 12)
}

#[inline(always)]
fn solve(input: Input, digit_num: usize) -> i64 {
    input.iter().map(|s| get_best_value(s, digit_num)).sum()
}

#[inline(always)]
fn get_best_value(s: &[u8], digit_num: usize) -> i64 {
    let mut digits = String::with_capacity(digit_num);

    let mut last_index = 0;
    for digit in 0..digit_num {
        let index = get_max(&s, last_index, s.len() - (digit_num - digit));
        digits.push(s[index] as char);
        last_index = index + 1;
    }

    digits.parse().unwrap()
}

#[inline(always)]
fn get_max(s: &[u8], start: usize, end: usize) -> usize {
    let mut max_char = b'\0';
    let mut max_char_index = start;

    for i in start..=end {
        let current_char = s[i];
        if current_char > max_char {
            max_char = current_char;
            max_char_index = i;
        }
        if max_char == b'9' {
            break;
        }
    }

    max_char_index
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
        assert_eq!(part1(input.clone()), 17493);
        assert_eq!(part2(input), 173685428989126);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(part1(input.clone()), 357);
        assert_eq!(part2(input), 3121910778619);
    }
}
