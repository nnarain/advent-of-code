//
// day1.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 01 2021
//

use std::path::PathBuf;
use std::error::Error;

use crate::{Options, Part};

pub fn run(opts: Options) -> Result<(), Box<dyn Error>> {
    // Input file path
    let path = PathBuf::from(opts.input);
    // Process the data from the file
    let data = process_input(path)?;

    match opts.part {
        Part::PartOne => part_one(data),
        Part::PartTwo => part_two(data),
    }

    Ok(())
}

fn part_one(data: Vec<i32>) {
    // An iterator representing the 'current' value
    let current = data.iter();
    // An iterator representing the 'next' value
    let next = data.iter().skip(1);

    // Paired iterator with the current value and the next value
    let paired = current.zip(next);

    // Compare the current value to the next value and count the times the value increased
    let count = paired.map(|(a, b)| b > a).filter(|r| *r).count();

    println!("Increased count: {}", count);
}

fn part_two(data: Vec<i32>) {
    let data: Vec<i32> = data
                            .windows(3)
                            .map(|window| window.iter().sum::<i32>())
                            .collect();
    part_one(data);
}

fn process_input(path: PathBuf) -> Result<Vec<i32>, Box<dyn Error>> {
    // Get an iterator over the input file lines
    let lines = crate::utils::file_line_iter(path)?;
    // Convert lines to strings
    let lines: Vec<String> = lines.filter_map(|line| line.ok()).collect();
    // Convert strings to ints
    let data: Vec<i32> = lines.iter().filter_map(|s| s.parse::<i32>().ok()).collect();

    Ok(data)
}
