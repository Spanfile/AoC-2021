use std::cmp::Ordering;

pub fn generator() -> Vec<i32> {
    let input = std::fs::read_to_string("input/day1").unwrap();
    input
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part1() -> usize {
    let input = generator();
    input
        .windows(2)
        .filter(|s| s[1].cmp(&s[0]) == Ordering::Greater)
        .count()
}

pub fn part2() -> usize {
    let input = generator();
    let windows: Vec<i32> = input.windows(3).map(|s| s.iter().sum()).collect();
    windows
        .windows(2)
        .filter(|s| s[1].cmp(&s[0]) == Ordering::Greater)
        .count()
}
