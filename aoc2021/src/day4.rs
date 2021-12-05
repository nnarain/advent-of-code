//
// day4.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 04 2021
//

use crate::{Options, Part};
use crate::utils;

use std::path::PathBuf;
use std::error::Error;

#[derive(Default, Debug, Clone, Copy)]
struct BoardData {
    value: i32,
    selected: bool,
}

impl BoardData {
    pub fn new(value: i32) -> Self {
        BoardData {
            value,
            selected: false,
        }
    }
}

#[derive(Clone)]
struct Board {
    data: Vec<Vec<BoardData>>
}

impl Board {
    pub fn new(data: Vec<Vec<i32>>) -> Self {
        let data: Vec<Vec<BoardData>> = data
                                            .into_iter()
                                            .map(|v| v.into_iter().map(BoardData::new).collect::<Vec<BoardData>>())
                                            .collect();
        Board {
            data,
        }
    }

    pub fn update(&mut self, value: i32)  {
        for row in self.data.iter_mut() {
            for data in row.iter_mut() {
                if data.value == value {
                    data.selected = true;
                }
            }
        }
    }

    pub fn winner(&self) -> bool {
        // Check rows
        let row_win = self.data.iter().map(|row| row.iter().all(|cell| cell.selected)).any(|r| r);

        let cols = self.data[0].len();
        let col_win = (0..cols).map(|col| self.data.iter().all(|row| row[col].selected)).any(|r| r);

        row_win || col_win
    }

    pub fn sum_unselected(&self) -> i32 {
        self.data
            .iter()
            .map(|row| row.iter().filter(|cell| !cell.selected).fold(0, |sum, cell| sum + cell.value))
            .sum()
    }
}

pub fn run(opts: Options) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(opts.input);
    let data = process_input(path)?;

    match opts.part {
        Part::PartOne => part_one(data.0, data.1),
        Part::PartTwo => {},
    }

    Ok(())
}

fn part_one(values: Vec<i32>, mut boards: Vec<Board>) {
    let mut last_called = 0;

    'outer: for input in values.iter() {
        for board in boards.iter_mut() {
            board.update(*input);

            if board.winner() {
                last_called = *input;
                break 'outer;
            }
        }
    }

    let (winners, _losers): (Vec<Board>, Vec<Board>) = boards.into_iter().partition(|board| board.winner());

    for winner in winners {
        let unselected_sum = winner.sum_unselected();
        let score = unselected_sum * last_called;
        println!("Unselected: {}, Last: {}, Score: {}", unselected_sum, last_called, score);
    }
}

fn process_input(path: PathBuf) -> Result<(Vec<i32>, Vec<Board>), Box<dyn Error>> {
    let lines = utils::file_to_lines(path)?;

    // Selected numbers
    let selected_numbers: Vec<i32> = lines[0].split(',').filter_map(|s| s.parse::<i32>().ok()).collect();

    // Get the lines representing the board
    let boards = lines
                    .into_iter()
                    .skip(1)
                    .filter(|line| line.len() > 1)
                    .collect::<Vec<String>>();
    // 
    let boards = boards
                    .chunks(5)
                    .map(|lines| {
                        let values = lines
                                        .iter()
                                        .map(|line| {
                                            line.split(' ').filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>()
                                        })
                                        .collect::<Vec<Vec<i32>>>();

                        Board::new(values)
                    })
                    .collect::<Vec<Board>>();

    Ok((selected_numbers, boards))
}
