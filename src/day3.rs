use crate::reader;
use core::panic;
use std::str::FromStr;
use std::string::ParseError;

struct Rucksack {
    chars: Vec<char>,
}

impl FromStr for Rucksack {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            chars: str.chars().collect(),
        })
    }
}

impl Rucksack {
    fn find_common_elements(left: Vec<char>, right: Vec<char>) -> Vec<char> {
        left.into_iter()
            .filter(|current| right.contains(current))
            .collect()
    }

    fn find_common_element_in_groups(groups: Vec<Vec<char>>) -> char {
        *groups
            .into_iter()
            .reduce(Self::find_common_elements)
            .unwrap()
            .first()
            .unwrap()
    }

    fn split_into(&self, n: usize) -> Vec<Vec<char>> {
        assert!(self.chars.len() % n == 0);
        self.chars
            .chunks(self.chars.len() / n)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn get_score(char: char) -> usize {
        match char {
            'a'..='z' => (char as usize) - ('a' as usize) + 1,
            'A'..='Z' => (char as usize) - ('A' as usize) + 1 + 26,
            _ => panic!("Invalid input charachter: '{:?}'.", char),
        }
    }
}

pub fn run() {
    println!(
        "Day 3\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Rucksack> {
    reader::open("files/day3.txt").lines_as()
}

fn part_one(rucksacks: Vec<Rucksack>) -> usize {
    rucksacks
        .iter()
        .map(|sack| sack.split_into(2))
        .map(Rucksack::find_common_element_in_groups)
        .map(Rucksack::get_score)
        .sum()
}

fn part_two(rucksacks: Vec<Rucksack>) -> usize {
    rucksacks
        .chunks(3)
        .map(|chunks| chunks.iter().map(|sack| sack.chars.to_owned()).collect())
        .map(Rucksack::find_common_element_in_groups)
        .map(Rucksack::get_score)
        .sum()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 157);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 70);
}

#[cfg(test)]
fn get_test_input() -> Vec<Rucksack> {
    reader::open("files/day3_test.txt").lines_as()
}
