use itertools::Itertools;

pub fn run(input: &str) -> i64 {
    input
        .split('\n')
        .group_by(|line| *line == "")
        .into_iter()
        .map(|(_, group)| group.map(|val| val.parse::<i64>().unwrap_or(0)).sum())
        .max()
        .unwrap()
}
