use std::collections::{HashMap, BTreeMap};
use std::fmt::Write;

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

// github.com/Erk- s solution 
#[aoc(day2, part2, Erk)]
fn part2(input: &str) -> String {
    let mut firstmap: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut secondmap: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut input_iter = input.split_whitespace();
    let fst = input_iter.next().unwrap();
    let splt = fst.len()/2;
    {
        let (a, b) = fst.split_at(splt);
        firstmap.insert(a, vec![fst]);
        secondmap.insert(b, vec![fst]);
    }
    for e in input_iter {
        let (a, b) = e.split_at(splt);
        firstmap.entry(a).and_modify(|v| v.push(e)).or_insert_with(|| vec![e]);
        secondmap.entry(b).and_modify(|v| v.push(e)).or_insert_with(|| vec![e]);
    }
    // https://users.rust-lang.org/t/composing-a-list-of-all-pairs/15475/3
    for e in firstmap.values() {
        if e.len() > 1 {
            //println!("{:?}", e);
            let pairs: Vec<(&str, &str)> = e.iter()
                .enumerate()
                .flat_map(move |t| std::iter::repeat(t.1).zip(e.iter().skip(t.0 + 1)))
                .map(|(a,b)| (*a, *b))
                .collect();
            for p in pairs {
                if is_one_off(p) {
                    return print_diff(p);
                }
            }
        }
    }
    for e in secondmap.values() {
        if e.len() > 1 {
            //println!("{:?}", e);
            let pairs: Vec<(&str, &str)> = e.iter()
                .enumerate()
                .flat_map(move |t| std::iter::repeat(t.1).zip(e.iter().skip(t.0 + 1)))
                .map(|(a,b)| (*a, *b))
                .collect();
            for p in pairs {
                if is_one_off(p) {
                    return print_diff(p);
                }
            }
        }
    }
    panic!("NONE FOUND!");
}

fn is_one_off(input: (&str, &str)) -> bool {
    let iter = input.0.chars().zip(input.1.chars());
    let mut one_diff = false;
    for e in iter {
        if e.0 != e.1 {
            if one_diff {
                return false;
            } else {
                one_diff = true;
            }
        }
    }
    true
}

fn print_diff(input: (&str, &str)) -> String {
    let mut ret_str = String::new();
    let iter = input.0.chars().zip(input.1.chars());
    for e in iter {
        if e.0 == e.1 {
            let _ = write!(ret_str, "{}",  e.0);
        }
    }
    ret_str
}