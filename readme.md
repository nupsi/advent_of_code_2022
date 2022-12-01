# Advent of Code 2022

## Template
```rs
use crate::reader;

pub fn run() {
    println!(
        "Day x\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<String> {
    reader::open("files/day.txt").lines()
}

fn part_one(values: Vec<String>) -> usize {
    0
}

fn part_two(values: Vec<String>) -> usize {
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
fn get_test_input() -> Vec<String> {
    reader::open("files/day_test.txt").lines()
}
```