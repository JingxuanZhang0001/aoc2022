use std::fs::File;
use std::io::{BufReader, BufRead};

#[allow(dead_code)]
fn get_score_part1(input: &str) -> u128 {
    match input {
        "A X" => return 4u128,
        "A Y" => return 8u128,
        "A Z" => return 3u128,
        "B X" => return 1u128,
        "B Y" => return 5u128,
        "B Z" => return 9u128,
        "C X" => return 7u128,
        "C Y" => return 2u128,
        "C Z" => return 6u128,
        _ => return 0u128,
    }
}

#[allow(dead_code)]
fn part1() {
    let file = File::open("./src/inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut res = 0;
    for line in reader.lines() {
        res += get_score_part1(&line.unwrap());
    }

    println!("{}", res);
}

// x: lose, y: draw, z: win
// a: 1, b: 2, c: 3
// b > a > c > b
// win: 6, draw: 3, lose: 0
fn get_score_part2(input: &str) -> u128 {
    match input {
        "A X" => return 3u128,
        "A Y" => return 4u128,
        "A Z" => return 8u128,
        "B X" => return 1u128,
        "B Y" => return 5u128,
        "B Z" => return 9u128,
        "C X" => return 2u128,
        "C Y" => return 6u128,
        "C Z" => return 7u128,
        _ => return 0u128,
    }
}

fn part2() {
    let file = File::open("./src/inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut res = 0;
    for line in reader.lines() {
        res += get_score_part2(&line.unwrap());
    }

    println!("{}", res);
}

pub fn solution() {
    part2();
}