use itertools::Itertools;

#[aoc(day1, part1)]
pub fn day1_part1(input: &str) -> i64 {
    input
        .split('\n')
        .group_by(|line| *line == "")
        .into_iter()
        .map(|(_, group)| group.map(|val| val.parse::<i64>().unwrap_or(0)).sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn day1_part2(input: &str) -> i64 {
    input
        .split('\n')
        .group_by(|line| *line == "")
        .into_iter()
        .map(|(_, group)| {
            group
                .map(|val| val.parse::<i64>().unwrap_or(0))
                .sum::<i64>()
        })
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .take(3)
        .sum::<i64>()
}
