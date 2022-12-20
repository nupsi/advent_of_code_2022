use crate::reader;

type Pair = (usize, isize);

pub fn run() {
    println!(
        "Day 20\n\tPart 1: {}\n\tPart 2: {}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Pair> {
    group_input(reader::open("files/day20.txt").lines_as())
}

fn group_input(input: Vec<isize>) -> Vec<Pair> {
    input.into_iter().enumerate().collect()
}

fn part_one(values: Vec<Pair>) -> isize {
    count_result(mix(values, 1, 1))
}

fn part_two(values: Vec<Pair>) -> isize {
    count_result(mix(values, 811_589_153, 10))
}

fn mix(original: Vec<Pair>, decryption_key: isize, mix_times: usize) -> Vec<Pair> {
    let mut values = original
        .iter()
        .copied()
        .map(|(i, n)| (i, n * decryption_key))
        .collect::<Vec<Pair>>();
    let len = values.len() as isize - 1;

    for _ in 0..mix_times {
        for (key, _) in &original {
            let start_index = index_of_key(&values, *key);
            let start = start_index as isize;
            let mut end_index = (start + values[start_index].1) % len;
            if end_index <= 0 {
                end_index += len;
            }
            let removed = values.remove(start_index);
            values.insert(end_index as usize, removed);
        }
    }
    values
}

fn count_result(values: Vec<Pair>) -> isize {
    let zero_index = index_of_value(&values, 0);
    vec![1000, 2000, 3000].into_iter().fold(0, |sum, offset| {
        sum + values[(zero_index + offset) % values.len()].1
    })
}

fn index_of(vec: &[Pair], f: impl Fn(&Pair) -> bool) -> usize {
    vec.iter().enumerate().find(|(_, x)| f(x)).unwrap().0
}

fn index_of_key(vec: &[Pair], key: usize) -> usize {
    index_of(vec, |(k, _)| *k == key)
}

fn index_of_value(vec: &[Pair], value: isize) -> usize {
    index_of(vec, |(_, v)| *v == value)
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 3);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 1_623_178_306);
}

#[cfg(test)]
fn get_test_input() -> Vec<Pair> {
    group_input(reader::open("files/day20_test.txt").lines_as())
}
