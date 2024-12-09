use advent::prelude::*;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

type Input = Disk;

fn parse_input(input: &str) -> Input {
    let input = input.trim();
    let mut disk: Vec<_> = Vec::new();

    for i in 0..input.len() {
        let size = i64::from_str(&input[i..i + 1]).unwrap();
        if i % 2 == 0 {
            disk.push(Node::new(DiskBlock::File(i as i64 / 2), size));
        } else {
            disk.push(Node::new(DiskBlock::Free, size));
        }
    }

    Disk(disk)
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 09))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Disk(Vec<Node>);

impl Deref for Disk {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Disk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Disk {
    fn to_raw(&self) -> Vec<DiskBlock> {
        let mut raw = Vec::new();
        for node in self.iter() {
            raw.extend(iter::repeat_n(node.block, node.size as usize));
        }

        raw
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    block: DiskBlock,
    size: i64,
}

impl Node {
    pub fn new(block: DiskBlock, size: i64) -> Self {
        Self { block, size }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum DiskBlock {
    File(i64),
    Free,
}

fn part1(input: Input) -> i64 {
    let mut disk = input.to_raw();

    let mut left = 0;
    let mut right = disk.len() - 1;
    while left < right {
        if disk[left] != DiskBlock::Free {
            left += 1;
        } else if disk[right] == DiskBlock::Free {
            right -= 1;
        } else {
            disk.swap(left, right);
        }
    }

    checksum(&disk)
}

fn part2(mut disk: Input) -> i64 {
    for file_index in (0..disk.len()).rev() {
        if disk[file_index].block == DiskBlock::Free {
            continue;
        }

        let first_suitable_free = disk[0..file_index]
            .iter()
            .enumerate()
            .filter(|(index, node)| {
                node.block == DiskBlock::Free && node.size >= disk[file_index].size
            })
            .next();

        if let Some((free_index, free_node)) = first_suitable_free {
            let free = free_node.size - disk[file_index].size;
            disk.swap(file_index, free_index);

            if free > 0 {
                disk[file_index].size -= free;
                disk.insert(free_index + 1, Node::new(DiskBlock::Free, free));
            }
        }
    }

    checksum(&disk.to_raw())
}

fn checksum(disk: &Vec<DiskBlock>) -> i64 {
    disk.iter()
        .enumerate()
        .map(|(index, val)| match *val {
            DiskBlock::File(id) => index as i64 * id,
            DiskBlock::Free => 0,
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 6463499258318);
    assert_eq!(part2(input), 6493634986625);
}

#[test]
fn examples() {
    let input = parse_input("2333133121414131402");
    assert_eq!(part1(input.clone()), 1928);
    assert_eq!(part2(input), 2858);
}
