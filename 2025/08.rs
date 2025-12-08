use advent::prelude::*;

type Input = Vec<Point>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    repr: usize,
    size: usize,
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let mut parts = line
                .split(',')
                .map(|val| val.trim().parse::<i64>().unwrap());
            let x: i64 = parts.next().unwrap();
            let y: i64 = parts.next().unwrap();
            let z: i64 = parts.next().unwrap();
            Point {
                x,
                y,
                z,
                repr: idx,
                size: 1,
            }
        })
        .collect()
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 08));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    solve(input, Policy::NFirst(1000))
}

fn part2(input: Input) -> i64 {
    solve(input, Policy::FinalMerge)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Policy {
    NFirst(usize),
    FinalMerge,
}

#[inline(always)]
fn solve(mut input: Input, policy: Policy) -> i64 {
    let mut dists = vec![];

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let dist = (input[i].x - input[j].x).pow(2)
                + (input[i].y - input[j].y).pow(2)
                + (input[i].z - input[j].z).pow(2);
            dists.push((dist, i, j));
        }
    }

    dists.sort_unstable();

    let it = if let Policy::NFirst(n) = policy {
        &dists[0..n]
    } else {
        dists.as_slice()
    };
    for &(_dist, i, j) in it {
        let mut repr_i = input[i].repr;
        while repr_i != input[repr_i].repr {
            repr_i = input[repr_i].repr;
        }

        let mut repr_j = input[j].repr;
        while repr_j != input[repr_j].repr {
            repr_j = input[repr_j].repr;
        }

        if repr_i != repr_j {
            let size_i = input[repr_i].size;
            let size_j = input[repr_j].size;

            if policy == Policy::FinalMerge && size_i + size_j == input.len() {
                return input[i].x * input[j].x;
            }

            if size_i < size_j {
                input[repr_i].repr = repr_j;
                input[repr_j].size += size_i;
            } else {
                input[repr_j].repr = repr_i;
                input[repr_i].size += size_j;
            }
        }
    }

    input.sort_unstable_by_key(|p| -(p.size as i64));
    input.iter().take(3).map(|p| p.size).product::<usize>() as i64
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
        assert_eq!(part1(input.clone()), 79560);
        assert_eq!(part2(input), 31182420);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
",
        );
        assert_eq!(solve(input.clone(), Policy::NFirst(10)), 40);
        assert_eq!(part2(input), 25272);
    }
}
