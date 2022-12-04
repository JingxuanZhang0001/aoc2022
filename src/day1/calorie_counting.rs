use std::cmp;
use std::fs;

use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[allow(dead_code)]
fn part1() {
    let contents = fs::read_to_string("./src/day1/input.txt").unwrap();
    let mut current: u128 = 0;
    let mut global_max: u128 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            global_max = cmp::max(current, global_max);
            current = 0;
        } else {
            current += line.trim().parse::<u128>().unwrap();
        }
    }

    println!("{}", global_max);
}

fn part2() {
    let contents = fs::read_to_string("./src/day1/input.txt").unwrap();

    let max_cap = 3;
    let mut heap = BinaryHeap::with_capacity(max_cap + 1);
    let mut current: u128 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            heap.push(Reverse(current));
            if heap.len() > max_cap {
                heap.pop();
            }
            current = 0;
        } else {
            current += line.trim().parse::<u128>().unwrap();
        }
    }


    let mut res: u128 = 0;
    while !heap.is_empty() {
        res += heap.pop().unwrap().0;
    }
    println!("{}", res);
}

pub fn solution() {
    part2();
}