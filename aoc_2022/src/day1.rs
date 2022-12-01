use itertools::Itertools;

fn atoi(bytes: &[u8]) -> i64 {
    let mut result = 0;
    for byte in bytes {
        result = result * 10 + (byte - b'0') as i64;
    }
    result
}

#[aoc(day1, part1)]
pub fn day1_part1(input: &[u8]) -> i64 {
    let res = input.split(|b| *b == b'\n').fold((0, 0), |acc, x| {
        if x == b"" {
            if acc.1 > acc.0 {
                (acc.1, 0)
            } else {
                (acc.0, 0)
            }
        } else {
            let x = atoi(x) + acc.1;
            (acc.0, x)
        }
    });

    res.0
}

#[aoc(day1, part2)]
pub fn day1_part2(input: &[u8]) -> i64 {
    let res = input.split(|b| *b == b'\n').fold((0, 0, 0, 0), |acc, x| {
        if x == b"" {
            let x = atoi(x) + acc.3;
            if x > acc.0 {
                (x, acc.0, acc.1, 0)
            } else if x > acc.1 {
                (acc.0, x, acc.1, 0)
            } else if x > acc.2 {
                (acc.0, acc.1, x, 0)
            } else {
                (acc.0, acc.1, acc.2, 0)
            }
        } else {
            (acc.0, acc.1, acc.2, acc.3 + atoi(x))
        }
    });

    res.0 + res.1 + res.2
}
