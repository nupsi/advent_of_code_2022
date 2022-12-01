use std::collections::BinaryHeap;

use crate::reader;

pub fn run() {
    println!(
        "Day 1\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<String> {
    reader::open("files/day1.txt").split_on_empty_line()
}

fn part_one(groups: Vec<String>) -> usize {
    sum_descending_calories(groups, 1)
}

fn part_two(groups: Vec<String>) -> usize {
    sum_descending_calories(groups, 3)
}

fn sum_descending_calories(groups: Vec<String>, n: usize) -> usize {
    get_descending_calories(groups).into_iter().take(n).sum()
}

fn get_descending_calories(groups: Vec<String>) -> BinaryHeap<usize> {
    groups
        .iter()
        .map(|part| {
            part.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum()
        })
        .collect::<BinaryHeap<usize>>()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 24000);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 45000);
}

#[cfg(test)]
fn get_test_input() -> Vec<String> {
    reader::open("files/day1_test.txt").split_on_empty_line()
}
