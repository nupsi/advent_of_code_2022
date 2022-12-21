use crate::reader;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
enum Job {
    Number(isize),
    Operator(String, String, String),
}

impl From<&str> for Job {
    fn from(value: &str) -> Self {
        if let Ok(value) = value.parse() {
            Job::Number(value)
        } else {
            let mut iter = value.split(' ');
            Job::Operator(
                iter.next().unwrap().to_string(),
                iter.next().unwrap().to_string(),
                iter.next().unwrap().to_string(),
            )
        }
    }
}

impl Job {
    fn get_children(&self) -> Option<(String, String)> {
        match self {
            Job::Operator(l, _, r) => Some((l.to_string(), r.to_string())),
            Job::Number(_) => None,
        }
    }

    fn op(&self) -> &str {
        if let Self::Operator(_, op, _) = self {
            &op
        } else {
            panic!("Unable to find job operator!")
        }
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job,
}

impl Monkey {
    fn eval(&self, lookup: &HashMap<String, Monkey>, values: &mut HashMap<String, isize>) -> isize {
        match &self.job {
            Job::Operator(lhs, op, rhs) => {
                let lhs = lookup.get(lhs).unwrap().eval(lookup, values);
                let rhs = lookup.get(rhs).unwrap().eval(lookup, values);
                let result = match op.as_str() {
                    "+" => lhs + rhs,
                    "*" => lhs * rhs,
                    "-" => lhs - rhs,
                    "/" => lhs / rhs,
                    _ => panic!("Unkown operator!"),
                };
                values.insert(self.name.to_string(), result);
                result
            }
            Job::Number(n) => {
                values.insert(self.name.to_string(), *n);
                *n
            }
        }
    }

    fn get_path(
        &self,
        lookup: &HashMap<String, Monkey>,
        mut path: Vec<String>,
    ) -> Option<Vec<String>> {
        path.push(self.name.to_string());
        if self.name == "humn" {
            return Some(path);
        }

        match &self.job {
            Job::Operator(lhs, _, rhs) => {
                if let Some(l) = lookup.get(lhs).unwrap().get_path(lookup, path.to_vec()) {
                    Some(l)
                } else if let Some(r) = lookup.get(rhs).unwrap().get_path(lookup, path.to_vec()) {
                    Some(r)
                } else {
                    None
                }
            }
            Job::Number(_) => None,
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (left, right) = str.split_once(": ").unwrap();
        Ok(Self {
            name: left.to_string(),
            job: right.into(),
        })
    }
}

pub fn run() {
    println!(
        "Day 21\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<Monkey> {
    reader::open("files/day21.txt").lines_as()
}

fn part_one(values: Vec<Monkey>) -> isize {
    let lookup = values
        .into_iter()
        .map(|monkey| (monkey.name.to_string(), monkey))
        .collect::<HashMap<String, Monkey>>();

    lookup
        .get("root")
        .unwrap()
        .eval(&lookup, &mut HashMap::new())
}

fn part_two(values: Vec<Monkey>) -> isize {
    let lookup = values
        .into_iter()
        .map(|monkey| (monkey.name.to_string(), monkey))
        .collect::<HashMap<String, Monkey>>();

    let path_to_humn = lookup
        .get("root")
        .unwrap()
        .get_path(&lookup, Vec::new())
        .unwrap();

    let mut values = HashMap::new();
    lookup.get("root").unwrap().eval(&lookup, &mut values);
    resolve("root".to_string(), 0, &path_to_humn, &lookup, &values) as isize
}

fn resolve(
    current: String,
    previous_value: isize,
    path: &Vec<String>,
    lookup: &HashMap<String, Monkey>,
    values: &HashMap<String, isize>,
) -> isize {
    let job = &lookup.get(&current).unwrap().job;
    if let Some((left, right)) = job.get_children() {
        let (lhs, rhs) = (*values.get(&left).unwrap(), *values.get(&right).unwrap());
        let (next, value, magic) = if path.contains(&left) {
            (left, rhs, 0)
        } else {
            (right, lhs, 1)
        };

        if current == "root" {
            resolve(next, value, path, lookup, values)
        } else {
            let result = eval(previous_value, inverse(job.op()), value, magic);
            resolve(next, result, path, lookup, values)
        }
    } else {
        previous_value
    }
}

fn inverse(str: &str) -> &str {
    match str {
        "+" => "-",
        "-" => "+",
        "*" => "/",
        "/" => "*",
        _ => panic!("Unkown operator!"),
    }
}

fn eval(lhs: isize, op: &str, rhs: isize, magic: usize) -> isize {
    match (op, magic) {
        ("+", 1) => rhs - lhs,
        ("+", _) => rhs + lhs,
        ("-", _) => lhs - rhs,
        ("*", _) => lhs * rhs,
        ("/", _) => lhs / rhs,
        _ => panic!("Unkown operator!"),
    }
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 152);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 301);
}

#[cfg(test)]
fn get_test_input() -> Vec<Monkey> {
    reader::open("files/day21_test.txt").lines_as()
}
