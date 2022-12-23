use crate::reader;
use std::collections::{HashMap, HashSet};

type Point = (isize, isize);

struct Board {
    points: HashSet<Point>,
    rules: Vec<(Point, [Point; 3])>,
}

impl From<String> for Board {
    fn from(input: String) -> Self {
        Self {
            points: input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(|(x, _)| (x as isize, y as isize))
                        .collect::<Vec<Point>>()
                })
                .flatten()
                .collect::<HashSet<Point>>(),
            rules: vec![
                ((0, -1), [(0, -1), (1, -1), (-1, -1)]),
                ((0, 1), [(0, 1), (1, 1), (-1, 1)]),
                ((-1, 0), [(-1, 0), (-1, 1), (-1, -1)]),
                ((1, 0), [(1, 0), (1, 1), (1, -1)]),
            ],
        }
    }
}

impl Board {
    fn bounds(&self) -> (isize, isize, isize, isize) {
        let (mut min_x, mut max_x) = (isize::MAX, isize::MIN);
        let (mut min_y, mut max_y) = (isize::MAX, isize::MIN);
        for point in &self.points {
            min_x = std::cmp::min(min_x, point.0);
            max_x = std::cmp::max(max_x, point.0);
            min_y = std::cmp::min(min_y, point.1);
            max_y = std::cmp::max(max_y, point.1);
        }
        (min_x, min_y, max_x, max_y)
    }

    fn width(&self) -> usize {
        let (min, _, max, _) = self.bounds();
        (max - min + 1) as usize
    }

    fn height(&self) -> usize {
        let (_, min, _, max) = self.bounds();
        (max - min + 1) as usize
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (min_x, min_y, max_x, max_y) = self.bounds();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.points.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn has_neighbour(&self, point: Point) -> bool {
        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                if self.points.contains(&(point.0 + x, point.1 + y)) {
                    return true;
                }
            }
        }
        false
    }

    fn get_proposition(&self, point: &Point, rule_offset: usize) -> Option<Point> {
        let rule_iter = self.rules.iter().cycle().skip(rule_offset).take(4);
        for (offset, rule) in rule_iter {
            if rule.iter().any(|offset| {
                self.points
                    .contains(&(point.0 + offset.0, point.1 + offset.1))
            }) {
                continue;
            }
            return Some((point.0 + offset.0, point.1 + offset.1));
        }
        None
    }
}

pub fn run() {
    println!(
        "Day 23\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Board {
    reader::open("files/day23.txt").text().into()
}

fn part_one(mut board: Board) -> usize {
    for i in 0..10 {
        next(&mut board, i);
    }
    board.width() * board.height() - board.len()
}

fn part_two(mut board: Board) -> usize {
    (0..).skip_while(|i| next(&mut board, *i)).next().unwrap() + 1
}

fn next(board: &mut Board, round: usize) -> bool {
    let mut moving = HashSet::with_capacity(board.len());
    let mut proposed: HashMap<Point, usize> = HashMap::with_capacity(board.len());

    for elf in &board.points {
        if !board.has_neighbour(*elf) {
            continue;
        }

        if let Some(proposition) = board.get_proposition(elf, round) {
            *proposed.entry(proposition).or_default() += 1;
            moving.insert(*elf);
        }
    }

    let mut new_points = HashSet::with_capacity(board.len());
    for elf in &board.points {
        if !moving.contains(elf) {
            new_points.insert(*elf);
            continue;
        }

        if let Some(proposition) = board.get_proposition(elf, round) {
            let canditates = proposed.get(&proposition).unwrap();
            if *canditates == 1 {
                new_points.insert(proposition);
            } else {
                new_points.insert(*elf);
            }
        }
    }

    assert_eq!(new_points.len(), board.points.len());
    board.points = new_points;
    moving.len() > 0
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 110);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 20);
}

#[cfg(test)]
fn get_test_input() -> Board {
    reader::open("files/day23_test.txt").text().into()
}
