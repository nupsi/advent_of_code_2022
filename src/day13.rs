use crate::reader;
use std::cmp::Ordering;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
enum Signal {
    None,
    Left,
    Right,
}

impl Signal {
    fn ordered(&self) -> bool {
        matches!(self, Self::Right)
    }

    fn option(self) -> Option<Self> {
        match self {
            Self::None => None,
            _ => Some(self),
        }
    }
}

struct Packet {
    left: String,
    right: String,
}

impl FromStr for Packet {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut iter = str.lines();
        Ok(Self {
            left: iter.next().unwrap().to_string(),
            right: iter.next().unwrap().to_string(),
        })
    }
}

pub fn run() {
    println!(
        "Day 13\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Packet> {
    reader::open("files/day13.txt").split_on_empty_line_into()
}

fn part_one(packets: Vec<Packet>) -> usize {
    packets
        .into_iter()
        .enumerate()
        .filter(|(_, item)| compare(&item.left, &item.right).ordered())
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_two(packets: Vec<Packet>) -> usize {
    let size = (packets.len() + 1) * 2;
    packets
        .into_iter()
        .chain([Packet {
            left: "[[2]]".to_string(),
            right: "[[6]]".to_string(),
        }])
        .flat_map(|signal| [signal.left, signal.right])
        .fold(Vec::with_capacity(size), merge_insert)
        .into_iter()
        .enumerate()
        .filter(|(_, row)| row == "[[2]]" || row == "[[6]]")
        .map(|(i, _)| i + 1)
        .product()
}

fn merge_insert(current: Vec<String>, cur: String) -> Vec<String> {
    let mut current = current;
    for i in 0..current.len() {
        if compare(cur.as_str(), &current[i]).ordered() {
            current.insert(i, cur);
            return current;
        }
    }
    current.push(cur);
    current
}

fn compare(lhs: &str, rhs: &str) -> Signal {
    match (lhs.len(), rhs.len()) {
        (0, 0) => Signal::None,
        (0, _) => Signal::Right,
        (_, 0) => Signal::Left,
        _ => match (&lhs[0..1], &rhs[0..1]) {
            ("[", "[") => compare_lists(lhs, rhs),
            ("[", _) => compare(lhs, format!("[{}]", rhs).as_str()),
            (_, "[") => compare(format!("[{}]", lhs).as_str(), rhs),
            (_, _) => compare_values(lhs, rhs),
        },
    }
}

fn compare_lists(lhs: &str, rhs: &str) -> Signal {
    let (left, left_tail) = split_list(lhs);
    let (right, right_tail) = split_list(rhs);
    compare(left, right)
        .option()
        .unwrap_or_else(|| compare(left_tail, right_tail))
}

fn split_list(list: &str) -> (&str, &str) {
    let head = get_next(list);
    let tail = &list[head.len()..list.len()]
        .split_once(',')
        .unwrap_or(("", ""))
        .1;
    (head, tail)
}

fn compare_values(lhs: &str, rhs: &str) -> Signal {
    let (left_value, left_tail) = split_value(lhs);
    let (right_value, right_tail) = split_value(rhs);
    match left_value.cmp(&right_value) {
        Ordering::Greater => Signal::Left,
        Ordering::Equal => compare(left_tail, right_tail),
        Ordering::Less => Signal::Right,
    }
}

fn split_value(value: &str) -> (usize, &str) {
    let (value, tail) = value.split_once(',').unwrap_or((value, ""));
    (value.parse().unwrap(), tail)
}

fn get_next(input: &str) -> &str {
    let mut open = 0;
    for (i, char) in input.chars().enumerate() {
        match (char, open) {
            ('[', _) => open += 1,
            (']', 1) => return &input[1..i],
            (']', _) => open -= 1,
            _ => (),
        }
    }
    panic!("Unable to get next block {:?}", input);
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 13);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 140);
}

#[cfg(test)]
fn get_test_input() -> Vec<Packet> {
    reader::open("files/day13_test.txt").split_on_empty_line_into()
}
