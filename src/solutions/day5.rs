use std::collections::VecDeque;
use std::fs::read_to_string;
use std::str::Lines;

use regex::Regex;

use crate::solutions::day5::CraneVersion::{Crane9000, Crane9001};

#[derive(PartialEq, Debug)]
enum CraneVersion {
    Crane9000,
    Crane9001,
}

pub fn day5() {
    let string = read_to_string("day5.txt").unwrap();
    let lines: Lines = string.lines();

    println!("day5 part 1: {}", move_crane(lines.clone(), &Crane9000));
    println!("day5 part 2: {}", move_crane(lines, &Crane9001));
}

fn move_crane(lines: Lines, crane_version: &CraneVersion) -> String {
    let mut stacks: Vec<VecDeque<char>> = vec![];

    for line in lines {
        if line.starts_with(" 1") {
            continue;
        } else if line.starts_with("move") {
            do_move(line, &mut stacks, crane_version);
        } else {
            do_stack_creation(line, &mut stacks);
        }
    }

    let front_elems: String = stacks.iter_mut().map(|s| s.pop_front().unwrap()).collect();
    front_elems
}

fn do_move(line: &str, stacks: &mut [VecDeque<char>], crane_version: &CraneVersion) {
    let re = Regex::new(
        r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let count: usize = caps["count"].parse().unwrap();
    let from: usize = caps["from"].parse().unwrap();
    let to: usize = caps["to"].parse().unwrap();
    match crane_version
    {
        Crane9000 => {
            for _i in 0..count {
                let from_stack = stacks.get_mut(from - 1).unwrap();
                let top = from_stack.pop_front().expect("Could not pop");

                let to_stack = stacks.get_mut(to - 1).unwrap();
                to_stack.push_front(top);
            }
        }
        Crane9001 => {
            let mut top: VecDeque<char> = VecDeque::new();
            for _i in 0..count {
                let from_stack = stacks.get_mut(from - 1).unwrap();
                top.push_front(from_stack.pop_front().expect("Could not pop"));
            }
            let to_stack = stacks.get_mut(to - 1).unwrap();
            for i in 0..count {
                to_stack.push_front(*top.get_mut(i).unwrap());
            }
        }
    }
}

fn do_stack_creation(line: &str, stacks: &mut Vec<VecDeque<char>>) {
    let vec = line.chars().collect::<Vec<char>>();
    let chunks = vec.chunks(4);
    for (index, x) in chunks.enumerate() {
        if stacks.get(index).is_none() {
            let stack = VecDeque::new();
            stacks.push(stack);
        }
        if x.starts_with(&['[']) {
            let stack = stacks.get_mut(index).unwrap();
            stack.push_back(*x.get(1).unwrap());
        }
    }
}
