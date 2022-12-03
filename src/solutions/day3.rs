use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::Lines;

pub fn day3() {
    let string = read_to_string("day3.txt").unwrap();
    let lines: Lines = string.lines();
    part1(&lines);
    part2(&lines);
}

fn part2(lines: &Lines) {
    let mut iter = lines.clone().into_iter();
    let mut sum = 0;
    while let (Some(x), Some(y), Some(z)) = (iter.next(), iter.next(), iter.next()) {
        let x_set: HashSet<char> = x.chars().collect();
        let y_set: HashSet<char> = y.chars().collect();
        let z_set: HashSet<char> = z.chars().collect();

        if let Some(found) = x_set
            .iter()
            .find(|p| y_set.contains(p) && z_set.contains(p))
        {
            sum += priority(*found);
        }
    }
    println!("day3 part 2: {}", sum);
}

fn part1(lines: &Lines) {
    let half_lines: Vec<(&str, &str)> = lines
        .clone()
        .into_iter()
        .map(|x| x.split_at(x.len() / 2))
        .collect();
    let jox: Vec<char> = half_lines
        .iter()
        .map(|(x, y)| x.chars().find(|&c| y.contains(c)).unwrap())
        .collect();

    let sum: usize = jox.iter().map(|&x| priority(x)).sum();
    println!("day3 part 1: {}", sum);
}

// Get priority: a-zA-Z characters are mapped to 1-52
fn priority(char: char) -> usize {
    let mut lower_case_range = (b'a'..=b'z').map(char::from);
    let mut upper_case_range = (b'A'..=b'Z').map(char::from);

    let lower_case_pos = lower_case_range.position(|x| x == char);
    if let Some(x) = lower_case_pos {
        return x + 1;
    }

    let upper_case_pos = upper_case_range.position(|y| y == char);
    if let Some(y) = upper_case_pos {
        return y + 27;
    }

    panic!("Unexpected char: {}", char);
}
