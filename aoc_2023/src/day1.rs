use aoc_runner_derive::{aoc, aoc_generator};

/* #[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut digits = line.chars().filter(|c| c.is_ascii_digit());
        let first = digits.next().unwrap().to_digit(10).unwrap();
        let last = digits.last().map(|d| d.to_digit(10).unwrap());
        
        return first * 10 + last.unwrap_or(first);
    }).sum()
} */

#[aoc(day1, part1)]
fn part1_sol2(input: &str) -> u32 {
    input.as_bytes().split(|b| *b == b'\n').map(|line| {
        let mut digits = line.into_iter().filter(|c| (*c - b'0') < 10);
        let first = (digits.next().unwrap() - b'0') as u32;
        let last = digits.last().map(|d| (d - b'0') as u32);
     
        return first * 10 + last.unwrap_or(first);
    }).sum()
}