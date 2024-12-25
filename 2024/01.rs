use advent::prelude::*;

type Input = (Vec<i32>, Vec<i32>);

fn parse_input(input: &str) -> Input {
    let mut l = Vec::new();
    let mut r = Vec::new();

    for line in input.lines() {
        let mut it = line.split_whitespace();
        l.push(it.next().unwrap().parse::<i32>().unwrap());
        r.push(it.next().unwrap().parse::<i32>().unwrap());
    }

    (l, r)
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2024 / 01));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1((mut l, mut r): Input) -> i32 {
    l.sort();
    r.sort();

    let mut dist = 0;
    for i in 0..l.len() {
        dist += (l[i] - r[i]).abs();
    }

    dist
}

fn part2((l, r): Input) -> i32 {
    let mut similarity = 0;
    for val in l {
        similarity += val as usize * r.iter().filter(|&&x| x == val).count();
    }

    similarity as i32
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
        assert_eq!(part1(input.clone()), 3246517);
        assert_eq!(part2(input), 29379307);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(part1(input.clone()), 11);
        assert_eq!(part2(input.clone()), 31);
    }
}
