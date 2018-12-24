use std::collections::HashMap;

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


// The solution of https://github.com/Erk-
#[aoc(day2, part1, Erk)]
pub fn day_part1_erk(input: &str) -> i32 {
    let vec_of_str = input.split('\n').collect::<Vec<&str>>();
    let mut twos = 0;
    let mut threes = 0;
    for id in vec_of_str.iter() {
        let mut is2 = false;
        let mut is3 = false;
        for ch in id.chars() {
            if is3 && is2 { continue; }
            let count = id.matches(ch).count();
            match count {
                2 => is2 = true,
                3 => is3 = true,
                _ => (),
            }
        } 
        if is2 { twos += 1; }
        if is3 { threes += 1; }
    }
    (twos * threes)
}