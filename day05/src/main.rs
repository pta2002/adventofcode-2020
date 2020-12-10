use std::fs::File;
use std::io::{self, BufReader, BufRead};
use itertools::Itertools;

#[derive(Debug)]
struct SeatNumber {
    row: usize,
    col: usize
}

impl SeatNumber {
    fn new(pos: &str) -> Self {
        let mut row_range = 0..127;
        let mut col_range = 0..7;

        for char in pos.chars() {
            match char {
                'F' => row_range.end -= (row_range.end - row_range.start + 1) / 2,
                'B' => row_range.start += (row_range.end - row_range.start + 1) / 2,

                'L' => col_range.end -= (col_range.end - col_range.start + 1) / 2,
                'R' => col_range.start += (col_range.end - col_range.start + 1) / 2,

                _ => panic!("Unexpected value!")
            }
        }

        Self { row: row_range.start, col: col_range.start }
    }

    fn get_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let seatids_iter = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| SeatNumber::new(&l))
        .map(|s| s.get_id());

    let mut max = 0;
    let mut myseat = None;
    let mut last = None;

    for seatid in seatids_iter.sorted() {
        if seatid > max {
            max = seatid;
        }

        if let Some(last) = last {
            if last == seatid - 2 {
                myseat = Some(seatid - 1);
            }
        }

        last = Some(seatid);
    }

    println!("(1) Highest ID is {}", max);
    println!("(2) My seat ID is {}", myseat.unwrap());

    Ok(())
}
