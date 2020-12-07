use std::fs::read_to_string;
// extern crate clap;
use std::env;

mod one;
mod two;
mod four;
mod five;
mod six;

use crate::one::day_one;
use crate::two::day_two;
use crate::four::day_four;
use crate::five::day_five;
use crate::six::day_six;
use itertools::Itertools;

#[macro_use] extern crate maplit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = format!("inputs/day_{}.txt", &args[1]);
    let lines = read_to_string(path).expect("Uh oh");
    match args.iter().map(|s| s.as_str()).collect_vec()[1..]{
        ["one", "one"] => day_one::run_one(lines),
        ["one", "two"] => day_one::run_two(lines),
        ["two", "one"] => day_two::run_one(lines),
        ["two", "two"] => day_two::run_two(lines),
        ["four", "one"] => day_four::run_one(lines),
        ["four", "two"] => day_four::run_two(lines),
        ["five", "one"] => day_five::run_one(lines),
        ["five", "two"] => day_five::run_two(lines),
        ["six", "one"] => day_six::run_one(lines),
        ["six", "two"] => day_six::run_two(lines),
        _ => (), // error or something
    }
}
