use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::vec::Vec;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug, Copy, Clone)]
enum Position {
    Floor,
    Seat(bool)
}

#[derive(Clone)]
struct Seats {
    seats: Vec<Position>,
    width: isize,
    height: isize
}

impl fmt::Debug for Seats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            write!(f, "\n")?;

            for x in 0..self.width {
                write!(f, "{}", match self.get_at(x, y).unwrap() {
                    Position::Seat(true) => "#",
                    Position::Seat(false) => "L",
                    Position::Floor => "."
                })?;
            }
        }

        Ok(())
    }
}

impl Seats {
    fn new() -> Self {
        Self {
            seats: Vec::new(),
            width: 0,
            height: 0
        }
    }

    fn push_line(&mut self, line: &str) {
        let len: isize = line.len().try_into().unwrap();
        if len > self.width  {
            self.width = len;
        }

        self.height += 1;

        for char in line.chars() {
            self.seats.push(match char {
                '.' => Position::Floor,
                '#' => Position::Seat(true),
                'L' => Position::Seat(false),
                _ => panic!("Impossible seat")
            })
        }
    }

    fn get_at(&self, x: isize, y: isize) -> Option<Position> {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            None
        } else {
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            let w: usize = self.width.try_into().unwrap();
            Some(self.seats[x + y * w])
        }
    }

    fn get_occupied(&self, x: isize, y: isize) -> isize {
        if let Some(Position::Seat(true)) = self.get_at(x, y) {
            return 1;
        } else {
            return 0;
        }
    }

    fn get_adjacent(&self, x: isize, y: isize) -> isize {
        let mut count = 0;

        for i in y-1..y+2 {
            for j in x-1..x+2 {
                if j != x || i != y {
                    count += self.get_occupied(j, i);
                }
            }
        }
        count
    }

    // returns true if it's changed
    fn step(&mut self) -> bool {
        let mut new_seats = self.seats.clone();
        let mut changed = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let w: usize = self.width.try_into().unwrap();
                let ux: usize = x.try_into().unwrap();
                let uy: usize = y.try_into().unwrap();
                match self.get_at(x, y).unwrap() {
                    Position::Seat(true) => {
                        if self.get_adjacent(x, y) >= 4 {
                            new_seats[ux + uy * w] = Position::Seat(false);
                            changed = true;
                        }
                    },
                    Position::Seat(false) => {
                        if self.get_adjacent(x, y) == 0 {
                            new_seats[ux + uy * w] = Position::Seat(true);
                            changed = true;
                        }
                    },
                    _ => {}
                }
            }
        }

        self.seats = new_seats;

        changed
    }

    fn check_clear(&self, mut x: isize, mut y: isize, incx: isize, incy: isize) -> isize {
        loop {
            x += incx;
            y += incy;

            match self.get_at(x, y) {
                Some(Position::Floor) => continue,
                Some(Position::Seat(true)) => return 1,
                _ => return 0
            }
        }
    }

    fn get_adjacent2(&self, x: isize, y: isize) -> isize {
        let mut count = 0;

        count += self.check_clear(x, y, -1, -1);
        count += self.check_clear(x, y, 1, 1);
        count += self.check_clear(x, y, -1, 1);
        count += self.check_clear(x, y, 1, -1);
        count += self.check_clear(x, y, 1, 0);
        count += self.check_clear(x, y, -1, 0);
        count += self.check_clear(x, y, 0, 1);
        count += self.check_clear(x, y, 0, -1);

        count
    }

    fn step2(&mut self) -> bool {
        let mut new_seats = self.seats.clone();
        let mut changed = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let w: usize = self.width.try_into().unwrap();
                let ux: usize = x.try_into().unwrap();
                let uy: usize = y.try_into().unwrap();
                match self.get_at(x, y).unwrap() {
                    Position::Seat(true) => {
                        if self.get_adjacent2(x, y) >= 5 {
                            new_seats[ux + uy * w] = Position::Seat(false);
                            changed = true;
                        }
                    },
                    Position::Seat(false) => {
                        if self.get_adjacent2(x, y) == 0 {
                            new_seats[ux + uy * w] = Position::Seat(true);
                            changed = true;
                        }
                    },
                    _ => {}
                }
            }
        }

        self.seats = new_seats;

        changed
    }

    fn count_occupied(&self) -> usize {
        self.seats.iter()
            .filter(|seat| match seat {
                Position::Seat(true) => true,
                _ => false
            })
            .count()
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut seats = Seats::new();

    for line in reader.lines() {
        seats.push_line(&line?);
    }

    let mut seats2 = seats.clone();

    while seats.step() {}
    println!("(1) There are {} occupied seats", seats.count_occupied());

    while seats2.step2() {}
    println!("(2) There are {} occupied seats", seats2.count_occupied());

    Ok(())
}
