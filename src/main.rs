extern crate core;

use crate::solutions::day1::day1;
use crate::solutions::day2::day2;
use std::env;

mod solutions;

fn main() {
    let days: Vec<fn()> = vec![day1, day2];

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            for day in days {
                day();
            }
        }
        2 => {
            let day = &args[1];
            match &day[..] {
                "day1" => day1(),
                "day2" => day2(),
                _ => {}
            }
        }
        _ => {
            eprintln!("Too many command line arguments");
        }
    }
}
