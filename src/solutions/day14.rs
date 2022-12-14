use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp;

#[allow(dead_code)]
fn part1() {
    let file = File::open("./src/inputs/day14.txt").unwrap();
    let reader = BufReader::new(file);

    let mut objs: HashSet<(i32, i32)> = HashSet::new();
    let mut bound_left = 500;
    let mut bound_right = 500;
    let mut bound_up = 0;
    let mut bound_down = 0;

    for line in reader.lines() {
        let input = line.unwrap();

        let mut prev_x = -1;
        let mut prev_y = -1;
        let mut next_x = -1;
        let mut next_y = -1;
        input.split(" -> ").for_each(|raw_coord| {
            prev_x = next_x;
            prev_y = next_y;
            let coords: Vec<i32> = raw_coord.split(",").map(|coord| coord.parse::<i32>().unwrap()).collect();
            next_x = coords[0];
            next_y = coords[1];

            bound_left = cmp::min(bound_left, next_x);
            bound_right = cmp::max(bound_right, next_x);
            bound_up = cmp::max(bound_up, next_y);
            bound_down = cmp::min(bound_down, next_y);

            if prev_x != next_x && prev_y == next_y {
                for x in cmp::min(prev_x, next_x)..=cmp::max(prev_x, next_x) {
                    objs.insert((x, prev_y));
                }
            } else if prev_x == next_x && prev_y != next_y {
                for y in cmp::min(prev_y, next_y)..=cmp::max(prev_y, next_y) {
                    objs.insert((prev_x, y));
                }
            }
        });
    }
    for y in bound_down..=bound_up {
        for x in bound_left..=bound_right {
            if x == 500 && y == 0 {
                print!("+");
            } else if objs.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let mut sand: HashSet<(i32, i32)> = HashSet::new();

    let mut count = 0;

    'outer: loop {
        let mut pos_x = 500;
        let mut pos_y = 0;
        for y in 1..=bound_up {
            if !objs.contains(&(pos_x, y)) {
                pos_y += 1;
                continue;
            }
            if !objs.contains(&(pos_x - 1, y)) {
                if pos_x - 1 < bound_left {
                    break 'outer;
                }
                pos_x -= 1;
                pos_y += 1;
                continue;
            }
            if !objs.contains(&(pos_x + 1, y)) {
                if pos_x + 1 > bound_right {
                    break 'outer;
                }
                pos_x += 1;
                pos_y += 1;
                continue;
            }
            objs.insert((pos_x, pos_y));
            sand.insert((pos_x, pos_y));
            break;
        }
        if pos_y == bound_up {
            break;
        }
        count += 1;
    }

    println!("{}", count);
}

fn part2() {
    let file = File::open("./src/inputs/day14.txt").unwrap();
    let reader = BufReader::new(file);

    let mut objs: HashSet<(i32, i32)> = HashSet::new();
    let mut bound_left = 500;
    let mut bound_right = 500;
    let mut bound_up = 0;
    let mut bound_down = 0;

    for line in reader.lines() {
        let input = line.unwrap();

        let mut prev_x = -1;
        let mut prev_y = -1;
        let mut next_x = -1;
        let mut next_y = -1;
        input.split(" -> ").for_each(|raw_coord| {
            prev_x = next_x;
            prev_y = next_y;
            let coords: Vec<i32> = raw_coord.split(",").map(|coord| coord.parse::<i32>().unwrap()).collect();
            next_x = coords[0];
            next_y = coords[1];

            bound_left = cmp::min(bound_left, next_x);
            bound_right = cmp::max(bound_right, next_x);
            bound_up = cmp::max(bound_up, next_y);
            bound_down = cmp::min(bound_down, next_y);

            if prev_x != next_x && prev_y == next_y {
                for x in cmp::min(prev_x, next_x)..=cmp::max(prev_x, next_x) {
                    objs.insert((x, prev_y));
                }
            } else if prev_x == next_x && prev_y != next_y {
                for y in cmp::min(prev_y, next_y)..=cmp::max(prev_y, next_y) {
                    objs.insert((prev_x, y));
                }
            }
        });
    }

    bound_up += 2;
    bound_left -= bound_up;
    bound_right += bound_up;
    for x in bound_left..=bound_right {
        objs.insert((x, bound_up));
    }
    // for y in bound_down..=bound_up {
    //     for x in bound_left..=bound_right {
    //         if x == 500 && y == 0 {
    //             print!("+");
    //         } else if objs.contains(&(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let mut sand: HashSet<(i32, i32)> = HashSet::new();

    'outer: loop {
        let mut pos_x = 500;
        let mut pos_y = 0;
        for y in 0..=bound_up {
            if objs.contains(&(500, 0)) {
                break 'outer;
            }
            if !objs.contains(&(pos_x, y)) {
                pos_y += 1;
                continue;
            }
            if !objs.contains(&(pos_x - 1, y)) {
                if pos_x - 1 < bound_left {
                    unreachable!();
                }
                pos_x -= 1;
                pos_y += 1;
                continue;
            }
            if !objs.contains(&(pos_x + 1, y)) {
                if pos_x + 1 > bound_right {
                    unreachable!();
                }
                pos_x += 1;
                pos_y += 1;
                continue;
            }
            if y == 0 {
                break 'outer;
            }
            objs.insert((pos_x, pos_y - 1));
            sand.insert((pos_x, pos_y - 1));
            break;
        }
    }

    println!("{}", sand.len());

    // for y in bound_down..=bound_up {
    //     for x in bound_left..=bound_right {
    //         if sand.contains(&(x, y)) {
    //             print!("o");
    //         } else if x == 500 && y == 0 {
    //             print!("+");
    //         } else if objs.contains(&(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
}

pub fn solution() {
    part2();
}