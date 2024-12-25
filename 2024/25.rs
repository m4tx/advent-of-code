use advent::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    locks: Vec<[i64; 5]>,
    keys: Vec<[i64; 5]>,
}

fn parse_input(input: &str) -> Input {
    let mut data = Input {
        locks: Vec::new(),
        keys: Vec::new(),
    };

    let mut grid: Vec<String> = Vec::new();
    let input = format!("{}\n\n", input.trim());
    for line in input.lines() {
        if line.is_empty() {
            let is_lock = grid[0] == "#####";
            if !is_lock {
                grid.reverse();
            }
            let mut result = [0; 5];
            #[allow(clippy::needless_range_loop)]
            for x in 0..5 {
                for y in 1..6 {
                    if grid[y].as_bytes()[x] == b'#' {
                        result[x] += 1;
                    }
                }
            }

            if is_lock {
                data.locks.push(result);
            } else {
                data.keys.push(result);
            }

            grid.clear();
        } else {
            grid.push(line.to_owned());
        }
    }

    data
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2024 / 25));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> usize {
    input
        .keys
        .into_iter()
        .cartesian_product(input.locks)
        .filter(|(key, lock)| {
            for x in 0..5 {
                if key[x] + lock[x] > 5 {
                    return false;
                }
            }
            true
        })
        .count()
}

fn part2(_input: Input) -> &'static str {
    "no part 2 on day 25!"
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
        assert_eq!(part1(input), 3451);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
        );
        assert_eq!(part1(input), 3);
    }
}
