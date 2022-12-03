extern crate core;

use crate::solutions::day1::day1;
use crate::solutions::day2::day2;
use crate::solutions::day3::day3;
use std::env;

mod solutions;

fn main() {
    let days: Vec<fn()> = vec![day1, day2, day3];

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            days.last().unwrap()(); // default to the last day
        }
        2 => {
            let day = &args[1];
            match &day[..] {
                "all" => {
                    for day in days {
                        day();
                    }
                }
                day_num => {
                    if day_num.starts_with("day") {
                        let (_, num) = day_num.split_at("day".len());
                        let day_index: usize = num
                            .parse::<usize>()
                            .expect("Needs to be day followed by number, e.g. day3.")
                            - 1;
                        let day_fn = days
                            .get(day_index)
                            .expect(&format!("{} not handled!", day_num));
                        day_fn()
                    }
                }
            }
        }
        _ => {
            eprintln!("Too many command line arguments");
        }
    }
}
