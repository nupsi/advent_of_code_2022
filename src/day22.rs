use std::collections::HashMap;

use crate::reader;

type Point = (isize, isize);

#[derive(Debug)]
enum Move {
    F(usize),
    R,
    L,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| line.chars().into_iter().collect())
            .collect::<Vec<Vec<char>>>();
        let max = rows.iter().map(|row| row.len()).max().unwrap();

        Self {
            map: rows
                .into_iter()
                .map(|row| {
                    let len = row.len();
                    row.into_iter()
                        .chain((0..(max - len)).map(|_| ' '))
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Input {
    map: Map,
    moves: Vec<Move>,
}

impl From<String> for Input {
    fn from(input: String) -> Self {
        let (map, moves) = input.split_once("\r\n\r\n").unwrap();
        Self {
            map: map.into(),
            moves: Self::parse_moves(moves),
        }
    }
}

impl Input {
    fn parse_moves(str: &str) -> Vec<Move> {
        let mut char_iter = str.chars().into_iter();
        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;
        while let Some(next) = char_iter.next() {
            match next {
                '0'..='9' => end += 1,
                'R' => {
                    result.push(Move::F(str[start..end].parse().unwrap()));
                    result.push(Move::R);
                    end += 1;
                    start = end;
                }
                'L' => {
                    result.push(Move::F(str[start..end].parse().unwrap()));
                    result.push(Move::L);
                    end += 1;
                    start = end;
                }
                _ => panic!("Unkown charachter: {:?}", next),
            }
        }
        result.push(Move::F(str[start..end].parse().unwrap()));
        result
    }

    fn tile(&self, x: isize, y: isize) -> char {
        self.map.map[y as usize][x as usize]
    }

    fn is_side(&self, (x, y): Point) -> bool {
        self.tile(x, y) != ' '
    }

    fn width(&self) -> isize {
        self.map.map[0].len() as isize
    }

    fn height(&self) -> isize {
        self.map.map.len() as isize
    }
}

pub fn run() {
    println!(
        "Day 22\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Input {
    reader::open("files/day22.txt").text().into()
}

fn part_one(values: Input) -> isize {
    let mut dir = (1, 0);
    let mut pos = (
        values.map.map[0]
            .iter()
            .enumerate()
            .skip_while(|(_, n)| n != &&'.')
            .next()
            .unwrap()
            .0 as isize,
        0,
    );

    for rule in &values.moves {
        match rule {
            Move::F(n) => {
                for _ in 0..*n {
                    let (mut x, mut y) = next_in_bounds(
                        (pos.0 + dir.0, pos.1 + dir.1),
                        (values.width(), values.height()),
                    );

                    let mut next_tile = values.tile(x, y);
                    while next_tile == ' ' {
                        (x, y) = next_in_bounds(
                            (x + dir.0, y + dir.1),
                            (values.width(), values.height()),
                        );
                        next_tile = values.tile(x, y);
                    }

                    if next_tile == '.' {
                        pos = (x, y);
                    } else {
                        break;
                    }
                }
            }
            Move::L => dir = (dir.1, -dir.0),
            Move::R => dir = (-dir.1, dir.0),
        }
    }
    (1000 * (pos.1 + 1)) + (4 * (pos.0 + 1)) + dir_score(dir)
}

fn dir_score(dir: (isize, isize)) -> isize {
    match dir {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!("Unkown direction: {:?}", dir),
    }
}

fn part_two(values: Input) -> isize {
    hard_two(values)
}

#[allow(dead_code)]
fn print_minimap(values: Input) {
    let mut label_iter = ('A'..).into_iter();
    let size = (values.width() - values.height()).abs();
    for y in 0..(values.height() / size) {
        for x in 0..(values.width() / size) {
            if values.is_side((x * size, y * size)) {
                print!("{}", label_iter.next().unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn next_in_bounds((mut x, mut y): Point, (max_x, max_y): Point) -> Point {
    if x < 0 {
        x = max_x - 1;
    } else if x >= max_x {
        x = 0;
    }

    if y < 0 {
        y = max_y - 1;
    } else if y >= max_y {
        y = 0;
    }
    (x, y)
}

const SIZE: isize = 50;

fn hard_two(values: Input) -> isize {
    // Hard coded mappings for real input :(
    let mut mapping = HashMap::new();
    mapping.insert('A', vec![('B', 0), ('C', 0), ('D', 2), ('F', 1)]);
    mapping.insert('B', vec![('E', 2), ('C', 1), ('A', 0), ('F', 0)]);
    mapping.insert('C', vec![('B', 3), ('E', 0), ('D', 3), ('A', 0)]);
    mapping.insert('D', vec![('E', 0), ('F', 0), ('A', 2), ('C', 1)]);
    mapping.insert('E', vec![('B', 2), ('F', 1), ('D', 0), ('C', 0)]);
    mapping.insert('F', vec![('E', 3), ('B', 0), ('A', 3), ('D', 0)]);

    let mut pos = (0, 0);
    let mut block = 'A';
    let mut dir = 0;

    for rule in &values.moves {
        match rule {
            Move::F(n) => {
                for _ in 0..*n {
                    let (next_pos, next_dir, next_block) =
                        step(pos, dir, block, &mapping.get(&block).unwrap());
                    let (gx, gy) = to_global(next_pos, next_block);
                    let tile = values.tile(gx, gy);

                    assert_ne!(tile, ' ');
                    if values.tile(gx, gy) == '.' {
                        pos = next_pos;
                        dir = next_dir;
                        block = next_block;
                    } else {
                        break;
                    }
                }
            }
            Move::L => dir = (dir + 3) % 4,
            Move::R => dir = (dir + 1) % 4,
        }
    }

    pos = to_global(pos, block);
    (1000 * (pos.1 + 1)) + (4 * (pos.0 + 1)) + (dir as isize)
}

fn step(
    local_position: Point,
    dir: usize,
    block: char,
    rules: &Vec<(char, usize)>,
) -> (Point, usize, char) {
    let size = 50;
    let mut new_position = local_position;
    let mut new_block = block;
    let mut new_dir = dir;

    match dir {
        0 => new_position.0 += 1,
        1 => new_position.1 += 1,
        2 => new_position.0 -= 1,
        _ => new_position.1 -= 1,
    }

    if should_wrap(new_position) {
        let rule = &rules[dir];
        new_block = rule.0;

        new_position = (
            (new_position.0 + size) % size,
            (new_position.1 + size) % size,
        );

        for _ in 0..rule.1 {
            new_position = (size - new_position.1 - 1, new_position.0);
            new_dir = (new_dir + 1) % 4;
        }
    }

    (new_position, new_dir, new_block)
}

fn should_wrap(local_position: Point) -> bool {
    local_position.0 < 0 || local_position.1 < 0 || local_position.0 == 50 || local_position.1 == 50
}

fn to_global(pos: Point, cos: char) -> Point {
    match cos {
        'A' => (pos.0 + SIZE, pos.1),
        'B' => (pos.0 + (2 * SIZE), pos.1),
        'C' => (pos.0 + SIZE, pos.1 + SIZE),
        'D' => (pos.0, pos.1 + (2 * SIZE)),
        'E' => (pos.0 + SIZE, pos.1 + (2 * SIZE)),
        'F' => (pos.0, pos.1 + (3 * SIZE)),
        _ => panic!(),
    }
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 6032);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(input()), 147245);
}

#[cfg(test)]
fn get_test_input() -> Input {
    reader::open("files/day22_test.txt").text().into()
}
