#![allow(unused_variables)]
#![allow(dead_code)]

use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn is_horizontal(self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(self) -> bool {
        self.start.0 == self.end.0
    }

    fn points(self) -> Vec<(i32, i32)> {
        let x_step = match self.start.0.cmp(&self.end.0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let y_step = match self.start.1.cmp(&self.end.1) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let mut points = Vec::new();
        let mut cur = self.start;
        while cur != self.end {
            points.push(cur);
            cur = (cur.0 + x_step, cur.1 + y_step);
        }

        points.push(cur);
        points
    }
}

fn generator() -> Vec<Line> {
    let input = std::fs::read_to_string("input/day5").unwrap();
    input
        .lines()
        .map(|l| {
            l.split_once(" -> ")
                .map(|(left, right)| Line {
                    start: (left
                        .split_once(',')
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                        .unwrap()),
                    end: (right
                        .split_once(',')
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())))
                    .unwrap(),
                })
                .unwrap()
        })
        .collect()
}

pub fn part1() -> usize {
    let lines = generator();
    let mut points = HashMap::new();

    for l in lines
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
    {
        for point in l.points() {
            let count = points.entry(point).or_insert(0);
            *count += 1;
        }
    }

    points.values().filter(|v| **v > 1).count()
}

pub fn part2() -> usize {
    let lines = generator();
    let mut points = HashMap::new();

    for l in lines {
        for point in l.points() {
            let count = points.entry(point).or_insert(0);
            *count += 1;
        }
    }

    points.values().filter(|v| **v > 1).count()
}
