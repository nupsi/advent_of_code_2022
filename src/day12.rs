use crate::reader;
use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);

#[derive(Debug)]
struct Heigthmap {
    start: Point,
    end: Point,
    data: Vec<Vec<usize>>,
}

impl From<String> for Heigthmap {
    fn from(input: String) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        Self {
            data: input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, char)| {
                            1 + match char {
                                'a'..='z' => (char as usize) - ('a' as usize),
                                'S' => {
                                    start = (x, y);
                                    0
                                }
                                'E' => {
                                    end = (x, y);
                                    ('z' as usize) - ('a' as usize)
                                }
                                _ => panic!("Unexpected charachter '{}'", char),
                            }
                        })
                        .collect()
                })
                .collect(),
            start,
            end,
        }
    }
}

impl Heigthmap {
    fn bfs(&self, win_condition: impl Fn(&Self, Point) -> bool) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_front((self.end, 0));
        while let Some((point, moves)) = queue.pop_front() {
            if win_condition(self, point) {
                return moves;
            }

            let heigth = self.get_point(point);
            for next_point in self.get_moves(point) {
                let next_heigth = self.get_point(next_point);
                let in_range = heigth - 1 <= next_heigth;
                if in_range && visited.insert(next_point) {
                    queue.push_back((next_point, (moves + 1)));
                }
            }
        }
        panic!("Did not reach last tile");
    }

    fn get_moves(&self, point: Point) -> Vec<Point> {
        let mut result = Vec::with_capacity(4);
        if point.0 > 0 {
            result.push((point.0 - 1, point.1));
        }
        if point.0 + 1 < self.width() {
            result.push((point.0 + 1, point.1));
        }
        if point.1 > 0 {
            result.push((point.0, point.1 - 1));
        }
        if point.1 + 1 < self.heigth() {
            result.push((point.0, point.1 + 1));
        }
        result
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn heigth(&self) -> usize {
        self.data.len()
    }

    fn get_point(&self, (x, y): Point) -> usize {
        self.data[y][x]
    }
}

pub fn run() {
    println!(
        "Day 12\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Heigthmap {
    reader::open("files/day12.txt").text().into()
}

fn part_one(heigthmap: Heigthmap) -> usize {
    heigthmap.bfs(|map, point| point == map.start)
}

fn part_two(heigthmap: Heigthmap) -> usize {
    heigthmap.bfs(|map, point| map.get_point(point) == 1)
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 31);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 29);
}

#[cfg(test)]
fn get_test_input() -> Heigthmap {
    reader::open("files/day12_test.txt").text().into()
}
