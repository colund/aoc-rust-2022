use regex::Regex;
use std::fs::read_to_string;
use std::str::Lines;

pub fn day4() {
    let string = read_to_string("day4.txt").unwrap();
    let lines: Lines = string.lines();

    let pairs: Vec<RangePair> = lines.map(|line| to_range_pair(line)).collect();
    part1(&pairs);
    part2(&pairs);
}

fn part1(pairs: &Vec<RangePair>) {
    let result: usize = pairs
        .into_iter()
        .map(|p| (p.min1 <= p.min2 && p.max1 >= p.max2) || (p.min2 <= p.min1 && p.max2 >= p.max1))
        .filter(|p| *p)
        .count();
    println!("Day 4 part 1: {:?}", result);
}

fn part2(pairs: &Vec<RangePair>) {
    let result = pairs
        .into_iter()
        .map(|p| (p.max1 >= p.min2 && p.max1 <= p.max2) || (p.max2 >= p.min1 && p.max2 <= p.max1))
        .filter(|b| *b)
        .count();
    println!("Day 4 part 2: {:?}", result);
}

#[derive(Debug)]
struct RangePair {
    min1: i32,
    max1: i32,
    min2: i32,
    max2: i32,
}

fn to_range_pair(line: &str) -> RangePair {
    let re = Regex::new(
        r"(?x)
(?P<min1>\d+)  # min of range 1
-
(?P<max1>\d+) # max of range 1
,
(?P<min2>\d+)  # min of range 2
-
(?P<max2>\d+) # max of range 2
",
    )
    .unwrap();

    let caps = re.captures(line).unwrap();
    RangePair {
        min1: caps["min1"].parse().ok().unwrap(),
        max1: caps["max1"].parse().ok().unwrap(),
        min2: caps["min2"].parse().ok().unwrap(),
        max2: caps["max2"].parse().ok().unwrap(),
    }
}
