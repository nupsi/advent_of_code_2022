use crate::reader;
use std::collections::HashMap;

pub fn run() {
    println!(
        "Day 7\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<String> {
    reader::open("files/day7.txt").lines()
}

fn part_one(commands: Vec<String>) -> usize {
    parse_commands(commands).into_values()
        .filter(|size| size <= &100_000)
        .sum()
}

fn part_two(commands: Vec<String>) -> usize {
    let dir = parse_commands(commands);
    let required = 30_000_000 - (70_000_000 - *dir.get("/").unwrap());
    dir.into_values()
        .filter(|size| size >= &required)
        .reduce(std::cmp::min)
        .unwrap()
}

fn parse_commands(lines: Vec<String>) -> HashMap<String, usize> {
    lines
        .into_iter()
        .rev()
        .fold(
            (HashMap::new(), 0),
            |(mut dir, mut size), command| match command.starts_with('$') {
                true => match command[2..].split_once(' ') {
                    Some((_, c)) => match c {
                        ".." => (dir, size),
                        c => {
                            dir.insert(c.to_owned(), size);
                            (dir, 0)
                        }
                    },
                    None => (dir, size),
                },
                false => {
                    size += match command.split_once(' ').unwrap() {
                        ("dir", d) => *dir.get(d).unwrap(),
                        (s, _) => s.parse().unwrap(),
                    };
                    (dir, size)
                }
            },
        )
        .0
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 95437);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 24933642);
}

#[cfg(test)]
fn get_test_input() -> Vec<String> {
    reader::open("files/day7_test.txt").lines()
}
