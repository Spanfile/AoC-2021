#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashSet;

// 1 = 2
// 4 = 4
// 7 = 3
// 8 = 7

pub fn part1() -> usize {
    let input = std::fs::read_to_string("input/day8").unwrap();
    input
        .lines()
        .map(|l| {
            l.split_once(" | ")
                .map(|(_, right)| {
                    right
                        .split(' ')
                        .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                        .count()
                })
                .unwrap()
        })
        .sum()
}

const fn char_num(c: char) -> u8 {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!(),
    }
}

pub fn part2() -> i32 {
    let input = std::fs::read_to_string("input/day8").unwrap();
    input
        .lines()
        .map(|l| {
            l.split_once(" | ")
                .map(|(left, right)| {
                    let mut segments = vec![0; 7];
                    left.split(' ')
                        .map(|s| {
                            (
                                s.len(),
                                s.chars().fold(0u8, |acc, c| acc | 1 << char_num(c)),
                            )
                        })
                        .for_each(|(len, sig)| match len {
                            2 => {
                                // 1
                                segments[0] &= !sig & 0b0111_1111;
                                segments[1] &= !sig & 0b0111_1111;
                                segments[2] |= sig;
                                segments[3] &= !sig & 0b0111_1111;
                                segments[4] &= !sig & 0b0111_1111;
                                segments[5] |= sig;
                                segments[6] &= !sig & 0b0111_1111;
                            }
                            3 => {
                                // 7
                                segments[0] |= sig;
                                segments[1] &= !sig & 0b0111_1111;
                                segments[2] |= sig;
                                segments[3] &= !sig & 0b0111_1111;
                                segments[4] &= !sig & 0b0111_1111;
                                segments[5] |= sig;
                                segments[6] &= !sig & 0b0111_1111;
                            }
                            4 => {
                                // 4
                                segments[0] &= !sig & 0b0111_1111;
                                segments[1] |= sig;
                                segments[2] |= sig;
                                segments[3] |= sig;
                                segments[4] &= !sig & 0b0111_1111;
                                segments[5] |= sig;
                                segments[6] &= !sig & 0b0111_1111;
                            }
                            _ => (),
                        });

                    // println!("{:?}", segments);

                    right.split(' ').for_each(move |s| {
                        let mut selected_segments = HashSet::new();
                        s.chars().for_each(|c| {
                            let sig = 1 << char_num(c);
                            println!("{c} -> {sig}");
                            let (index, _) = segments
                                .iter()
                                .enumerate()
                                .find(|(index, segment)| {
                                    !selected_segments.contains(index) && *segment & sig > 0
                                })
                                .unwrap();
                            selected_segments.insert(index);
                        });
                        println!("{s} -> {selected_segments:?}");
                    });
                })
                .unwrap()
        })
        .for_each(drop);
    0
}
