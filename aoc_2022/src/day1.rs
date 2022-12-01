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

#[aoc(day1, part1, Horrible)]
pub fn day1_part1_horrible(input: &[u8]) -> i64 {
    let res = input.iter().fold((0, 0, 0), |mut acc, x| match x {
        b'\n' => {
            if acc.2 == 0 {
                acc.0 = acc.0.max(acc.1);
                acc.1 = 0;
                acc.2 = 0;
            } else {
                acc.1 += acc.2;
                acc.2 = 0;
            }
            acc
        }
        &x => {
            acc.2 = acc.2 * 10 + x.wrapping_sub(b'0') as i64;
            acc
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

#[aoc(day1, part2, Horrible)]
pub fn day1_part2_horrible(input: &[u8]) -> i64 {
    let res = input.iter().fold((0, 0, 0, 0, 0), |mut acc, x| match x {
        b'\n' => {
            if acc.4 == 0 {
                let x = acc.3;
                if x > acc.0 {
                    acc.2 = acc.1;
                    acc.1 = acc.0;
                    acc.0 = x;
                } else if x > acc.1 {
                    acc.2 = acc.1;
                    acc.1 = x;
                } else if x > acc.2 {
                    acc.2 = x;
                }
                acc.3 = 0;
                acc.4 = 0;
            } else {
                acc.3 += acc.4;
                acc.4 = 0;
            }
            acc
        }
        x => {
            acc.4 = acc.4 * 10 + x.wrapping_sub(b'0') as i64;
            acc
        }
    });

    res.0 + res.1 + res.2
}
