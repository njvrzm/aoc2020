use std::fs::read_to_string;
// extern crate clap;
use std::env;

mod one;
mod two;
mod four;

use crate::one::day_one;
use crate::two::day_two;
use crate::four::day_four;
#[macro_use] extern crate maplit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = format!("inputs/day_{}.txt", &args[1]);
    let lines = read_to_string(path).expect("Uh oh");
    if args[1] == "one" {
        if args[2] == "one" {
            day_one::run_one(lines);
        } else {
            day_one::run_two(lines);
        }
    } else if args[1] == "two" {
        if args[2] == "one" {
            day_two::run_one(lines);
        } else {
            day_two::run_two(lines);
        }
    } else if args[1] == "four" {
        if args[2] == "one" {
            day_four::run_one(lines);
        }
    }
}
