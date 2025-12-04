use advent::prelude::*;

type Input = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|x| x.as_bytes().to_owned()).collect()
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 04));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(mut input: Input) -> i64 {
    remove_accessible(&mut input)
}

fn part2(mut input: Input) -> i64 {
    let mut total = 0;

    loop {
        let removed = remove_accessible(&mut input);
        if removed == 0 {
            break;
        }
        total += removed;
    }

    total
}

fn remove_accessible(input: &mut Input) -> i64 {
    let mut counter = 0;
    let mut to_remove = vec![];

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y].get(x) != Some(&b'@') {
                continue;
            }

            let mut rolls = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx, dy) == (0, 0) {
                        continue;
                    }

                    let x = x as i32 + dx;
                    let y = y as i32 + dy;

                    let Some(row) = input.get_mut(y as usize) else {
                        continue;
                    };
                    let Some(c) = row.get(x as usize) else {
                        continue;
                    };

                    if *c == b'@' {
                        rolls += 1;
                    }
                }
            }

            if rolls < 4 {
                counter += 1;
                to_remove.push((x, y));
            }
        }
    }

    for (x, y) in to_remove {
        *input.get_mut(y).unwrap().get_mut(x).unwrap() = b'.';
    }

    counter
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
        assert_eq!(part1(input.clone()), 1356);
        assert_eq!(part2(input), 8713);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert_eq!(part1(input.clone()), 13);
        assert_eq!(part2(input), 43);
    }
}
