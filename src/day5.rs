use crate::reader;

use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Move {
    count: usize,
    source: usize,
    target: usize,
}

impl FromStr for Move {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut parts = str
            .split_whitespace()
            .map(|part| part.parse())
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap());
        Ok(Self {
            count: parts.next().unwrap(),
            source: parts.next().unwrap() - 1,
            target: parts.next().unwrap() - 1,
        })
    }
}

impl Move {
    fn parse_moves(moves: &str) -> Vec<Move> {
        moves.lines().map(|line| line.parse().unwrap()).collect()
    }
}

#[derive(Debug)]
struct Crane {
    crates: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl From<String> for Crane {
    fn from(input: String) -> Self {
        let (crates, moves) = input.split_once("\r\n\r\n").unwrap();
        Self {
            crates: Crane::parse_crates(crates),
            moves: Move::parse_moves(moves),
        }
    }
}

impl Crane {
    fn parse_crate_line(line: &str) -> Vec<Vec<char>> {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| vec![chunk[1]])
            .collect()
    }

    fn combine_rows(acc: Vec<Vec<char>>, cur: Vec<Vec<char>>) -> Vec<Vec<char>> {
        acc.into_iter().zip(cur).map(Self::build_stack).collect()
    }

    fn build_stack((top, bottom): (Vec<char>, Vec<char>)) -> Vec<char> {
        bottom
            .into_iter()
            .chain(top.into_iter())
            .into_iter()
            .filter(|char| char != &' ')
            .collect()
    }

    fn parse_crates(input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .filter(|line| line.contains('['))
            .map(Self::parse_crate_line)
            .reduce(Self::combine_rows)
            .unwrap()
    }

    fn crate_mover_9000(self) -> String {
        let mut crates = self.crates;
        for turn in self.moves {
            let source = crates[turn.source].clone();
            let (head, tail) = source.split_at(source.len() - turn.count);
            crates[turn.target].append(&mut tail.to_owned().into_iter().rev().collect());
            crates[turn.source] = head.to_vec();
        }
        Self::select_last(crates)
    }

    fn crate_mover_9001(self) -> String {
        let mut crates = self.crates;
        for turn in self.moves {
            let source = crates[turn.source].clone();
            let (head, tail) = source.split_at(source.len() - turn.count);
            crates[turn.target].append(&mut tail.to_vec());
            crates[turn.source] = head.to_vec();
        }
        Self::select_last(crates)
    }

    fn select_last(crates: Vec<Vec<char>>) -> String {
        crates
            .iter()
            .map(|current| current.last().unwrap())
            .collect()
    }
}

pub fn run() {
    println!(
        "Day 5\n\tPart 1: {}\n\tPart 2: {}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Crane {
    reader::open("files/day5.txt").text().into()
}

fn part_one(crane: Crane) -> String {
    crane.crate_mover_9000()
}

fn part_two(crane: Crane) -> String {
    crane.crate_mover_9001()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), "CMZ");
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), "MCD");
}

#[cfg(test)]
fn get_test_input() -> Crane {
    reader::open("files/day5_test.txt").text().into()
}
