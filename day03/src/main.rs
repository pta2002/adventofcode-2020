use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec::Vec;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    map: Vec<bool>,
}

impl Map {
    fn new(reader: BufReader<File>) -> Option<Map> {
        let mut width = 0;
        let mut height = 0;
        let mut vec = vec![];

        for line in reader.lines() {
            let line = line.ok()?;

            width = line.len();
            height += 1;

            for letter in line.chars() {
                match letter {
                    '#' => vec.push(true),
                    '.' => vec.push(false),
                    _ => panic!("Unexpected character!"),
                }
            }
        }

        Some(Map {
            width,
            height,
            map: vec,
        })
    }

    fn get_at(&self, x: usize, y: usize) -> Option<bool> {
        if y < self.height {
            Some(self.map[x % self.width + y * self.width])
        } else {
            None
        }
    }

    fn get_encounters(&self, i: usize, j: usize) -> usize {
        let mut encounters = 0;

        let mut x = i;
        let mut y = j;

        loop {
            match self.get_at(x, y) {
                Some(true) => encounters += 1,
                None => break,
                _ => {}
            }

            x += i;
            y += j;
        }

        encounters
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let map = Map::new(reader).unwrap();

    let encounters11 = map.get_encounters(1, 1);
    let encounters31 = map.get_encounters(3, 1);
    let encounters51 = map.get_encounters(5, 1);
    let encounters71 = map.get_encounters(7, 1);
    let encounters12 = map.get_encounters(1, 2);

    println!("(1) Encountered {} trees.", encounters31);
    println!(
        "(2) Product of encounters: {} trees.",
        encounters11 * encounters31 * encounters51 * encounters71 * encounters12
    );

    Ok(())
}
