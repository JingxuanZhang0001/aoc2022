use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_limit(input: &str) -> (u32, u32) {
    let limits: Vec<u32> = input.split('-').map(|s| {s.parse().unwrap()}).collect();
    return (limits.get(0).unwrap().clone(), limits.get(1).unwrap().clone());
}

#[allow(dead_code)]
fn part1() {
    let file = File::open("./src/inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);

    let mut n = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let assignments: Vec<&str> = line_str.split(',').collect();
        let (start_1, end_1) = get_limit(assignments.get(0).unwrap());
        let (start_2, end_2) = get_limit(assignments.get(1).unwrap());
        if start_1 <= start_2 && end_1 >= end_2 || start_2 <= start_1 && end_2 >= end_1 {
            n += 1;
        }
    }

    println!("{}", n);
}

fn part2() {
    let file = File::open("./src/inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);

    let mut n = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let assignments: Vec<&str> = line_str.split(',').collect();
        let (start_1, end_1) = get_limit(assignments.get(0).unwrap());
        let (start_2, end_2) = get_limit(assignments.get(1).unwrap());
        if start_1 <= start_2 && end_1 >= end_2
            || start_2 <= start_1 && end_2 >= end_1
            || start_1 <= start_2 && start_2 <= end_1 && end_1 <= end_2 // start_1...start_2...end_1...end_2
            || start_2 <= start_1 && start_1 <= end_2 && end_2 <= end_1 { 
            n += 1;
        }
    }

    println!("{}", n);
}

pub fn solution() {
    part2();
}