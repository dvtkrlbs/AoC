use std::collections::{HashMap, HashSet};


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

#[aoc(day1, part2, Vector)]
pub fn day1_part2_solve_vec(input: &[i32]) -> i32 {
    let mut lookup = Vec::new();
    let mut freq = 0;

    for num in input.iter().cycle() {
        freq += num;
        match lookup.contains(&freq) {
                true => break,
                false => lookup.push(freq),
        };
    }

    freq
}

#[aoc(day1, part2, HashMap)]
pub fn day1_part2_solve_hash_map(input: &[i32]) -> i32 {
    let mut lookup = HashMap::new();
    let mut freq = 0;

    for num in input.iter().cycle() {
        freq += num;
        match lookup.get(&freq) {
                Some(_) => break,
                None => lookup.insert(freq, true),
        };
    }

    freq
}

// https://github.com/zeyla s solution just here to compare speeds
#[aoc(day1, part2, HasSet)]
pub fn day1_part2_solve_hash_set(input: &[i32]) -> i32 {
    let mut lookup = HashSet::new();
    let mut freq = 0;

    for num in input.iter().cycle() {
        freq += num;
        match lookup.insert(freq) {
                true => continue,
                false => break,
        };
    }

    freq
}