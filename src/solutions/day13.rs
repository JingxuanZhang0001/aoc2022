use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::{self, Ordering};


#[derive(Debug, PartialEq, Eq)]
enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>)
}

fn build_nested_integer(input: String) -> NestedInteger {
    // println!("Parsing {}", input);
    let mut num = 0;
    let mut handle_number = false;
    let mut result_stack: Vec<Vec<NestedInteger>> = vec![vec![]];
    for ch in input.chars() {
        match ch {
            '[' => {
                // println!("Handling: [");
                // add a layer to builer stack
                result_stack.push(vec![]);
            },
            ']' => {
                // println!("Handling: ]");
                // if is finishing a number, build a new NestedInteger::Int and push to the last of result_stack
                // if is finishing a list, pop the last of result_stack, build a new NestedInteger::List, push to the last of result_stack
                if handle_number {
                    let len = result_stack.len();
                    let last = &mut result_stack[len - 1];
                    last.push(NestedInteger::Int(num));
                    handle_number = false;
                }
                    let last = result_stack.pop().unwrap();
                    let ni = NestedInteger::List(last);
                    let len = result_stack.len();
                    let last_2 = &mut result_stack[len - 1];
                    last_2.push(ni);
                    // println!("result_stack: {:?}, len: {}", result_stack, result_stack.len());
                num = 0;
            },
            ',' => {
                // println!("Handling: ,");
                // comma only handles number
                if handle_number {
                    let len = result_stack.len();
                    let last = &mut result_stack[len - 1];
                    last.push(NestedInteger::Int(num));
                    handle_number = false;
                }
                num = 0;
            },
            digit => {
                // println!("Handling: {}", digit.to_digit(10).unwrap() as i32);
                handle_number = true;
                num = num * 10 + digit.to_digit(10).unwrap() as i32;
            }
        }
        // println!("result_stack {:?}", result_stack);
    }

    let res = result_stack.pop().unwrap().pop().unwrap();
    // println!("Got {:?}", res);
    return res;
}

fn compare(left: &NestedInteger, right: &NestedInteger) -> Ordering {
    match (left, right) {
        (NestedInteger::Int(int_left), NestedInteger::Int(int_right)) => {
            // println!("Comparing {} {}", int_left, int_right);
            if int_left < int_right {
                return Ordering::Less;
            } else if int_left == int_right {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        },
        (NestedInteger::Int(int_left), NestedInteger::List(_)) => {
            return compare(&NestedInteger::List(vec![NestedInteger::Int(*int_left)]), right);
        },
        (NestedInteger::List(_), NestedInteger::Int(int_right)) => {
            return compare(left, &NestedInteger::List(vec![NestedInteger::Int(*int_right)]));
        },
        (NestedInteger::List(list_left), NestedInteger::List(list_right)) => {
            // println!("Comparing list {:?} {:?}", list_left, list_right);
            for i in 0..cmp::min(list_left.len(), list_right.len()) {
                let res = compare(&list_left[i], &list_right[i]);
                if res != Ordering::Equal {
                    return res;
                }
            }
            if list_left.len() < list_right.len() {
                return Ordering::Less;
            }
            if list_left.len() == list_right.len() {
                return Ordering::Equal;
            }
            return Ordering::Greater;
        }
    }
}

#[allow(dead_code)]
fn part1() {
    let file = File::open("./src/inputs/day13.txt").unwrap();
    let reader = BufReader::new(file);

    let mut left: Option<NestedInteger> = None;
    let mut right: Option<NestedInteger> = None;

    let mut index = 0;
    let mut result = 0;

    for line in reader.lines() {
        let input = line.unwrap();
        if input.len() == 0 {
            index += 1;
            // println!("Compare {:?} {:?}", left, right);
            if compare(left.as_ref().unwrap(), right.as_ref().unwrap()) == Ordering::Less {
                println!("{}", index);
                result += index;
            }
            left = None;
            right = None;
        } else if left.is_none() {
            left = Some(build_nested_integer(input));
        } else if right.is_none() {
            right = Some(build_nested_integer(input));
        }
    }
    index += 1;
    // println!("Compare {:?} {:?}", left, right);
    if compare(left.as_ref().unwrap(), right.as_ref().unwrap()) == Ordering::Less {
        println!("{}", index);
        result += index;
    }

    println!("{}", result);
}

fn part2() {
    let file = File::open("./src/inputs/day13.txt").unwrap();
    let reader = BufReader::new(file);

    let mut packets: Vec<NestedInteger> = vec![];
    
    for line in reader.lines() {
        let input = line.unwrap();
        if input.len() == 0 {
            continue;
        }
        packets.push(build_nested_integer(input));
    }

    let divider_1 = NestedInteger::List(vec![NestedInteger::List(vec![NestedInteger::Int(2)])]);
    let divider_2 = NestedInteger::List(vec![NestedInteger::List(vec![NestedInteger::Int(6)])]);

    packets.push(divider_1);
    packets.push(divider_2);

    packets.sort_by(compare);

    let mut res = 1;
    for (i, packet) in packets.iter().enumerate() {
        // println!("{} {:?}", i + 1, packet);
        if packet == &NestedInteger::List(vec![NestedInteger::List(vec![NestedInteger::Int(2)])])
            || packet == &NestedInteger::List(vec![NestedInteger::List(vec![NestedInteger::Int(6)])]) {
                res *= i + 1;
            }
    }

    println!("{}", res);
}

pub fn solution() {
    part2();
}