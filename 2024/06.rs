use advent::prelude::*;

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 06))
}

fn part1(input: Input) -> i64 {
    calc(input).unwrap()
}

#[allow(clippy::needless_range_loop)]
fn calc(mut input: Input) -> Option<i64> {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let x_len = input[0].len();
    let y_len = input.len();

    for x_pos in 0..x_len {
        for y_pos in 0..y_len {
            if input[y_pos][x_pos] == '^' {
                x = x_pos as i64;
                y = y_pos as i64;
            }
        }
    }

    input[y as usize][x as usize] = 'X';
    let mut direction = 0;
    let mut count = 1;
    let mut visited = HashSet::new();
    loop {
        let state = (x, y, direction);
        if visited.contains(&state) {
            return None;
        } else {
            visited.insert(state);
        }
        move_player(&mut x, &mut y, direction, 1);

        if y < 0 || x < 0 || x >= x_len as i64 || y >= y_len as i64 {
            break;
        }
        match input[y as usize][x as usize] {
            '#' => {
                move_player(&mut x, &mut y, direction, -1);
                direction = (direction + 1) % 4;
            }
            ch if ch != 'X' => {
                count += 1;
                input[y as usize][x as usize] = 'X';
            }
            _ => {}
        }
    }

    Some(count)
}

fn move_player(x: &mut i64, y: &mut i64, direction: i32, len: i64) {
    match direction {
        0 => {
            *y -= len;
        }
        1 => {
            *x += len;
        }
        2 => {
            *y += len;
        }
        3 => {
            *x -= len;
        }
        _ => unreachable!(),
    }
}

fn part2(input: Input) -> i64 {
    let x_len = input[0].len();
    let y_len = input.len();

    let mut count = 0;
    for x_pos in 0..x_len {
        for y_pos in 0..y_len {
            let mut new_input = input.clone();
            new_input[y_pos][x_pos] = '#';
            if calc(new_input).is_none() {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 5208);
    assert_eq!(part2(input), 1972);
}

#[test]
fn examples() {
    let input = parse_input(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
    );
    assert_eq!(part1(input.clone()), 41);
    assert_eq!(part2(input), 6);
}
