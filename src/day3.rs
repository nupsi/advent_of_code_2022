use crate::reader;

struct Rucksack {
    chars: Vec<char>,
}

impl Rucksack {
    fn from(str: &str) -> Self {
        Self {
            chars: str.chars().collect(),
        }
    }

    fn find_common_elements(left: Vec<char>, rigth: Vec<char>) -> Vec<char> {
        let mut result = Vec::new();
        for char in left {
            if rigth.contains(&char) {
                result.push(char);
            }
        }
        result
    }

    fn find_common_elements_in_groups(groups: Vec<Vec<char>>) -> Vec<char> {
        let mut groups = groups;
        while groups.len() > 1 {
            groups = groups
                .windows(2)
                .map(|groups| Self::find_common_elements(groups[0].to_owned(), groups[1].to_owned()))
                .collect()
        }
        groups.into_iter().next().unwrap()
    }

    fn split_into(&self, n: usize) -> Vec<Vec<char>> {
        assert!(self.chars.len() % n == 0);
        self.chars
            .chunks(self.chars.len() / n)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn get_score(char: char) -> usize {
        let value = char as usize;
        if value > 96 {
            value - 96
        } else {
            value - 38
        }
    }
}

pub fn run() {
    println!(
        "Day 3\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Rucksack> {
    reader::open("files/day3.txt").parse_lines(Rucksack::from)
}

fn part_one(values: Vec<Rucksack>) -> usize {
    values
        .iter()
        .map(|sack| sack.split_into(2))
        .map(|groups| Rucksack::find_common_elements_in_groups(groups))
        .map(|common| *common.first().unwrap())
        .map(|char| Rucksack::get_score(char))
        .sum()
}

fn part_two(values: Vec<Rucksack>) -> usize {
    values
        .chunks(3)
        .map(|chunks| chunks.iter().map(|sack| sack.chars.to_owned()).collect())
        .map(|groups| Rucksack::find_common_elements_in_groups(groups))
        .map(|common| *common.first().unwrap())
        .map(|char| Rucksack::get_score(char))
        .sum()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 157);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 70);
}

#[cfg(test)]
fn get_test_input() -> Vec<Rucksack> {
    reader::open("files/day3_test.txt").parse_lines(Rucksack::from)
}
