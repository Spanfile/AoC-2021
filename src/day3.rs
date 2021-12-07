use std::collections::HashSet;

pub fn part1() -> usize {
    let input = std::fs::read_to_string("input/day3")
        .unwrap()
        .split('\n')
        .map(|s| u16::from_str_radix(s, 2))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let (gamma, epsilon) = (0..12).fold((0, 0), |(gamma, epsilon), bit| {
        if input.iter().filter(|n| *n & (1 << bit) > 0).count() > input.len() / 2 {
            (gamma | 1 << bit, epsilon)
        } else {
            (gamma, epsilon | 1 << bit)
        }
    });

    gamma * epsilon
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string("input/day3")
        .unwrap()
        .split('\n')
        .map(|s| u16::from_str_radix(s, 2))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut oxygen_candidates = HashSet::from_iter(input.iter().copied());
    let mut co2_candidates = HashSet::from_iter(input.iter().copied());

    for bit in (0..12).rev() {
        if oxygen_candidates.len() > 1 {
            let (oxygen_ones, oxygen_zeros): (HashSet<u16>, HashSet<u16>) = oxygen_candidates
                .into_iter()
                .partition(|n| *n & (1 << bit) > 0);

            oxygen_candidates = if oxygen_ones.len() >= oxygen_zeros.len() {
                oxygen_ones
            } else {
                oxygen_zeros
            };
        }

        if co2_candidates.len() > 1 {
            let (co2_ones, co2_zeros): (HashSet<u16>, HashSet<u16>) = co2_candidates
                .into_iter()
                .partition(|n| *n & (1 << bit) > 0);

            co2_candidates = if co2_ones.len() >= co2_zeros.len() {
                co2_zeros
            } else {
                co2_ones
            };
        }

        println!(
            "bit {}: oxygen {}, co2 {}",
            bit,
            oxygen_candidates.len(),
            co2_candidates.len()
        );
    }

    let oxygen = oxygen_candidates.drain().next().unwrap() as usize;
    let co2 = co2_candidates.drain().next().unwrap() as usize;

    oxygen * co2
}
