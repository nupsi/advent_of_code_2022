use crate::reader;
use std::collections::{HashMap, HashSet};

type Point = (isize, isize, isize);

const DIRECTIONS: [Point; 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
];

trait PointUtil {
    fn add(&self, other: &Point) -> Point;
}

impl PointUtil for Point {
    fn add(&self, other: &Point) -> Point {
        (self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

pub fn run() {
    println!(
        "Day 18\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<(isize, isize, isize)> {
    reader::open("files/day18.txt").parse_lines(parse_line)
}

fn parse_line(line: &str) -> Point {
    let mut iter = line.split(",");
    let mut next = || iter.next().unwrap().parse().unwrap();
    (next(), next(), next())
}

fn part_one(points: Vec<Point>) -> usize {
    count_exposed(&points.into_iter().collect())
}

fn part_two(points: Vec<Point>) -> usize {
    let mut points = points.into_iter().collect::<HashSet<Point>>();
    let surrounding_air_blocks = get_surrounding_air_blocks(&points);
    add_internal_air_blocks(&mut points, &surrounding_air_blocks);
    count_exposed(&points)
}

fn count_exposed(points: &HashSet<Point>) -> usize {
    points
        .iter()
        .map(|point| {
            DIRECTIONS
                .iter()
                .filter(|dir| !points.contains(&point.add(dir)))
                .count()
        })
        .sum()
}

fn get_surrounding_air_blocks(points: &HashSet<Point>) -> HashMap<Point, usize> {
    let mut result = HashMap::new();
    for point in points {
        for offset in &DIRECTIONS {
            let point = point.add(offset);
            if !points.contains(&point) {
                *result.entry(point).or_insert(0) += 1;
            }
        }
    }
    result
}

fn add_internal_air_blocks(
    points: &mut HashSet<Point>,
    surrounding_air_blocks: &HashMap<Point, usize>,
) {
    let bounds = calculate_bounds(points);
    let mut internal_air = HashSet::new();
    for (key, _) in surrounding_air_blocks
        .iter()
        .filter(|(point, value)| **value > 1 && in_bounds(point, &bounds))
    {
        if internal_air.contains(key) {
            continue;
        }

        if let Some(air) = dfs(*key, &mut HashSet::new(), &points, &bounds) {
            internal_air.extend(air)
        }
    }

    for block in internal_air {
        points.insert(block);
    }
}

fn in_bounds(point: &Point, bounds: &Point) -> bool {
    return point.0 > 0
        && point.0 <= bounds.0
        && point.1 > 0
        && point.1 <= bounds.1
        && point.2 > 0
        && point.2 <= bounds.2;
}

fn calculate_bounds(points: &HashSet<Point>) -> Point {
    let mut bounds = (0, 0, 0);
    for point in points {
        bounds.0 = std::cmp::max(bounds.0, point.0);
        bounds.1 = std::cmp::max(bounds.1, point.1);
        bounds.2 = std::cmp::max(bounds.2, point.2);
    }
    bounds
}

fn dfs(
    current: Point,
    seen: &mut HashSet<Point>,
    solid: &HashSet<Point>,
    bounds: &Point,
) -> Option<Vec<Point>> {
    if seen.contains(&current) {
        return Some(Vec::new());
    }

    let mut result = Vec::new();
    result.push(current);
    seen.insert(current);

    for offset in &DIRECTIONS {
        let point = current.add(offset);
        if !in_bounds(&point, bounds) {
            return None;
        }

        if solid.contains(&point) || seen.contains(&point) {
            continue;
        }

        match dfs(point, seen, solid, bounds) {
            Some(child_values) => result.extend(child_values),
            None => return None,
        }
    }
    Some(result)
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 64);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 58);
}

#[cfg(test)]
fn get_test_input() -> Vec<Point> {
    reader::open("files/day18_test.txt").parse_lines(parse_line)
}
