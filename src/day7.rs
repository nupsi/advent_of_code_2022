use crate::reader;

trait TreeUtils {
    fn get_child_index(&self, current: usize, child_name: String) -> usize;
    fn update_item_sizes(self, i: usize) -> Self;
    fn get_dirs(self) -> Self;
}

impl TreeUtils for Vec<Node> {
    fn get_child_index(&self, current: usize, child_name: String) -> usize {
        *self[current]
            .children
            .iter()
            .filter(|child| self[**child].name == child_name)
            .next()
            .unwrap()
    }

    fn update_item_sizes(self, i: usize) -> Self {
        let mut result = self;
        if result[i].is_dir && result[i].size == 0 {
            for child in result[i].children.clone().into_iter() {
                result = result.update_item_sizes(child);
                result[i].size += result[child].size;
            }
        }
        result
    }

    fn get_dirs(self) -> Self {
        self.into_iter().filter(|node| node.is_dir).collect()
    }
}

#[derive(Debug)]
struct Node {
    index: usize,
    name: String,
    parent: usize,
    children: Vec<usize>,
    size: usize,
    is_dir: bool,
}

impl From<String> for Node {
    fn from(input: String) -> Self {
        let (left, rigth) = input.split_once(" ").unwrap();
        let is_dir = left == "dir";
        Self {
            index: 0,
            parent: 0,
            name: rigth.to_string(),
            children: Vec::new(),
            size: left.parse().unwrap_or(0),
            is_dir,
        }
    }
}

impl Node {
    fn update(&mut self, index: usize, parent: usize) {
        self.index = index;
        self.parent = parent;
    }
}

#[derive(Debug)]
enum Command {
    None,
    MoveDown(String),
    MoveUp,
}

impl From<String> for Command {
    fn from(input: String) -> Self {
        match input[2..].split_once(" ") {
            Some((_, name)) => match name {
                ".." => Command::MoveUp,
                name => Command::MoveDown(name.to_string()),
            },
            None => Command::None,
        }
    }
}

enum Line {
    Input(Command),
    Output(Node),
}

impl From<String> for Line {
    fn from(input: String) -> Self {
        if input.starts_with("$") {
            Line::Input(input.into())
        } else {
            Line::Output(input.into())
        }
    }
}

pub fn run() {
    println!(
        "Day 7\n\tPart 1: {:?}\n\tPart 2: {:?}",
        part_one(input()),
        part_two(input())
    );
}

fn input() -> Vec<String> {
    reader::open("files/day7.txt").lines()
}

fn part_one(commands: Vec<String>) -> usize {
    parse_commands(commands)
        .into_iter()
        .filter(|dir| dir.size <= 100_000)
        .map(|dir| dir.size)
        .sum()
}

fn part_two(commands: Vec<String>) -> usize {
    let commands = parse_commands(commands);
    let current_space = 7_0000_000 - commands[0].size;
    let required_space = 3_000_0000 - current_space;
    commands
        .into_iter()
        .filter(|dir| dir.size >= required_space)
        .map(|dir| dir.size)
        .reduce(|acc, cur| std::cmp::min(acc, cur))
        .unwrap()
}

fn parse_commands(commands: Vec<String>) -> Vec<Node> {
    let init = (0, vec![Node::from("dir /".to_string())]);
    commands
        .into_iter()
        .skip(1)
        .map(|line| line.into())
        .fold(init, |(index, mut nodes), command| match command {
            Line::Input(command) => match command {
                Command::MoveDown(child_name) => (nodes.get_child_index(index, child_name), nodes),
                Command::MoveUp => (nodes[index].parent, nodes),
                _ => (index, nodes),
            },
            Line::Output(mut child) => {
                child.update(nodes.len(), index);
                nodes[index].children.push(*&child.index);
                nodes.push(child);
                (index, nodes)
            }
        })
        .1
        .update_item_sizes(0)
        .get_dirs()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(get_test_input()), 95437);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(get_test_input()), 24933642);
}

#[cfg(test)]
fn get_test_input() -> Vec<String> {
    reader::open("files/day7_test.txt").lines()
}
