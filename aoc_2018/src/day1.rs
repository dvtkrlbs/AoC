use std::collections::HashMap;


#[aoc_generator(day1)]
pub fn to_vec_of_int(input: &str) -> Vec<i32> {
    input
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[aoc(day1, part1)]
pub fn day1_part1_solve(input: &[i32]) -> i32 {
    input
        .iter()
        .sum()
}

#[aoc(day1, part2)]
pub fn day1_part2_solve(input: &[i32]) -> i32 {
    let mut lookup = HashMap::new();
    let mut freq = 0;

    for num in input.iter().cycle() {
        freq += num;
        match lookup.get(&freq) {
                Some(_) => break,
                None => lookup.insert(freq, 0),
        };
    }

    freq
}