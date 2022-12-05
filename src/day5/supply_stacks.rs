use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

pub fn solution() {
    let file = File::open("./src/day5/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cargo: Vec<Vec<char>> = vec![];

    let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in reader.lines() {
        let instruction = line.unwrap();
        if instruction == "" {
            for v in &mut cargo {
                v.reverse();
                // print!("{}", v.get(v.len() - 1).unwrap_or(&' '));
            }
            // println!();
            continue;
        }
        if instruction.chars().next().unwrap() == '[' {
            let n = (instruction.len() + 1) / 4;
            while cargo.len() < n {
                cargo.push(vec![]);
            }
            for (idx, ch) in instruction.chars().enumerate() {
                if idx % 4 == 1 && ch.is_alphabetic() {
                    cargo[idx / 4].push(ch);
                }
            }
            continue;
        }

        let capture = re.captures(&instruction);

        if capture.is_none() {
            continue;
        }
        
        let (num, from, to) = capture.map(|cap| {
            (cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<usize>().unwrap())
        }).unwrap();
        // part 1
        // for _ in 0..num {
        //     let top = cargo[from - 1].pop().unwrap();
        //     cargo[to - 1].push(top);
        // }
        
        // part 2
        let cut_start = cargo[from - 1].len() - num;
        let moving: Vec<char> = cargo[from - 1].drain(cut_start..).collect();
        cargo[to - 1].extend_from_slice(&moving);
    }
    for v in cargo {
        print!("{}", v.get(v.len() - 1).unwrap_or(&' '));
    }
    println!();
}
