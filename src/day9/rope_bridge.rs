use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn follow(head: &Point, tail: &mut Point) {
    if (head.x - tail.x).abs() > 1 && (head.y - tail.y).abs() > 1 {
        if head.x > tail.x {
            tail.x += 1;
        } else if head.x < tail.x {
            tail.x -= 1;
        }
        if head.y > tail.y {
            tail.y += 1;
        } else if head.y < tail.y {
            tail.y -= 1;
        }
    } else if (head.x - tail.x).abs() > 1 {
        if head.x > tail.x {
            tail.x += 1;
        } else if head.x < tail.x {
            tail.x -= 1;
        }
        if (head.y - tail.y).abs() == 1 {
            tail.y = head.y;
        }
    } else if (head.y - tail.y).abs() > 1 {
        if head.y > tail.y {
            tail.y += 1;
        } else if head.y < tail.y {
            tail.y -= 1;
        }
        if (head.x - tail.x).abs() == 1 {
            tail.x = head.x;
        }
    }
}

#[allow(dead_code)]
fn draw(points: &Vec<Point>) {
    let mut board = vec![vec!['.'; 6]; 5];
    for (i, p) in points.iter().enumerate() {
        if i == 0 {
            board[(p.y + 0) as usize][(p.x + 0) as usize] = '#';
            continue;
        }
        if board[p.y as usize][p.x as usize] == '.' {
            board[p.y as usize][p.x as usize] = i.to_string().chars().next().unwrap();
        }
    }
    board.reverse();
    for row in board {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

pub fn solution() {
    let file = File::open("./src/day9/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut visited: HashSet<Point> = HashSet::new();
    
    // part 1
    // let len = 2;

    // part 2
    let len = 10;

    let mut points: Vec<Point> = vec![];
    for _ in 0..len {
        points.push(Point {x: 0, y: 0});
    }
    visited.insert(points[len - 1].clone());
    for line in reader.lines() {
        let input = line.unwrap();
        let ops: Vec<&str> = input.split(" ").collect();

        let dir = ops[0];
        let steps = ops[1].parse::<u32>().unwrap();
        
        for _ in 0..steps {
            match dir {
                "U" => points[0].y += 1,
                "D" => points[0].y -= 1,
                "L" => points[0].x -= 1,
                "R" => points[0].x += 1,
                _ => unreachable!()
            }
            for i in 1..len {
                let (left, right) = points.split_at_mut(i);
                follow(left.last().unwrap(), right.first_mut().unwrap());
            }
            visited.insert(points[len - 1].clone());
            // draw(&points);
            // println!();
        }
    }
    println!("{}", visited.len());
}