use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

pub fn build_fs() -> HashMap<String, i32> {
    let file = File::open("./src/day7/input.txt").unwrap();
    let reader = BufReader::new(file);

    let root = "".to_string();
    let mut fs = HashMap::new();
    fs.insert(root.clone(), 0);

    let mut current = vec![];

    for line in reader.lines() {
        let input = line.unwrap();
        let mut tokens = input.split(' ');
        match tokens.next().unwrap() {
            "$" => {
                match tokens.next().unwrap() {
                    "cd" => {
                        match tokens.next().unwrap() {
                            "/" => {
                                current.clear();
                            },
                            ".." => {
                                current.pop();
                            },
                            dir => {
                                current.push(dir.to_owned());
                            },
                        }
                    },
                    "ls" => {

                    },
                    _ => unreachable!()
                }
            },
            "dir" => {

            },
            size_str => {
                let size = size_str.parse::<i32>().unwrap();
                fs.insert(root.clone(), fs.get(&(root.clone())).unwrap_or(&0) + size);
                let mut file = "".to_owned();
                for f in current.iter() {
                    file.push_str(&f);
                    let dir = file.to_owned();
                    fs.insert(dir.clone(), fs.get(&(dir.clone())).unwrap_or(&0) + size);
                }
            },
        }
    }

    return fs;
}

pub fn part1(fs: &HashMap<String, i32>) {
    let mut total_size = 0;
    for v in fs.values() {
        if *v <= 100000 {
            total_size += v;
        }
    }

    println!("{}", total_size);
}

pub fn part2(fs: &HashMap<String, i32>) {
    let target = 30000000 - (70000000 - fs[""]);
    let mut result = 30000000;
    for v in fs.values() {
        if *v >= target && *v < result {
            result = *v;
        }
    }
    println!("{}", result);
}


pub fn solution() {
    let fs = build_fs();

    // part1(&fs);

    part2(&fs);   
}