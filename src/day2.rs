pub fn part1() -> usize {
    let input = std::fs::read_to_string("input/day2").unwrap();
    let (x, y) = input
        .split('\n')
        .fold((0, 0), |(x, y), line| match line.split_once(' ') {
            Some((op, amount)) => match (op, amount.parse::<usize>().ok()) {
                ("forward", Some(amount)) => (x + amount, y),
                ("down", Some(amount)) => (x, y + amount),
                ("up", Some(amount)) => (x, y - amount),
                _ => panic!(),
            },
            None => panic!(),
        });
    x * y
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string("input/day2").unwrap();
    let (x, y, _) =
        input
            .split('\n')
            .fold((0, 0, 0), |(x, y, aim), line| match line.split_once(' ') {
                Some((op, amount)) => match (op, amount.parse::<usize>().ok()) {
                    ("forward", Some(amount)) => (x + amount, y + aim * amount, aim),
                    ("down", Some(amount)) => (x, y, aim + amount),
                    ("up", Some(amount)) => (x, y, aim - amount),
                    _ => panic!(),
                },
                None => panic!(),
            });
    x * y
}
