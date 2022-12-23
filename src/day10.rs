use crate::reader;
use std::str::FromStr;
use std::string::ParseError;

enum Instruction {
    Noop,
    Add(i32),
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(match str.split_once(' ') {
            Some((_, value)) => Instruction::Add(value.parse().unwrap()),
            None => Instruction::Noop,
        })
    }
}

pub fn run() {
    println!(
        "Day 10\n\tPart 1: {:?}\n\tPart 2:\n{}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Instruction> {
    reader::open("files/day10.txt").lines_as()
}

fn part_one(values: Vec<Instruction>) -> i32 {
    execute_instructions(values)
        .into_iter()
        .enumerate()
        .map(|(i, x)| ((i + 1) as i32, x))
        .filter(|(i, _)| i % 40 == 20)
        .map(|(i, x)| i * x)
        .sum()
}

fn part_two(values: Vec<Instruction>) -> String {
    execute_instructions(values)
        .into_iter()
        .enumerate()
        .map(|(i, x)| ((i as i32) % 40, x))
        .map(|(i, x)| if (x - i).abs() < 2 { '#' } else { '.' })
        .take(240)
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|chunk| chunk.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
}

fn execute_instructions(values: Vec<Instruction>) -> Vec<i32> {
    values.into_iter().fold(vec![1], |mut cycles, instruction| {
        let x = *cycles.last().unwrap();
        match instruction {
            Instruction::Add(n) => {
                cycles.push(x);
                cycles.push(x + n);
            }
            Instruction::Noop => cycles.push(x),
        }
        cycles
    })
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 13140);
}

#[test]
fn test_part_two() {
    assert_eq!(
        part_two(get_test_input()),
        reader::open("files/day10_out.txt").text()
    );
}

#[cfg(test)]
fn get_test_input() -> Vec<Instruction> {
    reader::open("files/day10_test.txt").lines_as()
}
