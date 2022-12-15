use crate::reader;
use std::str::FromStr;
use std::string::ParseError;

type Point = (isize, isize);

#[derive(Debug)]
struct Sensor {
    position: Point,
    distance: isize,
}

impl FromStr for Sensor {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (sensor_str, beacon_str) = str.split_once(':').unwrap();
        let (sx, sy) = sensor_str[10..].split_once(", ").unwrap();
        let (bx, by) = beacon_str[22..].split_once(", ").unwrap();
        let (sx, sy): Point = (sx[2..].parse().unwrap(), sy[2..].parse().unwrap());
        let (bx, by): Point = (bx[2..].parse().unwrap(), by[2..].parse().unwrap());
        Ok(Self {
            position: (sx, sy),
            distance: (sx - bx).abs() + (sy - by).abs(),
        })
    }
}

pub fn run() {
    println!(
        "Day 15\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input(), 2_000_000),
        part_two(input(), 4_000_000)
    );
}

fn input() -> Vec<Sensor> {
    reader::open("files/day15.txt").lines_as()
}

fn part_one(pairs: Vec<Sensor>, y: isize) -> isize {
    let mut ranges = Vec::with_capacity(pairs.len());
    fill_ranges(&mut ranges, &pairs, y, isize::MIN, isize::MAX);
    let result = eval_range(&mut ranges);
    result.1 - result.0
}

fn part_two(pairs: Vec<Sensor>, max: isize) -> isize {
    let mut ranges = Vec::with_capacity(pairs.len());
    for y in 0..=max {
        fill_ranges(&mut ranges, &pairs, y, 0, max);
        let result = eval_range(&mut ranges);
        if result.1 != max {
            return (result.1 + 1) * 4000000 + y;
        }
    }
    0
}

fn fill_ranges(ranges: &mut Vec<Point>, sensors: &Vec<Sensor>, y: isize, min: isize, max: isize) {
    ranges.clear();
    for sensor in sensors {
        let y_offset = (y - sensor.position.1).abs();
        if y_offset <= sensor.distance {
            let left = std::cmp::max(min, (sensor.position.0 - sensor.distance) + y_offset);
            let right = std::cmp::min(max, (sensor.position.0 + sensor.distance) - y_offset);
            if left <= right {
                ranges.push((left, right));
            }
        }
    }
}

fn eval_range(ranges: &mut [Point]) -> Point {
    ranges.sort();
    let (left, mut right) = (ranges[0].0, ranges[0].1);
    for range in ranges.iter().skip(1) {
        if range.0 <= right + 1 {
            right = std::cmp::max(right, range.1);
        } else {
            return (left, right);
        }
    }
    (left, right)
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input(), 10), 26);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input(), 20), 56_000_011);
}

#[cfg(test)]
fn get_test_input() -> Vec<Sensor> {
    reader::open("files/day15_test.txt").lines_as()
}
