use std::fs::read_to_string;

pub fn day1() {
    let sums = day1sums();

    let day1_1: &i32 = sums.first().unwrap();
    println!("day1 part 1: {}", day1_1);

    let day1_2: i32 = sums.into_iter().take(3).sum();
    println!("day1 part 2: {}", day1_2);
}

fn day1sums() -> Vec<i32> {
    let mut sums: Vec<i32> = vec![];
    let mut curr: i32 = 0;
    let string = read_to_string("day1.txt").unwrap();
    let lines = string.lines();

    for line in lines {
        if line.is_empty() {
            sums.push(curr);
            curr = 0;
        } else {
            let i: i32 = line.parse().unwrap();
            curr = curr + i;
        }
    }
    sums.sort_by(|a, b| b.cmp(a)); // Sort descending

    return sums;
}
