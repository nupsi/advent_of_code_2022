use crate::reader;
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

type Point = (i32, i32);

enum Direction {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

impl FromStr for Direction {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(match str.split_once(' ').unwrap() {
            ("U", n) => Direction::U(n.parse::<i32>().unwrap() + 1),
            ("D", n) => Direction::D(n.parse::<i32>().unwrap()),
            ("L", n) => Direction::L(n.parse::<i32>().unwrap()),
            ("R", n) => Direction::R(n.parse::<i32>().unwrap() + 1),
            _ => panic!("Unable to parse input."),
        })
    }
}

impl Direction {
    fn get_points(&self, c: Point) -> Vec<Point> {
        match &self {
            Direction::U(n) => ((c.1 + 1)..(c.1 + n)).map(|i| (c.0, i)).collect(),
            Direction::D(n) => ((c.1 - n)..c.1).rev().map(|i| (c.0, i)).collect(),
            Direction::L(n) => ((c.0 - n)..c.0).rev().map(|i| (i, c.1)).collect(),
            Direction::R(n) => ((c.0 + 1)..(c.0 + n)).map(|i| (i, c.1)).collect(),
        }
    }
}

struct Rope {
    head: Point,
    tail: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn with_length(length: usize) -> Self {
        Self {
            head: (0, 0),
            tail: (0..length).map(|_| (0, 0)).collect(),
            visited: HashSet::from_iter(vec![(0, 0)]),
        }
    }

    fn follow_direction(self, direction: Direction) -> Self {
        direction
            .get_points(self.head)
            .into_iter()
            .fold(self, |mut rope, point| {
                if rope.should_update_tail(point) {
                    rope.tail = rope
                        .tail
                        .into_iter()
                        .fold(vec![point], |mut acc, cur| {
                            acc.push(Rope::move_towards(cur, acc.last().unwrap()));
                            acc
                        })
                        .into_iter()
                        .skip(1)
                        .collect();
                    rope.visited.insert(*rope.tail.last().unwrap());
                }
                rope.head = point;
                rope
            })
    }

    fn move_towards(current: Point, target: &Point) -> Point {
        match (target.0 - current.0, target.1 - current.1) {
            (-1..=1, -1..=1) => current,
            (x, y) => (current.0 + x.signum(), current.1 + y.signum()),
        }
    }

    fn should_update_tail(&self, next: Point) -> bool {
        let (x, y) = self.tail.first().unwrap_or(&next);
        !matches!((next.0 - x, next.1 - y), (-1..=1, -1..=1))
    }

    fn unique_tail_positions(&self) -> usize {
        self.visited.len()
    }
}

pub fn run() {
    println!(
        "Day 9\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Direction> {
    reader::open("files/day9.txt").lines_as()
}

fn part_one(instructions: Vec<Direction>) -> usize {
    count_unique_tail_positions(instructions, 1)
}

fn part_two(instructions: Vec<Direction>) -> usize {
    count_unique_tail_positions(instructions, 9)
}

fn count_unique_tail_positions(instructions: Vec<Direction>, length: usize) -> usize {
    instructions
        .into_iter()
        .fold(Rope::with_length(length), |rope, direction| {
            rope.follow_direction(direction)
        })
        .unique_tail_positions()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input_one()), 13);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input_two()), 36);
}

#[cfg(test)]
fn get_test_input_one() -> Vec<Direction> {
    reader::open("files/day9_test1.txt").lines_as()
}

#[cfg(test)]
fn get_test_input_two() -> Vec<Direction> {
    reader::open("files/day9_test2.txt").lines_as()
}
