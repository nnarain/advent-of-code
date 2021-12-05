//
// day3.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 03 2021
//

use crate::{Options, Part};

use crate::utils;

use std::path::PathBuf;
use std::error::Error;

pub fn run(opts: Options) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(opts.input);
    let data = process_input(path)?;

    match opts.part {
        Part::PartOne => part_one(data.0, data.1),
        Part::PartTwo => part_two(),
    }

    Ok(())
}

fn part_one(data: Vec<u16>, bit_len: usize) {
    let mask = (0..bit_len).fold(0, |mask, bit| mask | (1 << bit));

    let transpose = (0..16).map(|bit|{
        let bits_set = data.iter().filter(|&r| r & (1 << bit) != 0).count();
        let bits_clr = data.len() - bits_set;
        if bits_set > bits_clr { 1 } else { 0 }
    });

    let gamma = transpose
                    .enumerate()
                    .fold(0u16, |gamma, (i, bit)| gamma | (bit << i));
    let epsilon = !gamma & mask;

    println!("Gamma: {}, Epsilon: {}, Multiplied: {}", gamma, epsilon, gamma as i32 * epsilon as i32);
}

fn part_two() {

}

fn process_input(path: PathBuf) -> Result<(Vec<u16>, usize), Box<dyn Error>> {
    let lines = utils::file_to_lines(path)?;
    let bit_len = lines[0].len();
    let data: Vec<u16> = lines
                            .into_iter()
                            .filter_map(|s| u16::from_str_radix(s.as_str(), 2).ok())
                            .collect();
    Ok(
        (data, bit_len)
    )
}
