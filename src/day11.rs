use crate::reader;
use std::str::{FromStr, Lines};
use std::string::ParseError;

#[derive(Debug)]
struct Operation {
    lhs: usize,
    rhs: usize,
    operator: char,
}

impl From<&str> for Operation {
    fn from(str: &str) -> Self {
        let mut iter = str[19..].split(" ").into_iter();
        Self {
            lhs: iter.next().unwrap().parse().unwrap_or(0),
            operator: iter.next().unwrap().chars().next().unwrap(),
            rhs: iter.next().unwrap().parse().unwrap_or(0),
        }
    }
}

impl Operation {
    fn execute(&self, old: usize) -> usize {
        let lhs = if self.lhs == 0 { old } else { self.lhs };
        let rhs = if self.rhs == 0 { old } else { self.rhs };
        match self.operator {
            '*' => lhs * rhs,
            '+' => lhs + rhs,
            _ => panic!("Unkown operator: {:?}", self.operator),
        }
    }
}

#[derive(Debug)]
struct Test {
    diviser: usize,
    on_true: usize,
    on_false: usize,
}

impl From<Lines<'_>> for Test {
    fn from(str: Lines<'_>) -> Self {
        let mut iter = str;
        Self {
            diviser: iter.next().unwrap()[21..].parse().unwrap(),
            on_true: iter.next().unwrap()[29..].parse().unwrap(),
            on_false: iter.next().unwrap()[30..].parse().unwrap(),
        }
    }
}

impl Test {
    fn execute(&self, value: usize) -> usize {
        if value % self.diviser == 0 {
            self.on_true
        } else {
            self.on_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    counted: usize,
}

impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut iter = str.lines().into_iter();
        iter.next().unwrap();
        let items = iter.next().unwrap()[18..]
            .split(", ")
            .map(|part| part.parse().unwrap())
            .collect();
        let operation = iter.next().unwrap().into();
        let test = iter.into();
        Ok(Monkey {
            items,
            operation,
            test,
            counted: 0,
        })
    }
}

impl Monkey {
    fn calculate_worry_level(&self, item: usize) -> usize {
        self.operation.execute(item)
    }

    fn get_next_monkey(&self, item: usize) -> usize {
        self.test.execute(item)
    }

    fn diviser(&self) -> usize {
        self.test.diviser
    }
}

pub fn run() {
    println!(
        "Day 11\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Monkey> {
    reader::open("files/day11.txt").split_on_empty_line_into()
}

fn part_one(monkeys: Vec<Monkey>) -> usize {
    let mut monkeys = monkeys;
    for _ in 0..20 {
        play_round(&mut monkeys, |worry| worry / 3);
    }
    monkey_business_level(monkeys)
}

fn part_two(monkeys: Vec<Monkey>) -> usize {
    let mut monkeys = monkeys;
    let modulus: usize = monkeys.iter().map(|monkey| monkey.diviser()).product();
    for _ in 0..10_000 {
        play_round(&mut monkeys, |worry| worry % modulus);
    }
    monkey_business_level(monkeys)
}

fn play_round(monkeys: &mut Vec<Monkey>, relax_method: impl Fn(usize) -> usize) {
    for i in 0..monkeys.len() {
        for item in monkeys[i].items.to_owned() {
            monkeys[i].counted += 1;
            let worry_level = relax_method(monkeys[i].calculate_worry_level(item));
            let next_monkey = monkeys[i].get_next_monkey(worry_level);
            monkeys[i].items.pop();
            monkeys[next_monkey].items.push(worry_level);
        }
    }
}

fn monkey_business_level(monkeys: Vec<Monkey>) -> usize {
    let mut monkeys = monkeys;
    monkeys.sort_by(|a, b| b.counted.cmp(&a.counted));
    monkeys
        .into_iter()
        .map(|monkey| monkey.counted)
        .take(2)
        .product()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 10_605);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 2_713_310_158);
}

#[cfg(test)]
fn get_test_input() -> Vec<Monkey> {
    reader::open("files/day11_test.txt").split_on_empty_line_into()
}
