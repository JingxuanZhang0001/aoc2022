use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp;

#[allow(dead_code)]
fn part2() {
    let file = File::open("./src/inputs/day17.txt").unwrap();
    let reader = BufReader::new(file);

    let patterns: Vec<Vec<(usize, usize)>> = vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)], // -, left most is the anchor
        vec![(1, 0), (1, 1), (1, 2), (0, 1), (2, 1)], // +, left bottom most is the anchor
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)], // _|, left bottom most is the anchor
        vec![(0, 0), (1, 0), (2, 0), (3, 0)], // |, bottom most is the anchor
        vec![(0, 0), (0, 1), (1, 0), (1, 1)] // #, left bottom most is the anchor
    ];

    for line in reader.lines() {
        let winds = line.unwrap().chars().collect::<Vec<char>>();
        let mut wind_idx: usize = 0;
        let mut map: Vec<Vec<char>> = vec![vec!['.'; 7]; 3];
        let mut current_top: usize = 0;

        // let mut compressed_map: Vec<u8> = vec![0; 3];

        let mut memo: HashMap<(usize, usize, (usize, usize, usize, usize, usize, usize, usize)), usize> = HashMap::new();

        for i in 0..10000 {
            let pattern_idx: usize = i % patterns.len();
            
            let spawn_point = (current_top + 3, 2 as usize);
            let mut block = patterns[pattern_idx].clone().iter().map(|(x, y)| (x + spawn_point.0, y + spawn_point.1)).collect::<Vec<(usize, usize)>>();
            
            while map.len() < current_top + 7 {
                map.push(vec!['.'; 7]);
                // compressed_map.push(0);
            }

            let block_len = block.len();
            loop {
                // println!("{:?}", block);
                // attempt to move
                // check collision
                // commit move
                let wind = winds[wind_idx];
                // println!("wind {}", wind);
                let mut collided: bool = false;
                for (row, col) in &block {
                    match wind {
                        '<' => {
                            if *col == 0 || map[*row][*col - 1] != '.' {
                                collided = true;
                                break;
                            }
                        },
                        '>' => {
                            if *col == 6 || map[*row][*col + 1] != '.' {
                                collided = true;
                                break;
                            }
                        },
                        _ => unreachable!()
                    }
                }
                if !collided {
                    for i in 0..block_len {
                        match wind {
                            '<' => {
                                block[i].1 -= 1;
                            },
                            '>' => {
                                block[i].1 += 1;
                            },
                            _ => unreachable!()
                        }
                    }
                }
                wind_idx = (wind_idx + 1) % winds.len();

                // attempt to drop
                // check collision
                // commit drop
                // else exit
                let mut settled: bool = false;
                for (row, col) in &block {
                    if *row == 0 || map[*row - 1][*col] != '.' {
                        settled = true;
                        // println!("settled");
                        break;
                    }
                }
                for i in 0..block_len {
                    let (row, col) = block[i];
                    if settled {
                        map[row][col] = '#';
                        current_top = cmp::max(current_top, row + 1);

                        // let mut number: u8 = 0;
                        // for ch in &map[row] {
                        //     number = number << 1;
                        //     if *ch == '#' {
                        //         number += 1;
                        //     }
                        // }
                        // compressed_map[row] = number;
                    } else {
                        block[i].0 -= 1;
                    }
                }
                // println!("current top {}", current_top);
                if settled {
                    // for row in (0..map.len()).rev() {
                    //     for col in &map[row] {
                    //         print!("{}", col);
                    //     }
                    //     println!();
                    // }
                    // (0..7).map(|col| {
                    //     for i in (0..current_top).rev() {
                    //         if map[col][i] != '.' {
                    //             return current_top - i;
                    //         }
                    //     }
                    //     return current_top;
                    // }).collect()
                    for col in 0..7 {
                        
                    }
                    
                    break;
                }
            }
        }

        println!("{}", current_top);
        // println!("{:?}", compressed_map);



        // for row in (0..map.len()).rev() {
        //     for col in &map[row] {
        //         print!("{}", col);
        //     }
        //     println!();
        // }
    }
}

fn part1() {
    let file = File::open("./src/inputs/day17.txt").unwrap();
    let reader = BufReader::new(file);

    let patterns: Vec<Vec<(usize, usize)>> = vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)], // -, left most is the anchor
        vec![(1, 0), (1, 1), (1, 2), (0, 1), (2, 1)], // +, left bottom most is the anchor
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)], // _|, left bottom most is the anchor
        vec![(0, 0), (1, 0), (2, 0), (3, 0)], // |, bottom most is the anchor
        vec![(0, 0), (0, 1), (1, 0), (1, 1)] // #, left bottom most is the anchor
    ];

    for line in reader.lines() {
        let winds = line.unwrap().chars().collect::<Vec<char>>();
        let mut wind_idx: usize = 0;
        let mut map: Vec<Vec<char>> = vec![vec!['.'; 7]; 3];
        let mut current_top: usize = 0;

        for i in 0..2022 {
            let pattern_idx: usize = i % patterns.len();
            
            let spawn_point = (current_top + 3, 2 as usize);
            let mut block = patterns[pattern_idx].clone().iter().map(|(x, y)| (x + spawn_point.0, y + spawn_point.1)).collect::<Vec<(usize, usize)>>();
            
            while map.len() < current_top + 7 {
                map.push(vec!['.'; 7]);
            }

            let block_len = block.len();
            loop {
                // println!("{:?}", block);
                // attempt to move
                // check collision
                // commit move
                let wind = winds[wind_idx];
                // println!("wind {}", wind);
                let mut collided: bool = false;
                for (row, col) in &block {
                    match wind {
                        '<' => {
                            if *col == 0 || map[*row][*col - 1] != '.' {
                                collided = true;
                                break;
                            }
                        },
                        '>' => {
                            if *col == 6 || map[*row][*col + 1] != '.' {
                                collided = true;
                                break;
                            }
                        },
                        _ => unreachable!()
                    }
                }
                if !collided {
                    for i in 0..block_len {
                        match wind {
                            '<' => {
                                block[i].1 -= 1;
                            },
                            '>' => {
                                block[i].1 += 1;
                            },
                            _ => unreachable!()
                        }
                    }
                }
                wind_idx = (wind_idx + 1) % winds.len();

                // attempt to drop
                // check collision
                // commit drop
                // else exit
                let mut settled: bool = false;
                for (row, col) in &block {
                    if *row == 0 || map[*row - 1][*col] != '.' {
                        settled = true;
                        // println!("settled");
                        break;
                    }
                }
                for i in 0..block_len {
                    let (row, col) = block[i];
                    if settled {
                        map[row][col] = '#';
                        current_top = cmp::max(current_top, row + 1);
                    } else {
                        block[i].0 -= 1;
                    }
                }
                // println!("current top {}", current_top);
                if settled {
                    // for row in (0..map.len()).rev() {
                    //     for col in &map[row] {
                    //         print!("{}", col);
                    //     }
                    //     println!();
                    // }
                    break;
                }
            }
        }

        println!("{}", current_top);

        // for row in (0..map.len()).rev() {
        //     for col in &map[row] {
        //         print!("{}", col);
        //     }
        //     println!();
        // }
    }
}

pub fn solution() {
    part1();
}