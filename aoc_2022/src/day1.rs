use itertools::Itertools;

#[aoc(day1, part1)]
pub fn day1_part1(input: &[u8]) -> i64 {
    input
        .split(|b| *b == b'\n')
        .group_by(|line| *line == b"")
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            group
                .map(|val| atoi_radix10::parse::<i64>(val).unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn day1_part2(input: &[u8]) -> i64 {
    input
        .split(|b| *b == b'\n')
        .group_by(|line| *line == b"")
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            group
                .map(|val| atoi_radix10::parse::<i64>(val).unwrap())
                .sum::<i64>()
        })
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .take(3)
        .sum::<i64>()
}
