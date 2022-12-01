use itertools::Itertools;

fn atoi(bytes: &[u8]) -> i64 {
    let mut result = 0;
    for byte in bytes {
        result = result * 10 + (byte - b'0') as i64;
    }
    result
}

pub fn run(input: &[u8]) -> i64 {
    input
        .split(|b| *b == b'\n')
        .group_by(|line| *line == b"")
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| group.map(|val| atoi(val)).sum::<i64>())
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .take(3)
        .sum::<i64>()
}
