use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn solution() {
    let file = File::open("./src/inputs/day6.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let input = line.unwrap();
        
        // part 1
        // let target = 4;

        // part 2
        let target = 14;
        for i in 0..input.len() {
            if i < target {
                continue;
            }
            // let set: HashSet<char> = HashSet::from_iter(input.as_str()[i - target..i].chars().clone());
            let set: HashSet<char> = HashSet::from_iter((&input)[i - target..i].chars().clone());
            if set.len() == target {
                println!("{}", i);
                return;
            }
        }
    }
}