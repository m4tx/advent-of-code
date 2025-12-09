use advent::prelude::*;

type Input = Vec<(i64, i64)>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let numbers = line.split(",");
            let mut iter = numbers.into_iter();
            let x: i64 = iter.next().unwrap().trim().parse().unwrap();
            let y: i64 = iter.next().unwrap().trim().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 09));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    let mut max_size = 0;

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let (x1, y1) = input[i];
            let (x2, y2) = input[j];

            let size = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            if size > max_size {
                max_size = max_size.max(size);
            }
        }
    }

    max_size
}

fn part2(input: Input) -> i64 {
    let compressed_x = compress_coordinates(input.iter().map(|(x, _)| *x).collect());
    let compressed_y = compress_coordinates(input.iter().map(|(_, y)| *y).collect());

    let mut space = vec![vec![None; compressed_x.len()]; compressed_y.len()];
    fill_lines(&input, &compressed_x, &compressed_y, &mut space);
    fill_inside(&mut space);

    let mut max_size = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let (x1, y1) = input[i];
            let (x2, y2) = input[j];

            let size = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            if size > max_size {
                let min_x = compressed_x[&x1.min(x2)];
                let min_y = compressed_y[&y1.min(y2)];
                let max_x = compressed_x[&x1.max(x2)];
                let max_y = compressed_y[&y1.max(y2)];

                if valid_rect(&space, min_x, min_y, max_x, max_y) {
                    max_size = max_size.max(size);
                }
            }
        }
    }

    max_size
}

fn compress_coordinates(mut coords: Vec<i64>) -> HashMap<i64, usize> {
    coords.sort_unstable();
    coords.dedup();

    let mut compressed = HashMap::new();
    for (i, &coord) in coords.iter().enumerate() {
        compressed.insert(coord, i);
    }

    compressed
}

fn fill_lines(
    input: &Input,
    compressed_x: &HashMap<i64, usize>,
    compressed_y: &HashMap<i64, usize>,
    space: &mut [Vec<Option<usize>>],
) {
    for i in 0..input.len() {
        let (x1, y1) = input[i];
        let (x2, y2) = input[(i + 1) % input.len()];

        let cx1 = compressed_x[&x1];
        let cy1 = compressed_y[&y1];
        let cx2 = compressed_x[&x2];
        let cy2 = compressed_y[&y2];

        if cx1 == cx2 {
            for y in cy1.min(cy2) + 1..cy1.max(cy2) {
                space[y][cx1] = Some(i);
            }
        } else if cy1 == cy2 {
            for x in cx1.min(cx2)..=cx1.max(cx2) {
                space[cy1][x] = Some(i);
            }
        } else {
            panic!("invalid input");
        }
    }
}

fn fill_inside(space: &mut [Vec<Option<usize>>]) {
    for y in 0..space.len() {
        let mut fill = None;

        for x in 0..space[0].len() {
            if space[y][x].is_some() {
                if fill.is_none() {
                    fill = space[y][x];
                } else if fill != space[y][x] {
                    fill = None;
                }
            } else if fill.is_some() {
                space[y][x] = fill;
            }
        }
    }
}

fn valid_rect(
    space: &[Vec<Option<usize>>],
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
) -> bool {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if space[y][x].is_none() {
                return false;
            }
        }
    }

    true
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
        assert_eq!(part1(input.clone()), 4781546175);
        assert_eq!(part2(input), 1573359081);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(part1(input.clone()), 50);
        assert_eq!(part2(input), 24);
    }
}
