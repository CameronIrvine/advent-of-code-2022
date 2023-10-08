use std::{env, process::exit};

mod day1;
mod day2;
mod day3;
mod day4;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        exit(0);
    }
    let day = &args[1];
    match day.as_str() {
        "1" => {
            day1::part1();
            day1::part2();
        }
        "2" => {
            day2::part1();
            day2::part2();
        }
        "3" => {
            day3::part1();
            day3::part2();
        }
        "4" => {
            day4::part1();
            day4::part2();
        }
        _ => {
            exit(1);
        }
    }
}
