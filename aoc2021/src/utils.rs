//
// utils.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 01 2021
//

use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

/// Returns an iterator
pub fn file_line_iter(path: PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn file_to_lines(path: PathBuf) -> Result<Vec<String>, io::Error> {
    let lines = file_line_iter(path)?;
    Ok(
        lines.filter_map(|line| line.ok()).collect()
    )
}
