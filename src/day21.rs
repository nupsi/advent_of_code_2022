use crate::reader;
use core::panic;
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

    fn form_expr(
        &self,
        lookup: &HashMap<String, Monkey>,
        values: &HashMap<String, isize>,
        path: &Vec<String>,
    ) -> String {
        if self.name == "humn" {
            return "x".to_string();
        }

        match &self.job {
            Job::Operator(lhs, op, rhs) => {
                let l = if path.contains(lhs) {
                    lookup.get(lhs).unwrap().form_expr(lookup, values, path)
                } else {
                    values.get(lhs).unwrap().to_string()
                };
                let r = if path.contains(rhs) {
                    lookup.get(rhs).unwrap().form_expr(lookup, values, path)
                } else {
                    values.get(rhs).unwrap().to_string()
                };
                match op.as_str() {
                    "+" => format!("({} + {})", l, r),
                    "*" => format!("({} * {})", l, r),
                    "-" => format!("({} - {})", l, r),
                    "/" => format!("({} / {})", l, r),
                    "=" => format!("({} = {})", l, r),
                    _ => panic!("Unkown operator!"),
                }
            }
            Job::Number(n) => n.to_string(),
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

    let path_to_me = lookup
        .get("root")
        .unwrap()
        .get_path(&lookup, Vec::new())
        .unwrap();

    let mut values = HashMap::new();
    lookup.get("root").unwrap().eval(&lookup, &mut values);

    let (lhs, rhs) = lookup.get("root").unwrap().job.get_children().unwrap();
    println!(
        "{} = {}",
        values.get(&"zhfp".to_string()).unwrap(),
        values.get(&"hghd".to_string()).unwrap()
    );

    let (humn_branch, correct_branch) = if path_to_me.contains(&lhs) {
        (lhs, rhs)
    } else {
        (rhs, lhs)
    };

    println!(
        "{} = {}",
        values.get(&correct_branch).unwrap(),
        lookup
            .get(&humn_branch)
            .unwrap()
            .form_expr(&lookup, &values, &path_to_me)
    );
    0
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
