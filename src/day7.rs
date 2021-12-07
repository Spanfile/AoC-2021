#![allow(unused_variables)]
#![allow(dead_code)]

fn generator() -> Vec<i32> {
    let input = std::fs::read_to_string("input/day7").unwrap();
    input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn part1() -> i32 {
    let crabs = generator();
    let max = *crabs.iter().max().unwrap();

    (0..=max)
        .map(|pos| crabs.iter().map(|c| (pos - c).abs()).sum())
        .min()
        .unwrap()
}

pub fn part2() -> i32 {
    let crabs = generator();
    let max = *crabs.iter().max().unwrap();

    (0..=max)
        .map(|pos| {
            crabs
                .iter()
                .map(|c| (0..=(pos - c).abs()).sum::<i32>())
                .sum()
        })
        .min()
        .unwrap()
}
