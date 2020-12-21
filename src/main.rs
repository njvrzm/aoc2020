use std::fs::read_to_string;
// extern crate clap;
use std::env;

mod one;
mod two;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod eleven;
mod twelve;
mod thirteen;
mod fourteen;
mod fifteen;
mod sixteen;
mod seventeen;

use crate::one::day_one;
use crate::two::day_two;
use crate::four::day_four;
use crate::five::day_five;
use crate::six::day_six;
use crate::seven::day_seven;
use crate::eight::day_eight;
use crate::nine::day_nine;
use crate::eleven::day_eleven;
use crate::twelve::day_twelve;
use crate::thirteen::day_thirteen;
use crate::fourteen::day_fourteen;
use crate::fifteen::day_fifteen;
use crate::sixteen::day_sixteen;
use crate::seventeen::day_seventeen;
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
        ["seven", "one"] => day_seven::run_one(lines),
        ["seven", "two"] => day_seven::run_two(lines),
        ["eight", "one"] => day_eight::run_one(lines),
        ["eight", "two"] => day_eight::run_two(lines),
        ["nine", "one"] => day_nine::run_one(lines),
        ["nine", "two"] => day_nine::run_two(lines),
        ["eleven", "one"] => day_eleven::run_one(lines),
        ["eleven", "two"] => day_eleven::run_two(lines),
        ["twelve", "one"] => day_twelve::run_one(lines),
        ["twelve", "two"] => day_twelve::run_two(lines),
        ["thirteen", "one"] => day_thirteen::run_one(lines),
        ["thirteen", "two"] => day_thirteen::run_two(lines),
        ["fourteen", "one"] => day_fourteen::run_one(lines),
        ["fourteen", "two"] => day_fourteen::run_two(lines),
        ["fifteen", "one"] => day_fifteen::run_one(lines),
        ["sixteen", "one"] => day_sixteen::run_one(lines),
        ["sixteen", "two"] => day_sixteen::run_two(lines),
        ["seventeen", "one"] => day_seventeen::run_one(lines),
        _ => (), // error or something
    }
}
