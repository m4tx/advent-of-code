use advent::prelude::*;
use std::ops::{Add, Sub};
use std::str::FromStr;

type Input = Vec<String>;

fn parse_input(input: &str) -> Input {
    input.lines().map(str::to_owned).collect()
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 21))
}

fn part1(input: Input) -> i64 {
    input
        .iter()
        .map(|line| calc_sequence(line, 2) * i64::from_str(&line[0..line.len() - 1]).unwrap())
        .sum()
}

fn part2(input: Input) -> i64 {
    input
        .iter()
        .map(|line| calc_sequence(line, 25) * i64::from_str(&line[0..line.len() - 1]).unwrap())
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn with_x(&self) -> Point2D {
        Self::new(self.x, 0)
    }

    pub const fn with_y(&self) -> Point2D {
        Self::new(0, self.y)
    }
}

impl Add<Self> for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Self> for Point2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn calc_sequence(line: &str, num_keyboards: usize) -> i64 {
    let mut sequences = numeric_to_directional_keypad(line);
    retain_shortest(&mut sequences);

    let mut mem = HashMap::new();
    sequences
        .into_iter()
        .map(|seq| calc(seq, num_keyboards, &mut mem))
        .min()
        .unwrap()
}

fn retain_shortest(sequences: &mut Vec<String>) {
    let min_len = sequences.iter().map(String::len).min().unwrap();
    sequences.retain(|seq| seq.len() == min_len);
}

fn calc(seq: String, num_keyboards: usize, mem: &mut HashMap<(String, usize), i64>) -> i64 {
    if num_keyboards == 0 {
        return seq.len() as i64;
    }
    if let Some(&length) = mem.get(&(seq.clone(), num_keyboards)) {
        return length;
    }

    let mut pos = DirectionalKeypad.get_pos('A');

    let mut length = 0;
    for key in seq.chars() {
        let new_pos = DirectionalKeypad.get_pos(key);
        let diff = new_pos - pos;

        let x_seq = diff_to_seq(diff.with_x());
        let y_seq = diff_to_seq(diff.with_y());

        let mut min = i64::MAX;
        if x_seq.is_empty() || y_seq.is_empty() {
            let seq = format!("{}{}A", x_seq, y_seq);
            min = calc(seq, num_keyboards - 1, mem);
        } else {
            if pos + diff.with_x() != DirectionalKeypad.get_banned() {
                let seq = format!("{}{}A", x_seq, y_seq);
                min = min.min(calc(seq, num_keyboards - 1, mem));
            }
            if pos + diff.with_y() != DirectionalKeypad.get_banned() {
                let seq = format!("{}{}A", y_seq, x_seq);
                min = min.min(calc(seq, num_keyboards - 1, mem));
            }
        }

        pos = new_pos;
        length += min;
    }

    mem.insert((seq, num_keyboards), length);
    length
}

fn numeric_to_directional_keypad(line: &str) -> Vec<String> {
    let pos = NumericKeypad.get_pos('A');

    let mut sequences = Vec::new();
    calc_sequence_inner(line, pos, String::new(), &mut sequences, &NumericKeypad);
    sequences
}

fn calc_sequence_inner<K: Keypad>(
    line: &str,
    pos: Point2D,
    cur_seq: String,
    sequences: &mut Vec<String>,
    keypad: &K,
) {
    if line.is_empty() {
        sequences.push(cur_seq);
        return;
    }

    let key = line.chars().next().unwrap();
    let new_pos = keypad.get_pos(key);
    let diff = new_pos - pos;

    let x_seq = diff_to_seq(diff.with_x());
    let y_seq = diff_to_seq(diff.with_y());

    if x_seq.is_empty() || y_seq.is_empty() {
        let seq = format!("{}{}{}A", cur_seq, x_seq, y_seq);
        calc_sequence_inner(&line[1..], new_pos, seq, sequences, keypad);
    } else {
        if pos + diff.with_x() != keypad.get_banned() {
            let seq = format!("{}{}{}A", cur_seq, x_seq, y_seq);
            calc_sequence_inner(&line[1..], new_pos, seq, sequences, keypad);
        }
        if pos + diff.with_y() != keypad.get_banned() {
            let seq = format!("{}{}{}A", cur_seq, y_seq, x_seq);
            calc_sequence_inner(&line[1..], new_pos, seq, sequences, keypad);
        }
    }
}

fn diff_to_seq(diff: Point2D) -> String {
    assert!(diff.x == 0 || diff.y == 0);

    let mut seq = String::new();
    for _ in 0..diff.x.abs() {
        seq.push(if diff.x > 0 { '>' } else { '<' });
    }
    for _ in 0..diff.y.abs() {
        seq.push(if diff.y > 0 { 'v' } else { '^' });
    }
    seq
}

trait Keypad {
    fn get_pos(&self, key: char) -> Point2D;

    fn get_banned(&self) -> Point2D;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct NumericKeypad;
impl Keypad for NumericKeypad {
    fn get_pos(&self, key: char) -> Point2D {
        match key {
            '7' => Point2D::new(0, 0),
            '8' => Point2D::new(1, 0),
            '9' => Point2D::new(2, 0),
            '4' => Point2D::new(0, 1),
            '5' => Point2D::new(1, 1),
            '6' => Point2D::new(2, 1),
            '1' => Point2D::new(0, 2),
            '2' => Point2D::new(1, 2),
            '3' => Point2D::new(2, 2),
            '0' => Point2D::new(1, 3),
            'A' => Point2D::new(2, 3),
            _ => panic!("invalid key"),
        }
    }

    fn get_banned(&self) -> Point2D {
        Point2D::new(0, 3)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct DirectionalKeypad;
impl Keypad for DirectionalKeypad {
    fn get_pos(&self, key: char) -> Point2D {
        match key {
            '^' => Point2D::new(1, 0),
            'A' => Point2D::new(2, 0),
            '<' => Point2D::new(0, 1),
            'v' => Point2D::new(1, 1),
            '>' => Point2D::new(2, 1),
            _ => panic!("invalid key"),
        }
    }

    fn get_banned(&self) -> Point2D {
        Point2D::new(0, 0)
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 157230);
    assert_eq!(part2(input), 195969155897936);
}

#[test]
fn examples() {
    let input = parse_input(
        "029A
980A
179A
456A
379A",
    );
    assert_eq!(part1(input.clone()), 126384);
    assert_eq!(part2(input), 154115708116294);
}
