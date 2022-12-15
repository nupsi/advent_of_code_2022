use crate::reader;
use std::str::FromStr;
use std::string::ParseError;

struct Range {
    start: u32,
    end: u32,
}

impl From<&str> for Range {
    fn from(input: &str) -> Self {
        let (start, end) = input.split_once('-').unwrap();
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

impl Range {
    fn is_complete_overlap(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn has_some_overlap(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.end
    }
}

struct Section {
    left: Range,
    right: Range,
}

impl FromStr for Section {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (left, rigth) = str.split_once(',').unwrap();
        Ok(Self {
            left: left.into(),
            right: rigth.into(),
        })
    }
}

impl Section {
    fn has_complete_overlap(&self) -> bool {
        self.left.is_complete_overlap(&self.right) || self.right.is_complete_overlap(&self.left)
    }

    fn has_some_overlap(&self) -> bool {
        self.left.has_some_overlap(&self.right) || self.right.has_some_overlap(&self.left)
    }
}

pub fn run() {
    println!(
        "Day 4\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Section> {
    reader::open("files/day4.txt").lines_as()
}

fn part_one(sections: Vec<Section>) -> usize {
    sections
        .iter()
        .filter(|section| section.has_complete_overlap())
        .count()
}

fn part_two(sections: Vec<Section>) -> usize {
    sections
        .iter()
        .filter(|section| section.has_some_overlap())
        .count()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 2);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 4);
}

#[cfg(test)]
fn get_test_input() -> Vec<Section> {
    reader::open("files/day4_test.txt").lines_as()
}
