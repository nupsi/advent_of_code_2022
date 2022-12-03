use crate::reader;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
struct Round {
    elf: Shape,
    player: Shape,
}

impl FromStr for Round {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (elf, player) = str.split_once(" ").unwrap();
        Ok(Self {
            elf: elf.parse().unwrap(),
            player: player.parse().unwrap(),
        })
    }
}

impl Round {
    fn play_round(&self) -> usize {
        self.player.score() + self.get_score()
    }

    fn get_score(&self) -> usize {
        match (self.elf, self.player) {
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => 0,
            _ => 3,
        }
    }

    fn expected_outcome_to_actual(&self) -> Self {
        Self {
            elf: self.elf,
            player: match (self.player, self.elf) {
                (Shape::Rock, Shape::Rock) => Shape::Scissors,
                (Shape::Rock, Shape::Paper) => Shape::Rock,
                (Shape::Rock, Shape::Scissors) => Shape::Paper,

                (Shape::Paper, n) => n,

                (Shape::Scissors, Shape::Rock) => Shape::Paper,
                (Shape::Scissors, Shape::Paper) => Shape::Scissors,
                (Shape::Scissors, Shape::Scissors) => Shape::Rock,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => panic!("Unable create shape from: '{:?}'.", str),
        }
    }
}

impl Shape {
    fn score(&self) -> usize {
        *self as usize
    }
}

pub fn run() {
    println!(
        "Day 2\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Round> {
    reader::open("files/day2.txt").lines_as()
}

fn part_one(rounds: Vec<Round>) -> usize {
    rounds.iter().map(|round| round.play_round()).sum()
}

fn part_two(rounds: Vec<Round>) -> usize {
    rounds
        .iter()
        .map(|round| round.expected_outcome_to_actual().play_round())
        .sum()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 15);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 12);
}

#[cfg(test)]
fn get_test_input() -> Vec<Round> {
    reader::open("files/day2_test.txt").lines_as()
}
