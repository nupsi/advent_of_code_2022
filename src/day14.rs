use crate::reader;
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

type Point = (isize, isize);
type Map = HashSet<Point>;

enum Bottom {
    Void(isize),
    Floor(isize),
}

impl Bottom {
    fn value(&self) -> isize {
        match self {
            Self::Floor(n) => *n,
            Self::Void(n) => *n,
        }
    }

    fn is_floor(&self) -> bool {
        matches!(self, Self::Floor(_))
    }
}

#[derive(Debug)]
struct Scan {
    points: Vec<Point>,
}

impl FromStr for Scan {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            points: str
                .split(" -> ")
                .map(|part| {
                    let (x, y) = part.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect(),
        })
    }
}

impl Scan {
    fn draw(&self, map: &mut HashSet<Point>) -> isize {
        let mut max_y_value = 0;
        for window in self.points.windows(2) {
            let (start, end) = (&window[0], &window[1]);
            let min_x = std::cmp::min(start.0, end.0);
            let max_x = std::cmp::max(start.0, end.0);
            let min_y = std::cmp::min(start.1, end.1);
            let max_y = std::cmp::max(start.1, end.1);
            max_y_value = std::cmp::max(max_y_value, max_y);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    map.insert((x, y));
                }
            }
        }
        max_y_value
    }
}

pub fn run() {
    println!(
        "Day 14\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Scan> {
    reader::open("files/day14.txt").lines_as()
}

fn part_one(scans: Vec<Scan>) -> usize {
    let (y, mut map) = scans_to_map(scans);
    let bottom = Bottom::Void(y + 1);
    (0..).find(|_| !drop(&bottom, &mut map)).unwrap()
}

fn part_two(scans: Vec<Scan>) -> usize {
    let (y, mut map) = scans_to_map(scans);
    let bottom = Bottom::Floor(y + 2);
    (0..).find(|_| !drop(&bottom, &mut map)).unwrap()
}

fn scans_to_map(scans: Vec<Scan>) -> (isize, Map) {
    let mut map = HashSet::new();
    let mut max_y = 0;
    for scan in scans {
        let cur_y = scan.draw(&mut map);
        max_y = std::cmp::max(max_y, cur_y);
    }
    (max_y, map)
}

fn drop(bottom: &Bottom, map: &mut Map) -> bool {
    let mut point = (500, 0);
    if map.contains(&point) {
        return false;
    }
    while let Some(next) = try_get_next_point(&point, map) {
        if next.1 >= bottom.value() {
            if bottom.is_floor() {
                break;
            } else {
                return false
            }
        }
        point = next;
    }
    map.insert(point);
    true
}

fn try_get_next_point((x, y): &Point, map: &Map) -> Option<Point> {
    for (n, m) in [(0, 1), (-1, 1), (1, 1)] {
        if !map.contains(&(x + n, y + m)) {
            return Some((x + n, y + m))
        }
    }
    None
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 24);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 93);
}

#[cfg(test)]
fn get_test_input() -> Vec<Scan> {
    reader::open("files/day14_test.txt").lines_as()
}
