use crate::reader;
use std::collections::{HashMap, HashSet, LinkedList};
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Valve {
    name: String,
    output: usize,
    named_outputs: Vec<String>,
    i_outputs: Vec<(usize, isize)>,
    n_outputs: Vec<(String, isize)>,
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
            i_outputs: Vec::new(),
            n_outputs: Vec::new(),
        })
    }
}

trait ValveUtils {
    fn index_of(&self, name: &String) -> usize;
}

impl ValveUtils for Vec<Valve> {
    fn index_of(&self, name: &String) -> usize {
        for (i, valve) in self.iter().enumerate() {
            if &valve.name == name {
                return i;
            }
        }
        panic!("unable to find index");
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
    let valves = process(valves);
    dfs(30, valves.len() - 1, &0, &valves, &mut HashMap::new())
}

fn part_two(valves: Vec<Valve>) -> usize {
    let valves = process(valves);
    let mut cache = HashMap::new();
    let mut max = 0;
    let end = (1 << (valves.len() - 1)) - 1;
    for i in 0..(end + 1) / 2 {
        let x = dfs(26, valves.len() - 1, &i, &valves, &mut cache);
        let y = dfs(26, valves.len() - 1, &(end ^ i), &valves, &mut cache);
        max = std::cmp::max(max, x + y);
    }
    max
}

fn can_open(position: usize, open_valves: usize) -> bool {
    open_valves & (1 << position) == 0
}

fn new_mask(position: usize, open_valves: usize) -> usize {
    open_valves | (1 << position)
}

fn process(mut valves: Vec<Valve>) -> Vec<Valve> {
    let first = valves.remove(valves.index_of(&"AA".to_string()));
    valves.push(first);
    for index in 0..valves.len() {
        valves[index].n_outputs = reduce_edges(valves[index].name.to_string(), &valves);
    }

    let mut output = valves
        .into_iter()
        .filter(|valve| valve.output > 0 || valve.name == "AA")
        .collect::<Vec<Valve>>();

    for index in 0..output.len() {
        output[index].i_outputs = output[index]
            .n_outputs
            .iter()
            .map(|(name, w)| (output.index_of(name), *w))
            .collect();
    }

    output
}

fn reduce_edges(initial: String, lookup: &Vec<Valve>) -> Vec<(String, isize)> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = LinkedList::new();
    queue.push_back((initial, 1));

    while let Some((current, distance)) = queue.pop_front() {
        for edge in lookup[lookup.index_of(&current)].named_outputs.to_vec() {
            if visited.contains(&edge) {
                continue;
            }
            visited.insert(edge.to_string());

            if lookup[lookup.index_of(&edge)].output > 0 {
                result.push((edge.to_string(), distance));
            }
            queue.push_back((edge.to_string(), distance + 1));
        }
    }

    result
}

fn dfs(
    minute: isize,
    index: usize,
    open_valves: &usize,
    valve_lookup: &Vec<Valve>,
    cache: &mut HashMap<(isize, usize, usize), usize>,
) -> usize {
    let key = (minute, index, *open_valves);
    if let Some(known_result) = cache.get(&key) {
        return *known_result;
    }

    let mut result = 0;
    for (next, w) in &valve_lookup[index].i_outputs {
        if !can_open(*next, *open_valves) {
            continue;
        }

        let time = minute - *w - 1;
        if time <= 0 {
            continue;
        }

        let new_valves = new_mask(*next, *open_valves);
        let open = dfs(time, *next, &new_valves, valve_lookup, cache);
        let score = valve_lookup[*next].output * (time as usize);
        result = std::cmp::max(result, open + score);
    }

    cache.insert(key, result);
    result
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
