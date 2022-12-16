use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp;
use regex::Regex;

#[allow(dead_code)]
fn part1() {
    let file = File::open("./src/inputs/day16.txt").unwrap();
    let reader = BufReader::new(file);

    // Valve GJ has flow rate=14; tunnels lead to valves UV, AO, MM, UD, GM
    let re: Regex = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z,\s]+)").unwrap();

    let mut neighbor_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut dist_map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut flow_map: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let input = line.unwrap();

        let capture = re.captures(&input);

        capture.map(|cap| {
            let valve = cap.get(1).unwrap().as_str().to_owned();
            let flow = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let neighbors = cap.get(3).unwrap().as_str().split(", ").map(|s| s.to_owned()).collect::<HashSet<String>>();
            neighbor_map.insert(valve.clone(), neighbors);
            flow_map.insert(valve.clone(), flow);
        });
    }

    let valves = flow_map.keys().map(|k| k.clone()).collect::<Vec<String>>();

    for valve_from in valves.clone() {
        for valve_to in valves.clone() {
            let dist = if neighbor_map[&valve_from].contains(&valve_to) { 1 } else { valves.len() as i32 };
            dist_map.entry(valve_from.clone()).or_insert(HashMap::new()).insert(valve_to.clone(), dist);
        }
        dist_map.get_mut(&valve_from).unwrap().insert(valve_from.clone(), 0);
    }
    for mid in valves.clone() {
        for valve_from in valves.clone() {
            for valve_to in valves.clone() {
                let tmp1 = dist_map[&valve_from][&mid];
                let tmp2 = dist_map[&mid][&valve_to];
                if tmp1 + tmp2 < dist_map[&valve_from][&valve_to] {
                    dist_map.get_mut(&valve_from).unwrap().insert(valve_to, tmp1 + tmp2);
                }
            }
        }
    }

    // println!("{:?}", dist_map);

    let mut result = 0;
    let mut stack = vec![(30, "AA".to_owned(), flow_map.clone(), 0)];
    while stack.len() > 0 {
        let (min, current_valve, current_flow_map, current_total) = stack.pop().unwrap();
        for valve in valves.clone() {
            let remain_min = min - dist_map[&current_valve][&valve] - 1;
            let potential_total = remain_min * current_flow_map[&valve];
            if potential_total > 0 {
                let mut flow_map_copy = current_flow_map.clone();
                flow_map_copy.insert(valve.clone(), 0);
                result = cmp::max(result, current_total + potential_total);
                stack.push((remain_min, valve.clone(), flow_map_copy, current_total + potential_total));
            }
        }        
    }

    println!("{}", result);
}

fn part2() {
    let file = File::open("./src/inputs/day16.txt").unwrap();
    let reader = BufReader::new(file);

    // Valve GJ has flow rate=14; tunnels lead to valves UV, AO, MM, UD, GM
    let re: Regex = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z,\s]+)").unwrap();

    let mut neighbor_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut dist_map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut flow_map: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let input = line.unwrap();

        let capture = re.captures(&input);

        capture.map(|cap| {
            let valve = cap.get(1).unwrap().as_str().to_owned();
            let flow = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let neighbors = cap.get(3).unwrap().as_str().split(", ").map(|s| s.to_owned()).collect::<HashSet<String>>();
            neighbor_map.insert(valve.clone(), neighbors);
            flow_map.insert(valve.clone(), flow);
        });
    }

    let valves = flow_map.keys().map(|k| k.clone()).collect::<Vec<String>>();

    for valve_from in valves.clone() {
        for valve_to in valves.clone() {
            let dist = if neighbor_map[&valve_from].contains(&valve_to) { 1 } else { valves.len() as i32 };
            dist_map.entry(valve_from.clone()).or_insert(HashMap::new()).insert(valve_to.clone(), dist);
        }
        dist_map.get_mut(&valve_from).unwrap().insert(valve_from.clone(), 0);
    }
    for mid in valves.clone() {
        for valve_from in valves.clone() {
            for valve_to in valves.clone() {
                let tmp1 = dist_map[&valve_from][&mid];
                let tmp2 = dist_map[&mid][&valve_to];
                if tmp1 + tmp2 < dist_map[&valve_from][&valve_to] {
                    dist_map.get_mut(&valve_from).unwrap().insert(valve_to, tmp1 + tmp2);
                }
            }
        }
    }

    let mut result = 0;
    let mut valves_remain: Vec<bool> = vec![false; valves.len()];
    for (i, valve) in valves.iter().enumerate() {
        if flow_map[valve] > 0 {
            valves_remain[i] = true;
        }
    }

    let mut memo: HashSet<(Vec<bool>, i32)> = HashSet::new();

    let mut stack = vec![(26, "AA".to_owned(), 26, "AA".to_owned(), valves_remain.clone(), 0)];
    while stack.len() > 0 {
        let (min_1, current_valve_1, min_2, current_valve_2, current_valves_remain, current_total) = stack.pop().unwrap();
        
        let maybe_visited = current_valves_remain.clone();
        if memo.contains(&(maybe_visited.clone(), current_total)) {
            continue;
        } else {
            memo.insert((maybe_visited, current_total));
        }
        for (i, valid) in current_valves_remain.iter().enumerate() {
            if !valid {
                continue;
            }
            let valve = valves[i].clone();
            let mut current_valves_remain_copy = current_valves_remain.clone();
            current_valves_remain_copy[i] = false;

            let remain_min_1 = min_1 - dist_map[&current_valve_1][&valve] - 1;
            let potential_total_1 = remain_min_1 * &flow_map[&valve];
            if potential_total_1 > 0 {
                result = cmp::max(result, current_total + potential_total_1);
                stack.push((remain_min_1, valve.clone(), min_2, current_valve_2.clone(), current_valves_remain_copy.clone(), current_total + potential_total_1));
            }

            let remain_min_2 = min_2 - dist_map[&current_valve_2][&valve] - 1;
            let potential_total_2 = remain_min_2 * flow_map[&valve];
            if potential_total_2 > 0 {
                result = cmp::max(result, current_total + potential_total_2);
                stack.push((min_1, current_valve_1.clone(), remain_min_2, valve.clone(), current_valves_remain_copy.clone(), current_total + potential_total_2));
            }
        }
    }

    println!("{}", result);
}

pub fn solution() {
    part2();
}