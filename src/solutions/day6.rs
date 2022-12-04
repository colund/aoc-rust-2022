use std::collections::HashSet;
use std::fs::read_to_string;
use std::process::exit;
use std::str::Chars;

pub fn day6() {
    let string = read_to_string("day6.txt").unwrap();
    let line = string.lines().into_iter().next().unwrap();
    let part_1 = day_6(line, 4);
    println!("day6 part 1: {}", part_1);
    let part_1 = day_6(line, 14);
    println!("day6 part 1: {}", part_1);
}

fn day_6(line: &str, unique_count: usize) -> usize {
    let mut pos = 0;

    let chars: Vec<char> = line.chars().collect();
    let char_lists = chars.windows(unique_count);
    for chars in char_lists {
        let set: HashSet<char> =  HashSet::from_iter(chars.iter().cloned());
        if set.len() == unique_count {
            break;
        }
        pos += 1;
    }
    return pos + unique_count;
}
