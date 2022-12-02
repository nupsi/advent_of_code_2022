use crate::reader;

type Round = (Shape, Shape);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn from(str: &str) -> Shape {
        match str {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Unable create shape from: '{:?}'.", str),
        }
    }

    fn score(&self) -> usize {
        *self as usize
    }

    fn play(player: Shape, elf: Shape) -> usize {
        player.score() + Shape::outcome(player, elf)
    }

    fn play_expected(expected: Shape, elf: Shape) -> usize {
        match (expected, elf) {
            (Self::Rock, Shape::Rock) => Shape::play(Shape::Scissors, elf),
            (Self::Rock, Shape::Paper) => Shape::play(Shape::Rock, elf),
            (Self::Rock, Shape::Scissors) => Shape::play(Shape::Paper, elf),

            (Self::Paper, n) => Shape::play(n.to_owned(), elf),

            (Self::Scissors, Shape::Rock) => Shape::play(Shape::Paper, elf),
            (Self::Scissors, Shape::Paper) => Shape::play(Shape::Scissors, elf),
            (Self::Scissors, Shape::Scissors) => Shape::play(Shape::Rock, elf),
        }
    }

    fn outcome(player: Shape, elf: Shape) -> usize {
        match (elf, player) {
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => 0,
            _ => 3,
        }
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
    reader::open("files/day2.txt").parse_lines(parse_line)
}

fn parse_line(line: &str) -> Round {
    let (elf, player) = line.split_once(" ").unwrap();
    (Shape::from(player), Shape::from(elf))
}

fn part_one(values: Vec<Round>) -> usize {
    values
        .into_iter()
        .map(|(player, elf)| Shape::play(player, elf))
        .sum()
}

fn part_two(values: Vec<Round>) -> usize {
    values
        .into_iter()
        .map(|(expected, elf)| Shape::play_expected(expected, elf))
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
    reader::open("files/day2_test.txt").parse_lines(parse_line)
}
