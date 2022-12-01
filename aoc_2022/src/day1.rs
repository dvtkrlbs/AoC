use itertools::Itertools;

fn atoi(bytes: &[u8]) -> i64 {
    let mut result = 0;
    for byte in bytes {
        result = result * 10 + (byte - b'0') as i64;
    }
    result
}

#[aoc(day1, part1)]
pub fn day1_part1(input: &[u8]) -> i64 {
    input
        .split(|b| *b == b'\n')
        .group_by(|line| *line == b"")
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| group.map(|val| atoi(val)).sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn day1_part2(input: &[u8]) -> i64 {
    let res = input
        .split(|b| *b == b'\n')
        .group_by(|line| *line == b"")
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| group.map(|val| atoi(val)).sum::<i64>())
        .fold((0, 0, 0), |acc, x| {
            if x > acc.0 {
                (x, acc.0, acc.1)
            } else if x > acc.1 {
                (acc.0, x, acc.1)
            } else if x > acc.2 {
                (acc.0, acc.1, x)
            } else {
                acc
            }
        });

    res.0 + res.1 + res.2
}
