#![feature(array_windows)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>()
        .array_windows()
        .filter(|[a, b]| a < b)
        .count();

    println!("{}", res);

    Ok(())
}
