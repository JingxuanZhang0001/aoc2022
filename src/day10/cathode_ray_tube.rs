use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec;

#[allow(dead_code)]
fn part1() {
    let file = File::open("./src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut signal = 1;

    let mut cycle = 0;
    let reporting_cycle: HashSet<i32> = vec!(20, 60, 100, 140, 180, 220).into_iter().collect();

    let mut strength = 0;

    for line in reader.lines() {
        let input = line.unwrap();
        for op in input.split(" ") {
            cycle += 1;
            if reporting_cycle.contains(&cycle) {
                // println!("{} {}", cycle, signal);
                strength += cycle * signal;
            }
            match op {
                "addx" => {
                    
                },
                "noop" => {

                },
                number_str => {
                    signal += number_str.parse::<i32>().unwrap();
                }
            }
            
        }
    }

    println!("{}", strength);
}

fn part2() {
    let file = File::open("./src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut signal = 1;

    let mut cycle = 0;

    for line in reader.lines() {
        let input = line.unwrap();
        for op in input.split(" ") {
            if cycle % 40 + 1 == signal || cycle % 40 == signal || cycle % 40 - 1 == signal {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
            
            match op {
                "addx" => {
                    
                },
                "noop" => {

                },
                number_str => {
                    signal += number_str.parse::<i32>().unwrap();
                }
            }
            
            if cycle % 40 == 0 {
                println!();
            }
        }
    }
}

pub fn solution() {
    part2();
}
