use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip();

    left.sort();
    right.sort();

    let total = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (left_v, right_v)| acc + (left_v - right_v).abs());

    total
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let mut left = HashMap::new();
    let mut right = HashMap::new();
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .for_each(|(l_v, r_v)| {
            left.entry(l_v).and_modify(|v| *v += 1).or_insert(1);
            right.entry(r_v).and_modify(|v| *v += 1).or_insert(1);
        });

    let similarity = left
        .keys()
        .map(|key| (key, right.get(key).unwrap_or(&0)))
        .fold(0, |acc, (key, v)| (key * v) + acc);

    similarity
}
