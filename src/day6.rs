fn generator() -> Vec<u8> {
    let input = std::fs::read_to_string("input/day6").unwrap();
    input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn simulate(steps: usize) -> usize {
    let fishies = generator();
    let mut fish_ages = [0usize; 9];

    for age in fishies {
        fish_ages[age as usize] += 1;
    }

    for _ in 0..steps {
        let mut new_ages = [0usize; 9];
        let zero_ages = fish_ages[0];

        new_ages[0..8].clone_from_slice(&fish_ages[1..9]);
        new_ages[6] += zero_ages;
        new_ages[8] = zero_ages;

        fish_ages = new_ages;
    }

    fish_ages.iter().sum()
}

pub fn part1() -> usize {
    simulate(80)
}

pub fn part2() -> usize {
    simulate(256)
}
