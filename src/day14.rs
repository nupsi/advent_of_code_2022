use crate::reader;
use std::str::FromStr;
use std::string::ParseError;

const X_OFFSET: usize = 300;

type Point = (usize, usize);
type Size = (usize, usize);
type Map = Vec<Vec<char>>;

trait Utils {
    fn is_free(&self, point: &Point) -> bool;
    fn set_point(&mut self, x: usize, y: usize, value: char);
}

impl Utils for Map {
    fn is_free(&self, (x, y): &Point) -> bool {
        self[*y][*x] == '.'
    }

    fn set_point(&mut self, x: usize, y: usize, value: char) {
        self[y][x] = value;
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
                    let (x, y) = part.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect(),
        })
    }
}

impl Scan {
    fn draw(&self, mut map: Map) -> Map {
        for window in self.points.windows(2) {
            let (start, end) = (&window[0], &window[1]);
            let min_x = std::cmp::min(start.0, end.0);
            let max_x = std::cmp::max(start.0, end.0);
            let min_y = std::cmp::min(start.1, end.1);
            let max_y = std::cmp::max(start.1, end.1);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    map.set_point(x, y, '#');
                }
            }
        }
        map
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
    let (starting_point, mut map) = get_initial_values(scans);
    (0..)
        .skip_while(|_| drop_sand(&starting_point, &mut map))
        .next()
        .unwrap()
}

fn part_two(scans: Vec<Scan>) -> usize {
    let (starting_point, mut map) = get_initial_values(scans);
    map.push((0..=map[0].len()).into_iter().map(|_| '#').collect());
    (0..)
        .skip_while(|_| drop_sand(&starting_point, &mut map))
        .next()
        .unwrap()
}

fn get_initial_values(scans: Vec<Scan>) -> (Point, Map) {
    let (points, bounds) = relocate(scans);
    (
        (500 - X_OFFSET + 1, 1),
        points
            .into_iter()
            .fold(initialize_map(&bounds), |map, scan| scan.draw(map)),
    )
}

fn initialize_map(bounds: &Size) -> Map {
    (0..=bounds.1)
        .into_iter()
        .map(|_| (0..=bounds.0).into_iter().map(|_| '.').collect())
        .collect()
}

fn drop_sand(point: &Point, map: &mut Map) -> bool {
    if !map.is_free(&point) {
        return false;
    }
    
    let mut point = (point.0, point.1);
    loop {
        if map.is_free(&(point.0, point.1 + 1)) {
            point.1 += 1;
        } else if map.is_free(&(point.0 - 1, point.1 + 1)) {
            point.0 -= 1;
            point.1 += 1;
        } else if map.is_free(&(point.0 + 1, point.1 + 1)) {
            point.0 += 1;
            point.1 += 1;
        } else {
            break;
        }

        if point.1 + 1 == map.len() {
            return false;
        }
    }
    map[point.1][point.0] = 'O';
    true
}

fn relocate(scans: Vec<Scan>) -> (Vec<Scan>, Size) {
    let mut bounds = (0, 0);

    for scan in &scans {
        for point in &scan.points {
            bounds.0 = std::cmp::max(bounds.0, point.0);
            bounds.1 = std::cmp::max(bounds.1, point.1 + 2);
        }
    }

    (
        scans
            .into_iter()
            .map(|scan| Scan {
                points: scan
                    .points
                    .into_iter()
                    .map(|point| (1 + point.0 - X_OFFSET, 1 + point.1))
                    .collect(),
            })
            .collect(),
        bounds,
    )
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
