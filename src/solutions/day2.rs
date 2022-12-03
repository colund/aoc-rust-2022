use crate::solutions::day2::RPS::{Paper, Rock, Scissors};
use std::fs::read_to_string;
use std::str::Lines;

#[derive(PartialEq, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn as_value(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
    fn to_winning_losing_drawn(&self, them: &RPS) -> &RPS {
        // for simplicity, matching Rock, Paper, Scissors instead of X, Y, Z
        match self {
            Rock => {
                // Need to lose
                match them {
                    Rock => &Scissors,
                    Paper => &Rock,
                    Scissors => &Paper,
                }
            }

            Paper => {
                // Need to draw
                match them {
                    Rock => &Rock,
                    Paper => &Paper,
                    Scissors => &Scissors,
                }
            }

            Scissors => {
                // Need to win
                match them {
                    Rock => &Paper,
                    Paper => &Scissors,
                    Scissors => &Rock,
                }
            }
        }
    }
}

fn score(me: &RPS, them: &RPS) -> i32 {
    if me == them {
        3 // draw
    } else if (me == &Rock && them == &Scissors)
        || (me == &Paper && them == &Rock)
        || (me == &Scissors && them == &Paper)
    {
        6 // win
    } else {
        0 // lose
    }
}

fn string_to_rps(abc: &str) -> Option<RPS> {
    return match abc {
        "A" | "X" => Some(Rock),
        "B" | "Y" => Some(Paper),
        "C" | "Z" => Some(Scissors),
        &_ => None,
    };
}

pub fn day2() {
    let string = read_to_string("day2.txt").unwrap();

    let them_me_vec = get_them_me_rps_vec(string);

    println!("day2 part 1: {:?}", part1(&them_me_vec));

    println!("day2 part 2: {:?}", part2(&them_me_vec));
}

fn part2(them_me_vec: &Vec<(RPS, RPS)>) -> i32 {
    them_me_vec
        .iter()
        .map(|(them, me)| {
            let me2 = me.to_winning_losing_drawn(&them);
            score(me2, &them) + me2.as_value()
        })
        .sum()
}

fn part1(them_me_vec: &Vec<(RPS, RPS)>) -> i32 {
    let part1: i32 = them_me_vec
        .iter()
        .map(|(them, me)| score(&me, &them) + me.as_value())
        .sum();
    part1
}

//  Convert string pairs into rock, paper, scissor pairs
fn get_them_me_rps_vec(string: String) -> Vec<(RPS, RPS)> {
    // First split the the string into lines
    let lines: Lines = string.lines();

    // Convert {A, B, C}  {X, Y, Z} pairs into a vector of Rock Paper Scissor tuples (pairs)
    lines
        .map(|line| {
            let (abc, xyz) = line.split_once(" ").unwrap();
            (string_to_rps(abc).unwrap(), string_to_rps(xyz).unwrap())
        })
        .collect()
}
