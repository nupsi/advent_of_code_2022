use crate::reader;

pub fn run() {
    println!(
        "Day 6\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> String {
    reader::open("files/day6.txt").text()
}

fn part_one(input: String) -> usize {
    solve(input, 4)
}

fn part_two(input: String) -> usize {
    solve(input, 14)
}

fn solve(input: String, n: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .enumerate()
        .filter(|(_, window)| are_all_charachters_unique(window))
        .map(|(index, window)| index + window.len())
        .next()
        .unwrap()
}

fn are_all_charachters_unique(str: &[char]) -> bool {
    let mut bitmask = 0;
    str.iter()
        .map(|char| (*char as usize) - ('a' as usize))
        .all(|offset| {
            let is_unique = bitmask & (1 << offset) == 0;
            bitmask |= 1 << offset;
            is_unique
        })
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 7);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 19);
}

#[cfg(test)]
fn get_test_input() -> String {
    reader::open("files/day6_test.txt").text()
}
