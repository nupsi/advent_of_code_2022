use crate::reader;
use std::collections::HashMap;

type Shape = Vec<usize>;

pub fn run() {
    println!(
        "Day 17\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> String {
    reader::open("files/day17.txt").text()
}

fn part_one(values: String) -> usize {
    play(values, 2022)
}

fn part_two(values: String) -> usize {
    play(values, 1_000_000_000_000)
}

fn play(rules: String, goal_rock_count: usize) -> usize {
    let shapes = get_shapes();
    let mut rule_iter = rules.chars().into_iter().cycle().peekable();
    let mut shape_iter = shapes.iter().enumerate().cycle();
    let mut stack = Vec::with_capacity(4000);
    let mut mem = HashMap::new();
    let mut dropped_rocks = 0;
    let mut skipped = 0;

    while dropped_rocks < goal_rock_count {
        let (id, original_shape) = shape_iter.next().unwrap();
        let initial_move = *rule_iter.peek().unwrap();
        let shape_height = original_shape.len();
        stack.extend((0..3 + shape_height).map(|_| 0));
        let mut current_shape = original_shape.to_vec();
        'inifine: for y in 0.. {
            let rule = rule_iter.next().unwrap();
            let end = stack.len() - y;
            let start = end - shape_height;

            'horizontal: {
                for (shape_index, board_index) in (start..end).enumerate() {
                    let shape_row = current_shape[shape_height - shape_index - 1];
                    let board_row = stack[board_index];
                    if !can_move(&shape_row, &board_row, &rule) {
                        break 'horizontal;
                    }
                }

                for row in &mut current_shape {
                    *row = shift(row, &rule);
                }
            }

            'vertical: {
                if shape_height + y == stack.len() {
                    break 'vertical;
                }

                for (shape_index, board_index) in (start..end).enumerate() {
                    if current_shape[shape_height - shape_index - 1] & stack[board_index - 1] != 0 {
                        break 'vertical;
                    }
                }

                continue 'inifine;
            }

            for (shape_index, board_index) in (start..end).enumerate() {
                stack[board_index] |= current_shape[shape_height - shape_index - 1];
            }
            break;
        }

        while !stack.is_empty() && stack[stack.len() - 1] == 0 {
            stack.pop();
        }

        let start = std::cmp::max(0, stack.len() as isize - 30) as usize;
        let sample = stack[start..stack.len()].to_owned();
        let key = (id, initial_move, sample);

        if let Some((previous_count, previous_height)) = mem.get(&key) {
            let heigth_offset = stack.len() - previous_height;
            let rock_count_offset = dropped_rocks - previous_count;
            let n = (goal_rock_count - dropped_rocks) / rock_count_offset;
            skipped += heigth_offset * n;
            dropped_rocks += rock_count_offset * n;
        }

        let value = (dropped_rocks, stack.len());
        mem.insert(key, value);

        dropped_rocks += 1;
    }
    skipped + stack.len()
}

fn get_shapes() -> Vec<Shape> {
    vec![
        vec![(1 << 4) | (1 << 3) | (1 << 2) | (1 << 1)],
        vec![(1 << 3), (1 << 4) | (1 << 3) | (1 << 2), (1 << 3)],
        vec![(1 << 2), (1 << 2), (1 << 4) | (1 << 3) | (1 << 2)],
        vec![(1 << 4), (1 << 4), (1 << 4), (1 << 4)],
        vec![(1 << 4) | (1 << 3), (1 << 4) | (1 << 3)],
    ]
}

#[allow(dead_code)]
fn draw(stack: &[usize]) {
    for i in stack.iter().rev() {
        let x = format!("{:#09b}", i)[2..]
            .replace('0', ".")
            .replace('1', "#");
        println!("|{}|", x);
    }
    println!("+-------+");
}

fn can_shift_in_room(a: &usize, rule: &char) -> bool {
    match rule {
        '<' => a & (1 << 6) == 0,
        '>' => a & 1 == 0,
        chr => panic!("Unkown rule: '{:?}'", chr),
    }
}

fn shift(a: &usize, rule: &char) -> usize {
    match rule {
        '<' => a << 1,
        '>' => a >> 1,
        _ => panic!(),
    }
}

fn can_move(shape: &usize, board: &usize, rule: &char) -> bool {
    can_shift_in_room(shape, rule) && shift(shape, rule) & board == 0
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 3068);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 1_514_285_714_288);
}

#[cfg(test)]
fn get_test_input() -> String {
    reader::open("files/day17_test.txt").text()
}
