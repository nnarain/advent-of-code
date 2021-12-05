//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 01 2021
//
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

mod utils;

use clap::Parser;

use std::str::FromStr;

pub enum Part {
    PartOne, PartTwo,
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "part1" => Ok(Part::PartOne),
            "part2" => Ok(Part::PartTwo),
            _ => Err("no match"),
        }
    }
}

#[derive(Parser)]
pub struct Options {
    part: Part,
    input: String,
}
