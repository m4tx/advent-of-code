use advent::prelude::*;
use std::ops::{Add, AddAssign};

trait GridExt {
    fn size(&self) -> (usize, usize);
}

impl<T> GridExt for Vec<Vec<T>> {
    fn size(&self) -> (usize, usize) {
        (self[0].len(), self.len())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    grid: Vec<Vec<Item>>,
    moves: Vec<Move>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Item {
    Empty,
    Wall,
    Box,
    BoxWide1,
    BoxWide2,
    Player,
}

impl Item {
    fn neighbor_offset(&self) -> Point2D {
        match self {
            Item::BoxWide1 => Point2D::new(1, 0),
            Item::BoxWide2 => Point2D::new(-1, 0),
            _ => panic!("invalid item"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn to_point(self) -> Point2D {
        match self {
            Move::Up => Point2D::new(0, -1),
            Move::Right => Point2D::new(1, 0),
            Move::Down => Point2D::new(0, 1),
            Move::Left => Point2D::new(-1, 0),
        }
    }

    fn is_horizontal(self) -> bool {
        match self {
            Move::Up | Move::Down => false,
            Move::Right | Move::Left => true,
        }
    }

    fn is_vertical(self) -> bool {
        !self.is_horizontal()
    }
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

impl AddAssign<Self> for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

fn parse_input(input: &str) -> Input {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum Mode {
        Grid,
        Moves,
    }

    let mut mode = Mode::Grid;
    let mut grid = Vec::new();
    let mut moves = Vec::new();
    for line in input.lines() {
        match mode {
            Mode::Grid => {
                if line.is_empty() {
                    mode = Mode::Moves;
                    continue;
                }

                let row: Vec<_> = line
                    .chars()
                    .map(|c| match c {
                        '.' => Item::Empty,
                        '#' => Item::Wall,
                        'O' => Item::Box,
                        '@' => Item::Player,
                        _ => panic!("invalid item: {}", c),
                    })
                    .collect();
                grid.push(row);
            }
            Mode::Moves => {
                let moves_line: Vec<_> = line
                    .chars()
                    .map(|c| match c {
                        '^' => Move::Up,
                        '>' => Move::Right,
                        'v' => Move::Down,
                        '<' => Move::Left,
                        _ => panic!("invalid move: {}", c),
                    })
                    .collect();
                moves.extend(moves_line);
            }
        }
    }

    Input { grid, moves }
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 15))
}

fn part1(mut input: Input) -> i64 {
    let mut player = find_player(&mut input.grid);
    for player_move in input.moves {
        if can_move(player_move, &mut input.grid, player) {
            handle_move(player_move, &mut input.grid, player);
            player += player_move.to_point();
        }
    }

    calculate_score(&input.grid)
}

fn calculate_score(grid: &Vec<Vec<Item>>) -> i64 {
    let (size_x, size_y) = grid.size();

    (0..size_x)
        .cartesian_product(0..size_y)
        .filter(|&(x, y)| grid[y][x] == Item::Box || grid[y][x] == Item::BoxWide1)
        .map(|(x, y)| 100 * y as i64 + x as i64)
        .sum()
}

fn find_player(grid: &mut [Vec<Item>]) -> Point2D {
    for y in grid.iter_mut().enumerate() {
        for x in y.1.iter_mut().enumerate() {
            if *x.1 == Item::Player {
                *x.1 = Item::Empty;
                return Point2D::new(x.0 as i64, y.0 as i64);
            }
        }
    }

    panic!("player not found")
}

fn part2(mut input: Input) -> i64 {
    let (_, size_y) = input.grid.size();
    for y in 0..size_y {
        let row = &mut input.grid[y];
        let mut new_row = Vec::new();

        for item in row.iter() {
            match item {
                Item::Empty => {
                    new_row.push(Item::Empty);
                    new_row.push(Item::Empty);
                }
                Item::Wall => {
                    new_row.push(Item::Wall);
                    new_row.push(Item::Wall);
                }
                Item::Box => {
                    new_row.push(Item::BoxWide1);
                    new_row.push(Item::BoxWide2);
                }
                Item::Player => {
                    new_row.push(Item::Player);
                    new_row.push(Item::Empty);
                }
                _ => {}
            }
        }

        *row = new_row;
    }

    part1(input)
}

fn can_move(player_move: Move, grid: &mut Vec<Vec<Item>>, player: Point2D) -> bool {
    let next_pos = player + player_move.to_point();
    let next_item = grid[next_pos.y as usize][next_pos.x as usize];

    if next_item == Item::Wall {
        false
    } else if next_item == Item::Empty {
        true
    } else if player_move.is_vertical()
        && (next_item == Item::BoxWide1 || next_item == Item::BoxWide2)
    {
        can_move(player_move, grid, next_pos)
            && can_move(player_move, grid, next_pos + next_item.neighbor_offset())
    } else if next_item == Item::Box || next_item == Item::BoxWide1 || next_item == Item::BoxWide2 {
        can_move(player_move, grid, next_pos)
    } else {
        unreachable!("invalid item")
    }
}

fn handle_move(player_move: Move, grid: &mut Vec<Vec<Item>>, position: Point2D) {
    let next_pos = position + player_move.to_point();
    let next_item = grid[next_pos.y as usize][next_pos.x as usize];
    let current_item = grid[position.y as usize][position.x as usize];

    if player_move.is_vertical() && (next_item == Item::BoxWide1 || next_item == Item::BoxWide2) {
        handle_move(player_move, grid, next_pos);
        handle_move(player_move, grid, next_pos + next_item.neighbor_offset());
        move_item(grid, position, next_pos, current_item);
    } else if next_item == Item::Box || next_item == Item::BoxWide1 || next_item == Item::BoxWide2 {
        handle_move(player_move, grid, next_pos);
        move_item(grid, position, next_pos, current_item);
    } else if next_item == Item::Empty {
        move_item(grid, position, next_pos, current_item);
    } else {
        unreachable!("invalid item");
    }
}

fn move_item(grid: &mut [Vec<Item>], curr: Point2D, next: Point2D, current_item: Item) {
    grid[next.y as usize][next.x as usize] = current_item;
    grid[curr.y as usize][curr.x as usize] = Item::Empty;
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1446158);
    assert_eq!(part2(input), 1446175);
}

#[test]
fn examples() {
    let input = parse_input(
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
    );
    assert_eq!(part1(input.clone()), 2028);

    let input = parse_input(
        "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
    );
    assert_eq!(part1(input.clone()), 908);
    assert_eq!(part2(input), 618);

    let input = parse_input(
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    );
    assert_eq!(part1(input.clone()), 10092);
    assert_eq!(part2(input), 9021);
}
