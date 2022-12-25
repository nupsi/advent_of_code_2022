use crate::reader;

pub fn run() {
    println!("Day 25\n\tPart 1: {:?}", part_one(input()));
}

fn input() -> Vec<String> {
    reader::open("files/day25.txt").lines()
}

fn part_one(values: Vec<String>) -> String {
    to_base_5(values.into_iter().map(|line| from_base_5(&line)).sum())
}

fn to_base_5(mut dec: isize) -> String {
    let mut chars = Vec::new();
    while dec > 0 {
        dec = match dec % 5 {
            4 => (dec + 1, chars.push('-')),
            3 => (dec + 2, chars.push('=')),
            n => (dec, chars.push(isize_to_char(n))),
        }
        .0 / 5;
    }
    chars.into_iter().rev().collect()
}

fn from_base_5(line: &str) -> isize {
    line.chars()
        .rev()
        .fold((0, 1), |(sum, i), cur| {
            (
                match cur {
                    '=' => sum + i * -2,
                    '-' => sum + i * -1,
                    n => sum + i * char_to_isize(n),
                },
                i * 5,
            )
        })
        .0
}

fn char_to_isize(char: char) -> isize {
    (char as isize) - ('0' as isize)
}

fn isize_to_char(n: isize) -> char {
    char::from_digit(n as u32, 10).unwrap()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), "2=-1=0");
}

#[cfg(test)]
fn get_test_input() -> Vec<String> {
    reader::open("files/day25_test.txt").lines()
}
