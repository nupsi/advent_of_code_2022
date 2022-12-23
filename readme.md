# Advent of Code 2022

Rust Version 1.66.0

## Template
```rs
use crate::reader;
use std::str::FromStr;
use std::string::ParseError;

type Point = (isize, isize);

struct Example {
    left: String,
    right: String,
}

impl FromStr for Example {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (left, right) = str.split_once(' ').unwrap();
        Ok(Self {
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

pub fn run() {
    println!(
        "Day x\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Example> {
    reader::open("files/day.txt").lines_as()
}

fn part_one(values: Vec<Example>) -> usize {
    0
}

fn part_two(values: Vec<Example>) -> usize {
    0
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 0);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 0);
}

#[cfg(test)]
fn get_test_input() -> Vec<Example> {
    reader::open("files/day_test.txt").lines_as()
}
```