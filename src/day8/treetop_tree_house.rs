use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp;

// O(n^2)
pub fn part1() {
    let file = File::open("./src/day8/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<u32>> = vec![];
    reader.lines()
        .map(|line| line.unwrap().chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .for_each(|v| map.push(v));
    
    let m = map.len();
    let n = map[0].len();
    let mut max_from_left: Vec<Vec<u32>> = vec![];
    for i in 0..m {
        let mut line: Vec<u32> = vec![0; n];
        for j in 1..n {
            line[j] = cmp::max(line[j - 1], map[i][j - 1]);
        }
        max_from_left.push(line);
    }

    let mut max_from_right: Vec<Vec<u32>> = vec![];
    for i in 0..m {
        let mut line: Vec<u32> = vec![0; n];
        for j in (0..n-1).rev() {
            line[j] = cmp::max(line[j + 1], map[i][j + 1]);
        }
        max_from_right.push(line);
    }

    let mut max_from_top: Vec<Vec<u32>> = vec![vec![0; n]; m];
    for i in 1..m {
        for j in 0..n {
            max_from_top[i][j] = cmp::max(max_from_top[i - 1][j], map[i - 1][j]);
        }
    }

    let mut max_from_bottom: Vec<Vec<u32>> = vec![vec![0; n]; m];
    for i in (0..m-1).rev() {
        for j in 0..n {
            max_from_bottom[i][j] = cmp::max(max_from_bottom[i + 1][j], map[i + 1][j]);
        }
    }


    let mut res = 2 * m + 2 * n - 4;
    for i in 1..m-1 {
        for j in 1..n-1 {
            if map[i][j] > max_from_left[i][j] || map[i][j] > max_from_right[i][j] || map[i][j] > max_from_bottom[i][j] || map[i][j] > max_from_top[i][j] {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

// O(n^3)
pub fn part2() {
    let file = File::open("./src/day8/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<u32>> = vec![];
    reader.lines()
        .map(|line| line.unwrap().chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .for_each(|v| map.push(v));
    
    let m = map.len();
    let n = map[0].len();

    let mut highest = 0;
    for i in 1..m-1 {
        for j in 1..n-1 {
            let mut score = 1;
            // check left
            let mut tmp = 0;
            for k in (0..j).rev() {
                tmp += 1;
                if map[i][j] <= map[i][k]{
                    break;
                }
            }
            score *= tmp;
            // check right
            tmp = 0;
            for k in j+1..n {
                tmp += 1;
                if map[i][j] <= map[i][k]{
                    break;
                }
            }
            score *= tmp;
            // check up
            tmp = 0;
            for k in (0..i).rev() {
                tmp += 1;
                if map[i][j] <= map[k][j]{
                    break;
                }
            }
            score *= tmp;
            // check down
            tmp = 0;
            for k in i+1..m {
                tmp += 1;
                if map[i][j] <= map[k][j]{
                    break;
                }
            }
            score *= tmp;
            // print!("{} ", score);
            highest = cmp::max(highest, score);
        }
        // println!();
    }

    println!("{}", highest);
}

pub fn solution() {
    part2()
}