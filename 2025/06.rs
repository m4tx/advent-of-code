use advent::prelude::*;

#[derive(Debug, Clone)]
struct Input {
    columns: Vec<Column>,
}

#[derive(Debug, Clone)]
struct Column {
    width: usize,
    operation: MathOp,
    numbers: Vec<i64>,
    numbers_vert: Vec<i64>,
}

impl Column {
    fn new(width: usize, operation: MathOp) -> Self {
        Column {
            width,
            operation,
            numbers: vec![],
            numbers_vert: vec![],
        }
    }
}

#[derive(Debug, Clone)]
enum MathOp {
    Add,
    Multiply,
}

fn parse_input(input: &str) -> Input {
    let lines: Vec<_> = input.lines().collect();
    let mut columns = prepare_columns(&lines);

    for line in lines.iter().take(lines.len() - 1) {
        for (col_idx, part) in line.split_whitespace().enumerate() {
            columns[col_idx].numbers.push(part.parse::<i64>().unwrap());
        }
    }

    let mut offset = 0;
    for col in &mut columns {
        for x in 0..col.width {
            let mut num = String::new();
            for idx in 0..lines.len() - 1 {
                num.push(lines[idx].as_bytes()[offset + x] as char);
            }
            col.numbers_vert.push(num.trim().parse::<i64>().unwrap());
        }

        offset += col.width + 1;
    }

    Input { columns }
}

fn prepare_columns(lines: &Vec<&str>) -> Vec<Column> {
    let mut columns = Vec::new();

    let mut width = 0;
    let mut op = None;
    for ch in lines.last().unwrap().chars() {
        if ch.is_whitespace() {
            width += 1;
        } else {
            if op.is_some() {
                columns.push(Column::new(width, op.take().unwrap()));
                width = 0;
                op = None;
            }

            if ch == '+' {
                op = Some(MathOp::Add);
            } else if ch == '*' {
                op = Some(MathOp::Multiply);
            }
        }
    }
    columns.push(Column::new(width + 1, op.take().unwrap()));

    columns
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 06));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    solve_part(input, |col| &col.numbers)
}

fn part2(input: Input) -> i64 {
    solve_part(input, |col| &col.numbers_vert)
}

fn solve_part(input: Input, numbers: fn(&Column) -> &[i64]) -> i64 {
    input
        .columns
        .into_iter()
        .map(|col| match col.operation {
            MathOp::Add => numbers(&col).iter().sum::<i64>(),
            MathOp::Multiply => numbers(&col).iter().product(),
        })
        .sum()
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
        assert_eq!(part1(input.clone()), 4405895212738);
        assert_eq!(part2(input), 7450962489289);
    }

    #[test]
    fn examples() {
        let input = parse_input(concat!(
            "123 328  51 64 \n",
            " 45 64  387 23 \n",
            "  6 98  215 314\n",
            "*   +   *   +  "
        ));
        assert_eq!(part1(input.clone()), 4277556);
        assert_eq!(part2(input), 3263827);
    }
}
