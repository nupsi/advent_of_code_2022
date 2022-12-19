use crate::reader;
use std::collections::{HashSet, LinkedList};
use std::str::FromStr;
use std::string::ParseError;

type Ores = Values;
type Robots = Values;
type Values = (usize, usize, usize, usize);

fn add(a: Values, b: Values) -> Values {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

fn sub(a: Values, b: Values) -> Values {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}

fn can_sub(a: Values, b: Values) -> bool {
    a.0 >= b.0 && a.1 >= b.1 && a.2 >= b.2 && a.3 >= b.3
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot: Values,
    clay_robot: Values,
    obsidian_robot: Values,
    geode_robot: Values,
}

impl FromStr for Blueprint {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (left, right) = str.split_once(':').unwrap();
        let mut robot_iter = right.split('.');
        Ok(Self {
            id: left[10..].parse().unwrap(),
            ore_robot: parse_material(robot_iter.next().unwrap()),
            clay_robot: parse_material(robot_iter.next().unwrap()),
            obsidian_robot: parse_material(robot_iter.next().unwrap()),
            geode_robot: parse_material(robot_iter.next().unwrap()),
        })
    }
}

impl Blueprint {
    fn max_ore(&self) -> usize {
        Self::max(
            self.ore_robot.0,
            self.clay_robot.0,
            self.obsidian_robot.0,
            self.geode_robot.0,
        )
    }

    fn max_clay(&self) -> usize {
        Self::max(
            self.ore_robot.1,
            self.clay_robot.1,
            self.obsidian_robot.1,
            self.geode_robot.1,
        )
    }

    fn max_obisidian(&self) -> usize {
        Self::max(
            self.ore_robot.2,
            self.clay_robot.2,
            self.obsidian_robot.2,
            self.geode_robot.2,
        )
    }

    fn max(a: usize, b: usize, c: usize, d: usize) -> usize {
        use std::cmp::max;
        max(a, max(b, max(c, d)))
    }
}

fn parse_material(input: &str) -> Values {
    let mut input_iter = input.trim().split(' ').skip(4);
    let mut materials = (0, 0, 0, 0);
    loop {
        let cost = input_iter.next().unwrap().parse().unwrap();
        let cost_type = input_iter.next().unwrap();
        let result = match cost_type {
            "ore" => (cost, 0, 0, 0),
            "clay" => (0, cost, 0, 0),
            "obsidian" => (0, 0, cost, 0),
            _ => panic!("Unkown cost type: '{:?}'", cost_type),
        };
        materials = add(materials, result);
        if input_iter.next().is_none() {
            break;
        }
    }
    materials
}

pub fn run() {
    println!(
        "Day 19\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Blueprint> {
    reader::open("files/day19.txt").lines_as()
}

fn part_one(blueprints: Vec<Blueprint>) -> usize {
    blueprints
        .into_iter()
        .map(|blueprint| {
            resolve(
                State::from((0, 0, 0, 0), (1, 0, 0, 0), 24),
                Refs::from(&blueprint, &mut HashSet::new()),
            ) * blueprint.id
        })
        .sum()
}

fn part_two(values: Vec<Blueprint>) -> usize {
    values
        .into_iter()
        .take(3)
        .map(|blueprint| {
            resolve(
                State::from((0, 0, 0, 0), (1, 0, 0, 0), 32),
                Refs::from(&blueprint, &mut HashSet::new()),
            )
        })
        .product()
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    ores: Ores,
    robots: Robots,
    time: usize,
}

impl State {
    fn from(ores: Ores, robots: Robots, time: usize) -> Self {
        Self { ores, robots, time }
    }

    fn next(&self, new_ores: Ores, new_robots: Robots) -> Self {
        Self {
            ores: add(new_ores, self.robots),
            robots: add(self.robots, new_robots),
            time: self.time - 1,
        }
    }
}

#[derive(Debug)]
struct Refs<'a> {
    blueprint: &'a Blueprint,
    mem: &'a mut HashSet<State>,
    max_consume: Values,
}

impl<'a> Refs<'a> {
    fn from(blueprint: &'a Blueprint, mem: &'a mut HashSet<State>) -> Self {
        Self {
            blueprint,
            mem,
            max_consume: (
                blueprint.max_ore(),
                blueprint.max_clay(),
                blueprint.max_obisidian(),
                0,
            ),
        }
    }

    fn has_over_production(&self, state: &State) -> bool {
        let (x, y, z, _) = self.max_consume;
        let (a, b, c, _) = state.robots;
        x < a || y < b || z < c
    }

    fn has_seen_state(&self, state: &State) -> bool {
        self.mem.get(state).is_some()
    }
}

fn resolve(initial: State, refs: Refs) -> usize {
    let (mut max, mut max_time) = (0, initial.time);
    let mut queue = LinkedList::new();
    queue.push_back(initial);

    while let Some(state) = queue.pop_front() {
        let new_max_value = max < state.ores.3;
        if new_max_value {
            max = std::cmp::max(max, state.ores.3);
            max_time = state.time;
        }

        if refs.has_over_production(&state) {
            continue;
        }

        let cannot_surpas_max_value = state.ores.3 + 2 < max && state.time <= max_time;
        if cannot_surpas_max_value {
            continue;
        }

        let ran_out_of_time = state.time == 0;
        if ran_out_of_time {
            continue;
        }

        if refs.has_seen_state(&state) {
            continue;
        }

        for (new_ores, new_robots) in permute(state.ores, refs.blueprint) {
            queue.push_back(state.next(new_ores, new_robots));
        }
        refs.mem.insert(state);
    }
    max
}

fn permute(ores: Ores, blueprint: &Blueprint) -> Vec<(Ores, Robots)> {
    if can_sub(ores, blueprint.geode_robot) {
        return vec![(sub(ores, blueprint.geode_robot), (0, 0, 0, 1))];
    }

    if can_sub(ores, blueprint.obsidian_robot) {
        return vec![
            (sub(ores, blueprint.obsidian_robot), (0, 0, 1, 0)),
            (ores, (0, 0, 0, 0)),
        ];
    }

    let mut result = Vec::with_capacity(3);

    if can_sub(ores, blueprint.clay_robot) {
        result.push((sub(ores, blueprint.clay_robot), (0, 1, 0, 0)));
    }

    if can_sub(ores, blueprint.ore_robot) {
        result.push((sub(ores, blueprint.ore_robot), (1, 0, 0, 0)));
    }

    result.push((ores, (0, 0, 0, 0)));

    result
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 33);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 56 * 62);
}

#[cfg(test)]
fn get_test_input() -> Vec<Blueprint> {
    reader::open("files/day19_test.txt").lines_as()
}
