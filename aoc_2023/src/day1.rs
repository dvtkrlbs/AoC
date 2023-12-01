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
    input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| {
            let mut digits = line.into_iter().filter(|c| (*c - b'0') < 10);
            let first = (digits.next().unwrap_or(&0) - b'0') as u32;
            let last = digits.last().map(|d| (d - b'0') as u32);

            return first * 10 + last.unwrap_or(first);
        })
        .sum()
}

fn needle_to_digit(needle: &'static [u8]) -> u64 {
    match needle {
        // "zero" => 0,
        b"one" => 1,
        b"two" => 2,
        b"three" => 3,
        b"four" => 4,
        b"five" => 5,
        b"six" => 6,
        b"seven" => 7,
        b"eight" => 8,
        b"nine" => 9,
        _ => unreachable!(),
    }
}
const NEEDLES: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aoc(day1, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut lastIdx = 0;
            let mut buffer: &[u8] = &[];
            let mut first = None;
            let mut last = None;
            for (idx, next_char) in l.bytes().enumerate() {
                buffer = &l.as_bytes()[..idx + 1];
                if next_char.is_ascii_digit() {
                    let digit = (next_char - b'0') as u64;
                    if first.is_none() {
                        first = Some(digit);
                    }
                    last = Some(digit);
                    lastIdx = idx;
                    continue;
                }
                
                let start_index = (idx - 5).min(0).max(lastIdx);       
                for needle in NEEDLES {
                    if (&buffer[start_index..idx + 1]).ends_with(needle.as_bytes()) {
                        let digit = needle_to_digit(needle.as_bytes());
                        if first.is_none() {
                            first = Some(digit);
                        }
                        last = Some(digit);
                        lastIdx = idx;
                        break;
                    }
                }
            }
            let first = first.unwrap_or(0);

            first * 10 + last.unwrap_or(first)
        })
        .sum()
}
