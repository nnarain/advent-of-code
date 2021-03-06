//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 01 2021
//

use clap::Parser;
use aoc2021::*;

use std::error::Error;

#[derive(Parser)]
enum Days {
    Day1(aoc2021::Options),
    Day2(aoc2021::Options),
    Day3(aoc2021::Options),
    Day4(aoc2021::Options),
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Natesh Narain")]
struct Options {
    #[clap(subcommand)]
    day: Days,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::parse();

    match opts.day {
        Days::Day1(opts) => day1::run(opts),
        Days::Day2(opts) => day2::run(opts),
        Days::Day3(opts) => day3::run(opts),
        Days::Day4(opts) => day4::run(opts),
    }
}
