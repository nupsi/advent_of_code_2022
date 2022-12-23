use std::ops::Range;

use crate::reader;

type Point = (usize, usize);

enum Visibility {
    Visible(usize),
    Blocked(usize),
}

impl Visibility {
    fn value(&self) -> usize {
        match self {
            Visibility::Blocked(n) => *n,
            Visibility::Visible(n) => *n,
        }
    }

    fn is_visible(&self) -> bool {
        matches!(self, Self::Visible(_))
    }
}

#[derive(Debug)]
enum Trace {
    U(usize, Range<usize>),
    D(usize, Range<usize>),
    L(usize, Range<usize>),
    R(usize, Range<usize>),
}

impl Trace {
    fn get_points(&self) -> Vec<Point> {
        match self {
            Self::U(n, r) => (r.start..r.end).map(|i| (*n, i)).rev().collect(),
            Self::D(n, r) => ((r.start + 1)..r.end).map(|i| (*n, i)).collect(),
            Self::L(n, r) => (r.start..r.end).map(|i| (i, *n)).rev().collect(),
            Self::R(n, r) => ((r.start + 1)..r.end).map(|i| (i, *n)).collect(),
        }
    }
}

struct Forest {
    trees: Vec<usize>,
    width: usize,
    heigth: usize,
}

impl From<String> for Forest {
    fn from(input: String) -> Self {
        Self {
            width: input.lines().next().unwrap().len(),
            heigth: input.lines().count(),
            trees: input
                .replace("\n", "")
                .chars()
                .map(|char| {
                    println!("{:?} -> {}", char, char as usize);
                    (char as usize) - ('0' as usize)
                })
                .collect(),
        }
    }
}

impl Forest {
    fn is_visible(&self, point: Point) -> bool {
        self.get_visibility(point)
            .into_iter()
            .any(|visibility| visibility.is_visible())
    }

    fn scenic_score(&self, point: Point) -> usize {
        self.get_visibility(point)
            .into_iter()
            .map(|visibility| visibility.value())
            .product()
    }

    fn trace(&self, value: usize, trace: Trace) -> Visibility {
        let mut distance = 0;
        for point in trace.get_points() {
            distance += 1;
            if self.get_value(point) >= &value {
                return Visibility::Blocked(distance);
            }
        }
        Visibility::Visible(distance)
    }

    fn get_visibility(&self, (x, y): Point) -> Vec<Visibility> {
        let current = *self.get_value((x, y));
        vec![
            self.trace(current, Trace::L(y, 0..x)),
            self.trace(current, Trace::R(y, x..self.width())),
            self.trace(current, Trace::U(x, 0..y)),
            self.trace(current, Trace::D(x, y..self.heigth())),
        ]
    }

    fn get_points(&self) -> Vec<Point> {
        (0..self.trees.len())
            .map(|i| self.index_to_point(i))
            .collect()
    }

    fn get_value(&self, (x, y): Point) -> &usize {
        &self.trees[x + (y * self.width())]
    }

    fn index_to_point(&self, i: usize) -> Point {
        (i % self.width(), i / self.heigth())
    }

    fn width(&self) -> usize {
        self.width
    }

    fn heigth(&self) -> usize {
        self.heigth
    }
}

pub fn run() {
    println!(
        "Day 8\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Forest {
    reader::open("files/day8.txt").text().into()
}

fn part_one(forest: Forest) -> usize {
    forest
        .get_points()
        .into_iter()
        .filter(|point| forest.is_visible(*point))
        .count()
}

fn part_two(forest: Forest) -> usize {
    forest.get_points().into_iter().fold(0, |max, point| {
        std::cmp::max(max, forest.scenic_score(point))
    })
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 21);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 8);
}

#[cfg(test)]
fn get_test_input() -> Forest {
    reader::open("files/day8_test.txt").text().into()
}
