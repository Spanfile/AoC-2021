use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> usize {
    input
        .windows(2)
        .filter(|s| s[1].cmp(&s[0]) == Ordering::Greater)
        .count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> usize {
    let windows: Vec<i32> = input.windows(3).map(|s| s.iter().sum()).collect();
    windows
        .windows(2)
        .filter(|s| s[1].cmp(&s[0]) == Ordering::Greater)
        .count()
}
