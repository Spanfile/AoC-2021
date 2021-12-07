#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

const VICTORIES: &[&[u8]] = &[
    // rows
    &[0, 1, 2, 3, 4],
    &[5, 6, 7, 8, 9],
    &[10, 11, 12, 13, 14],
    &[15, 16, 17, 18, 19],
    &[20, 21, 22, 23, 24],
    // columns
    &[0, 5, 10, 15, 20],
    &[1, 6, 11, 16, 21],
    &[2, 7, 12, 17, 22],
    &[3, 8, 13, 18, 23],
    &[4, 9, 14, 19, 24],
];

#[derive(Debug)]
struct Board {
    nums: Vec<u8>,
}

impl Board {
    fn has_won(&self, marks: &HashSet<u8>) -> bool {
        let marked_indices = self
            .nums
            .iter()
            .enumerate()
            .filter_map(|(i, num)| {
                if marks.contains(num) {
                    Some(i as u8)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        for condition in VICTORIES {
            if condition.iter().all(|n| marked_indices.contains(n)) {
                return true;
            }
        }

        false
    }

    fn score(&self, marks: &HashSet<u8>, called_number: u8) -> u32 {
        self.nums
            .iter()
            .filter_map(|n| {
                if !marks.contains(n) {
                    Some(*n as u32)
                } else {
                    None
                }
            })
            .sum::<u32>()
            * called_number as u32
    }
}

fn generator() -> (Vec<u8>, Vec<Board>) {
    let input = std::fs::read_to_string("input/day4").unwrap();
    let mut args = input.split("\n\n");

    let nums = args
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let boards = args
        .map(|board_str| Board {
            nums: board_str
                .split(char::is_whitespace)
                .filter_map(|s| if !s.is_empty() { Some(s.parse()) } else { None })
                .collect::<Result<_, _>>()
                .unwrap(),
        })
        .collect();

    (nums, boards)
}

pub fn part1() -> u32 {
    let (nums, boards) = generator();
    let mut marks = HashSet::from_iter(nums.iter().take(5).copied());

    for called in nums.iter().skip(5) {
        marks.insert(*called);

        for board in &boards {
            if board.has_won(&marks) {
                return board.score(&marks, *called);
            }
        }
    }

    panic!("no victories")
}

pub fn part2() -> u32 {
    let (nums, boards) = generator();
    let mut boards: HashMap<usize, Board> = HashMap::from_iter(boards.into_iter().enumerate());
    let mut marks = HashSet::from_iter(nums.iter().take(5).copied());
    let mut last_victory = 0;

    for called in nums.iter().skip(5) {
        marks.insert(*called);

        boards.retain(|i, board| {
            if board.has_won(&marks) {
                last_victory = board.score(&marks, *called);
                false
            } else {
                true
            }
        });
    }

    last_victory
}
