use crate::reader;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Valve {
    name: String,
    output: usize,
    named_outputs: Vec<String>,
    outputs: Vec<usize>,
    w_outputs: Vec<(usize, usize)>,
}

impl FromStr for Valve {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (left, right) = str.split_once(';').unwrap();
        Ok(Self {
            name: left[6..8].to_string(),
            output: left[23..].parse().unwrap(),
            named_outputs: right[23..]
                .split(',')
                .map(|part| part.trim().to_string())
                .collect(),
            outputs: Vec::new(),
            w_outputs: Vec::new(),
        })
    }
}

pub fn run() {
    println!(
        "Day 16\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Valve> {
    reader::open("files/day16.txt").lines_as()
}

fn part_one(valves: Vec<Valve>) -> usize {
    dfs(1, 0, 0, 0, &0, &process(valves), &mut HashMap::new()).unwrap_or(0)
}

fn part_two(valves: Vec<Valve>) -> usize {
    dfs2(1, 0, 0, 0, 0, &0, &process(valves), &mut HashMap::new()).unwrap_or(0)
}

fn can_open(position: usize, open_valves: usize) -> bool {
    open_valves & (1 << position) == 0
}

fn new_mask(position: usize, open_valves: usize) -> usize {
    open_valves | (1 << position)
}

fn process(mut valves: Vec<Valve>) -> Vec<Valve> {
    valves.sort_by(|a, b| a.name.cmp(&b.name));
    let conversion_lookup = valves
        .iter()
        .enumerate()
        .map(|(i, valve)| (valve.name.to_string(), i))
        .collect::<HashMap<String, usize>>();
    let mut output = valves
        .into_iter()
        .map(|mut current| {
            current.outputs = current
                .named_outputs
                .iter()
                .map(|name| *conversion_lookup.get(name).unwrap())
                .collect();
            current
        })
        .collect::<Vec<Valve>>();
    for index in 0..output.len() {
        output[index].w_outputs = reduce_edges(index, 1, &mut 0, &output);
    }
    output
}

fn reduce_edges(
    src: usize,
    cost: usize,
    visited: &mut usize,
    lookup: &Vec<Valve>,
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if (1 << src) & *visited != 0 {
        return result;
    }
    *visited |= 1 << src;
    for edge in &lookup[src].outputs {
        if *&lookup[*edge].output > 0 {
            result.push((*edge, cost));
        } else {
            result.extend(reduce_edges(*edge, cost + 1, visited, lookup));
        }
    }
    result
}

fn dfs(
    minute: usize,
    index: usize,
    flow_rate: usize,
    current_score: usize,
    open_valves: &usize,
    valve_lookup: &Vec<Valve>,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> Option<usize> {
    if minute > 30 {
        return Some(current_score);
    }

    let key = (minute, flow_rate, index);
    if let Some(cached_value) = cache.get(&key) {
        if cached_value >= &current_score {
            return None;
        }
    }
    cache.insert(key, current_score);

    let current_valve = &valve_lookup[index];

    let open_current = if current_valve.output > 0 && can_open(index, *open_valves) {
        dfs(
            minute + 1,
            index,
            flow_rate + current_valve.output,
            current_score + flow_rate,
            &new_mask(index, *open_valves),
            valve_lookup,
            cache,
        )
    } else {
        None
    };

    current_valve
        .w_outputs
        .iter()
        .filter_map(|(next_valve, distance)| {
            let possible = std::cmp::min(31, minute + *distance) - minute;
            dfs(
                minute + possible,
                *next_valve,
                flow_rate,
                current_score + (flow_rate * possible),
                open_valves,
                valve_lookup,
                cache,
            )
        })
        .max()
        .max(open_current)
}

fn dfs2(
    minute: usize,
    a_index: usize,
    b_index: usize,
    flow_rate: usize,
    current_score: usize,
    open_valves: &usize,
    valve_lookup: &Vec<Valve>,
    cache: &mut HashMap<(usize, usize, usize, usize), usize>,
) -> Option<usize> {
    if minute > 26 {
        return Some(current_score);
    }

    let cache_key = (minute, flow_rate, a_index, b_index);
    if let Some(cached_value) = cache.get(&cache_key) {
        if *cached_value >= current_score {
            return None;
        }
    }
    cache.insert(cache_key, current_score);

    let a_flow_rate = valve_lookup[a_index].output;
    let b_flow_rate = valve_lookup[b_index].output;
    let can_open_a_valve = a_flow_rate > 0 && can_open(a_index, *open_valves);
    let can_open_b_valve = b_flow_rate > 0 && can_open(b_index, *open_valves);

    let mut results = Vec::new();

    if can_open_a_valve {
        let new_open_valves = new_mask(a_index, *open_valves);
        for next_index in &valve_lookup[b_index].outputs {
            results.push(dfs2(
                minute + 1,
                a_index,
                *next_index,
                flow_rate + a_flow_rate,
                current_score + flow_rate,
                &new_open_valves,
                valve_lookup,
                cache,
            ));
        }
    }

    if can_open_b_valve {
        let new_open_valves = new_mask(b_index, *open_valves);
        for new_my_location in &valve_lookup[a_index].outputs {
            results.push(dfs2(
                minute + 1,
                *new_my_location,
                b_index,
                flow_rate + b_flow_rate,
                current_score + flow_rate,
                &new_open_valves,
                valve_lookup,
                cache,
            ));
        }
    }

    if can_open_b_valve && can_open_a_valve && a_index != b_index {
        let new_open_valves = new_mask(a_index, new_mask(b_index, *open_valves));
        results.push(dfs2(
            minute + 1,
            a_index,
            b_index,
            flow_rate + a_flow_rate + b_flow_rate,
            current_score + flow_rate,
            &new_open_valves,
            valve_lookup,
            cache,
        ));
    }

    for next_b_index in &valve_lookup[b_index].outputs {
        for next_a_index in &valve_lookup[a_index].outputs {
            results.push(dfs2(
                minute + 1,
                *next_a_index,
                *next_b_index,
                flow_rate,
                current_score + flow_rate,
                open_valves,
                valve_lookup,
                cache,
            ));
        }
    }

    results.into_iter().flatten().max()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 1651);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 1707);
}

#[cfg(test)]
fn get_test_input() -> Vec<Valve> {
    reader::open("files/day16_test.txt").lines_as()
}
