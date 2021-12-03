//
// day2.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 02 2021
//

// https://adventofcode.com/2021/day/2

use crate::{Options, Part, utils};

use std::path::PathBuf;
use std::str::FromStr;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
enum Step {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, Clone, Copy)]
enum StepError {
    InvalidFormat,
    InvalidCommand,
    ParseError,
}

impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl Error for StepError {}

impl FromStr for Step {
    type Err = StepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split into parts (command and number of steps)
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            Err(StepError::InvalidFormat)
        }
        else {
            // Part the count
            let count = parts[1].parse::<i32>().map_err(|_| StepError::ParseError)?;

            // Match the command type
            match parts[0] {
                "forward" => Ok(Step::Forward(count)),
                "down" => Ok(Step::Down(count)),
                "up" => Ok(Step::Up(count)),
                _ => Err(StepError::InvalidCommand),
            }
        }
    }
}

pub fn run(opts: Options) -> Result<(), Box<dyn Error>> {
    // Path to into data
    let path = PathBuf::from(opts.input);
    // Get the step commands
    let steps = process_input(path)?;

    match opts.part {
        Part::PartOne => part_one(steps),
        Part::PartTwo => part_two(steps),
    }

    Ok(())
}

fn part_one(steps: Vec<Step>) {
    // Process each step, adding to the current position
    // Horizontal position + Depth
    let pos = (0, 0);
    let pos = steps.into_iter().fold(pos, |pos, step| {
        let (x, y) = match step {
            Step::Forward(count) => (count, 0),
            Step::Up(count) => (0, -count),
            Step::Down(count) => (0, count),
        };

        (pos.0 + x, pos.1 + y)
    });

    print!("Position: {:?}, Multiply: {}", pos, pos.0 * pos.1);
}

fn part_two(steps: Vec<Step>) {
    // Horizontal position + Depth + Aim
    let pos_aim = (0, 0, 0);
    let pos_aim = steps.into_iter().fold(pos_aim, |pos_aim, step| {
        let (x, depth, aim) = match step {
            Step::Forward(count) => (count, pos_aim.2 * count, 0),
            Step::Up(count) => (0, 0, -count),
            Step::Down(count) => (0, 0, count)
        };

        (pos_aim.0 + x, pos_aim.1 + depth, pos_aim.2 + aim)
    });

    println!("Position/Aim: {:?}, Multiplied: {}", pos_aim, pos_aim.0 * pos_aim.1);
}

fn process_input(path: PathBuf) -> Result<Vec<Step>, Box<dyn Error>> {
    // File to strings
    let lines = utils::file_to_lines(path)?;
    // strings to step commands
    lines.into_iter().map(|s| Step::from_str(&s).map_err(|e| e.into())).collect()
}
