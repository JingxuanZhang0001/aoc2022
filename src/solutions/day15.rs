use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::cmp;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Data {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32
}

#[allow(dead_code)]
fn part1() {
    let file = File::open("./src/inputs/day15.txt").unwrap();
    let reader = BufReader::new(file);

    let re: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let mut left_bound = std::i32::MAX;
    let mut right_bound = std::i32::MIN;

    let mut data: Vec<Data> = vec![];

    for line in reader.lines() {
        let input = line.unwrap();

        let capture = re.captures(&input);

        let (sensor_x, sensor_y, beacon_x, beacon_y) = capture.map(|cap| {
            (
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        }).unwrap();

        left_bound = cmp::min(left_bound, sensor_x - (sensor_x - beacon_x).abs() - (sensor_y - beacon_y).abs());
        right_bound = cmp::max(right_bound, sensor_x + (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs());

        data.push(Data { sensor_x, sensor_y, beacon_x, beacon_y});

    }

    println!("bound {} {}", left_bound, right_bound);

    let y = 2000000;

    let mut n_empty = 0;
    for x in left_bound..=right_bound {
        let mut maybe_beacon: bool = true;
        for d in data.iter() {
            if x == d.beacon_x && y == d.beacon_y {
                maybe_beacon = true;
                break;
            }
            if (x - d.sensor_x).abs() + (y - d.sensor_y).abs() <= (d.sensor_x - d.beacon_x).abs() + (d.sensor_y - d.beacon_y).abs() {
                maybe_beacon = false;
                break;
            }
        }
        if !maybe_beacon {
            n_empty += 1;
        }
    }

    println!("{}", n_empty);
}

// brute force parallel processing using part 1 dumb solution
// no, it's too slow
#[allow(dead_code)]
fn part2_2() {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    let file = File::open("./src/inputs/day15.txt").unwrap();
    let reader = BufReader::new(file);

    let re: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let mut data: Vec<Data> = vec![];

    for line in reader.lines() {
        let input = line.unwrap();

        let capture = re.captures(&input);

        let (sensor_x, sensor_y, beacon_x, beacon_y) = capture.map(|cap| {
            (
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        }).unwrap();

        data.push(Data { sensor_x, sensor_y, beacon_x, beacon_y});
    }

    (0..=4000000i32).into_iter().for_each(|y| {
        println!("{}", y);
        (0..=4000000i32).into_par_iter().for_each(|x| {
            let mut maybe_beacon: bool = true;
            for d in data.iter() {
                if (x - d.sensor_x).abs() + (y - d.sensor_y).abs() <= (d.sensor_x - d.beacon_x).abs() + (d.sensor_y - d.beacon_y).abs() {
                    maybe_beacon = false;
                    break;
                }
            }
            if maybe_beacon {
                println!("{}", x as i128 * 4000000i128 + y as i128);
                return;
            }
        })
    });
}

fn part2() {
    let file = File::open("./src/inputs/day15.txt").unwrap();
    let reader = BufReader::new(file);

    let re: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let mut data: Vec<Data> = vec![];

    for line in reader.lines() {
        let input = line.unwrap();

        let capture = re.captures(&input);

        let (sensor_x, sensor_y, beacon_x, beacon_y) = capture.map(|cap| {
            (
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        }).unwrap();

        data.push(Data { sensor_x, sensor_y, beacon_x, beacon_y});
    }

    let limit = 4000000;
    (0..=limit).into_iter().for_each(|y| {
        // let mut maybe_beacon: bool = true;
        let mut ranges: Vec<(i32, i32)> = data.iter().filter_map(|d| {
            let range = (d.sensor_x - d.beacon_x).abs() + (d.sensor_y - d.beacon_y).abs() - (y - d.sensor_y).abs();
            if range > 0 {
                return Some((d.sensor_x - range, d.sensor_x + range));
            } else {
                return None;
            }
        }).collect();
        ranges.sort_by_key(|pair| pair.0);
        let mut right = 0;
        for range in ranges {
            if range.0 > right {
                println!("{}", (right as i128 + 1i128) * 4000000i128 + y as i128);
                return;
            }
            right = cmp::max(right, range.1);
        }
        if right < limit {
            println!("{}", limit * 4000000 + y);
            return;
        }
    });
}


pub fn solution() {
    part2();
}