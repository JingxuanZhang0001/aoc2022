use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn get_priority(ch: char) -> u32 {
    if ch.is_ascii_lowercase() {
        return ch as u32 - 'a' as u32 + 1;
    } else if ch.is_ascii_uppercase() {
        return ch as u32 - 'A' as u32 + 27;
    }
    unreachable!()
}

fn get_badge_priority(r1: &str, r2: &str, r3: &str) -> u32 {
    let mut common1 = HashSet::new();
    let mut common2 = HashSet::new();
    for ch in r1.chars() {
        common1.insert(ch);
    }
    for ch in r2.chars() {
        if common1.contains(&ch) {
            common2.insert(ch);
        }
    }
    for ch in r3.chars() {
        if common2.contains(&ch) {
            return get_priority(ch);
        }
    }
    unreachable!();
}

fn part2() {
    let file = File::open("./src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut iter = reader.lines();

    let mut score = 0;
    loop {
        let line1 = iter.next();
        if line1.is_none() {
            break;
        }
        score += get_badge_priority(&line1.expect("exists").unwrap(), &iter.next().expect("exists").unwrap(), &iter.next().expect("exists").unwrap());
    }

    println!("{}", score);
}

#[allow(dead_code)]
fn get_common_items_score(input: &str) -> u32 {
    let len = input.len();
    let half_len = len / 2;
    let mut common = HashSet::new();
    let mut score = 0;
    for (i, ch) in input.chars().enumerate() {
        if i < half_len {
            common.insert(ch);
        } else {
            if common.contains(&ch) {
                score += get_priority(ch);
                common.remove(&ch);
            }
        }
    }
    return score;
}

#[allow(dead_code)]
fn part1() {
    let file = File::open("./src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        score += get_common_items_score(&line.unwrap());
    }

    println!("{}", score);
}

pub fn solution() {
    part2();
}