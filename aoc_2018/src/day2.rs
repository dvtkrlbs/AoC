use std::collections::{HashMap, BTreeMap};

#[aoc(day2, part1, HashMap)]
pub fn day2_part1_solve_hash(input: &str) -> i32 {
    let vec_of_str = input.split('\n').collect::<Vec<&str>>();
    let mut lookup = HashMap::new();
    let mut twos = 0;
    let mut threes = 0;
    for id in vec_of_str.iter() {
        for ch in id.chars() {
            let entry = lookup.entry(ch).or_insert(0);
            *entry += 1;
        }
        match lookup.values().find(|&&x| x == 2) {
            Some(2) => twos += 1,
            _ => (),
        }
        match lookup.values().find(|&&x| x == 3) {
            Some(3) => threes += 1,
            _ => (),
        }
        lookup.clear();
    }
    twos * threes
}

#[aoc(day2, part1, BTreeMap)]
pub fn day2_part1_solve_btree(input: &str) -> i32 {
    let vec_of_str = input.split('\n').collect::<Vec<&str>>();
    let mut lookup = BTreeMap::new();
    let mut twos = 0;
    let mut threes = 0;
    for id in vec_of_str.iter() {
        for ch in id.chars() {
            let entry = lookup.entry(ch).or_insert(0);
            *entry += 1;
        }
        match lookup.values().find(|&&x| x == 2) {
            Some(2) => twos += 1,
            _ => (),
        }
        match lookup.values().find(|&&x| x == 3) {
            Some(3) => threes += 1,
            _ => (),
        }
        lookup.clear();
    }
    twos * threes
}

#[aoc(day2, part2)]
pub fn day2_part2_naive(input: &str) -> String {
    let vec_of_str = input.split('\n').collect::<Vec<&str>>();
    let mut result = String::new();

    for first in &vec_of_str {
        for second in &vec_of_str {
            if first == second {
                continue;
            }

            let mut differs = 0;
            let mut posi: usize = 0;
            for (pos, (c1, c2)) in first.chars().zip(second.chars()).enumerate() {
                if c1 != c2 {
                    differs += 1;
                    posi = pos;
                }
            }

            if differs == 1 {
                result = String::from(first.clone());
                result.remove(posi);
            }
        }
    }
    result
}