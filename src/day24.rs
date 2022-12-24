use crate::reader;
use std::collections::{HashSet, LinkedList};

type Point = (isize, isize);

#[derive(Debug)]
struct Board {
    position: Point,
    offset: Point,
    start: Point,
    end: Point,
    minutes: isize,
    width: isize,
    height: isize,
    storms: Vec<(Point, char)>,
}

impl From<String> for Board {
    fn from(value: String) -> Self {
        let width = value.lines().next().unwrap().len() as isize - 2;
        let height = value.lines().count() as isize - 2;
        Self {
            position: (0, -1),
            start: (0, -1),
            end: (width - 1, height),
            offset: (0, 0),
            minutes: 0,
            width,
            height,
            storms: value
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, chr)| *chr != '.')
                        .filter(|(_, chr)| *chr != '#')
                        .map(|(x, chr)| ((x as isize - 1, y as isize - 1), chr))
                        .collect::<Vec<(Point, char)>>()
                })
                .collect(),
        }
    }
}

impl Board {
    fn next(&self) -> Board {
        Self {
            position: self.position,
            start: self.start,
            end: self.end,
            offset: (
                (self.offset.0 + 1) % self.width,
                (self.offset.1 + 1) % self.height,
            ),
            minutes: self.minutes + 1,
            width: self.width,
            height: self.height,
            storms: self
                .storms
                .iter()
                .map(|storm| self.next_storm(storm))
                .collect(),
        }
    }

    fn next_storm(&self, ((x, y), c): &(Point, char)) -> (Point, char) {
        let mut res = (*x, *y);
        match c {
            '<' => res.0 = (res.0 + self.width - 1) % self.width,
            '>' => res.0 = (res.0 + 1) % self.width,
            '^' => res.1 = (res.1 + self.height - 1) % self.height,
            'v' => res.1 = (res.1 + 1) % self.height,
            _ => panic!("Unknown direction: '{:?}'", c),
        }
        (res, *c)
    }

    fn copy(&self) -> Self {
        Self {
            position: self.position,
            start: self.start,
            end: self.end,
            offset: self.offset,
            minutes: self.minutes,
            width: self.width,
            height: self.height,
            storms: self.storms.to_vec(),
        }
    }

    fn is_free(&self, offset: Point) -> bool {
        let position = (self.position.0 + offset.0, self.position.1 + offset.1);
        let is_special = position == self.start || position == self.end; 
        let in_x_range = (0..self.width).contains(&position.0);
        let in_y_range = (0..self.height).contains(&position.1);
        if (!in_x_range || !in_y_range) && !is_special {
            return false;
        }

        self.storms.iter().all(|(point, _)| point != &position)
    }

    #[allow(dead_code)]
    fn get_point(&self, point: Point) -> char {
        let mut points = Vec::new();
        for storm in &self.storms {
            if storm.0 == point {
                points.push(storm.1);
            }
        }
        match points.len() {
            0 => '.',
            1 => points[0],
            _ => points.len().to_string().chars().next().unwrap(),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.position == (x, y) {
                    print!("E");
                } else {
                    print!("{}", self.get_point((x, y)));
                }
            }
            println!("");
        }
    }
}

pub fn run() {
    println!(
        "Day 24\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Board {
    reader::open("files/day24.txt").text().into()
}

fn part_one(board: Board) -> isize {
    let end = board.end;
    bfs(board, end).minutes
}

fn part_two(board: Board) -> isize {
    let start = board.start;
    let end = board.end;
    bfs(bfs(bfs(board, end), start), end).minutes
}

fn bfs(initial_board: Board, target: Point) -> Board {
    let mut mem = HashSet::new();
    let mut queue = LinkedList::new();
    queue.push_back(initial_board);
    while let Some(board) = queue.pop_front() {
        if board.position == target {
            return board;
        }

        let key = (board.position, board.offset);
        if mem.contains(&key) {
            continue;
        }
        mem.insert(key);

        let next_board = board.next();
        for offset in [(0, 0), (1, 0), (-1, 0), (0, -1), (0, 1)] {
            if next_board.is_free(offset) {
                let mut target_board = next_board.copy();
                target_board.position.0 += offset.0;
                target_board.position.1 += offset.1;
                queue.push_back(target_board);
            }
        }
    }
    panic!("Unable to find path!");
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 18);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 54);
}

#[cfg(test)]
fn get_test_input() -> Board {
    reader::open("files/day24_test.txt").text().into()
}
