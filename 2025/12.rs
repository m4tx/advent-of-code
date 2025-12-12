use advent::prelude::*;

#[derive(Debug, Clone)]
struct Input {
    shapes: Vec<Shape>,
    queries: Vec<Query>,
}

#[derive(Debug, Clone)]
struct Shape {
    _grid: Vec<Vec<bool>>,
    filled: i64,
}

impl Shape {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        Self {
            filled: Self::filled(&grid),
            _grid: grid,
        }
    }

    fn filled(grid: &[Vec<bool>]) -> i64 {
        grid.iter()
            .map(|row| row.iter().filter(|&&cell| cell).count() as i64)
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Query {
    width: i64,
    height: i64,
    shape_counts: Vec<i64>,
}

fn parse_input(input: &str) -> Input {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut index = 0;
    let mut shapes = vec![];

    for _ in 0..6 {
        let mut shape = vec![];
        index += 1; // skip shape index line
        for _ in 0..3 {
            let line = lines[index];
            index += 1;
            let row = line
                .trim()
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("invalid character in shape"),
                })
                .collect::<Vec<bool>>();
            shape.push(row);
        }
        shapes.push(Shape::new(shape));
        index += 1; // skip empty line
    }

    let mut queries = vec![];
    while index < lines.len() {
        let line = lines[index];
        index += 1;
        let mut parts = line.split(':');
        let size_part = parts.next().unwrap().trim();
        let counts_part = parts.next().unwrap().trim();

        let mut size_parts = size_part.split('x');
        let width: i64 = size_parts.next().unwrap().trim().parse().unwrap();
        let height: i64 = size_parts.next().unwrap().trim().parse().unwrap();

        let shape_counts = counts_part
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        queries.push(Query {
            width,
            height,
            shape_counts,
        });
    }

    Input { shapes, queries }
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 12));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    input
        .queries
        .iter()
        .map(|query| {
            let total_area = query.width * query.height;
            let filled = input
                .shapes
                .iter()
                .zip(&query.shape_counts)
                .map(|(shape, count)| shape.filled * count)
                .sum::<i64>();

            if filled > total_area { 0 } else { 1 }
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).build();
    solution.cli()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn default() {
        let input = default_input();
        assert_eq!(part1(input.clone()), 427);
    }
}
