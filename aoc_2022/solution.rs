use itertools::Itertools;

pub fn run(input: &[u8]) -> i64 {
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
