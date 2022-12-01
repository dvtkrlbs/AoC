use itertools::Itertools;

pub fn run(input: &str) -> i64 {
    input
        .split('\n')
        .group_by(|line| *line == "")
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| group.map(|val| val.parse::<i64>().unwrap()).sum())
        .max()
        .unwrap()
}
