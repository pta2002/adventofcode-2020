use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Coordinate(i64, i64, i64, i64);

impl Coordinate {
    fn neighbours3d(&self) -> Vec<Coordinate> {
        let mut neighbours = vec![];

        for x in (self.0 - 1)..(self.0 + 2) {
            for y in (self.1 - 1)..(self.1 + 2) {
                for z in (self.2 - 1)..(self.2 + 2) {
                    neighbours.push(Coordinate(x, y, z, 0));
                }
            }
        }

        neighbours
    }

    fn neighbours4d(&self) -> Vec<Coordinate> {
        let mut neighbours = vec![];

        for x in (self.0 - 1)..(self.0 + 2) {
            for y in (self.1 - 1)..(self.1 + 2) {
                for z in (self.2 - 1)..(self.2 + 2) {
                    for w in (self.3 - 1)..(self.3 + 2) {
                        neighbours.push(Coordinate(x, y, z, w));
                    }
                }
            }
        }

        neighbours
    }
}

#[derive(Debug, Clone)]
struct Dimension {
    map: HashSet<Coordinate>,
}

impl Dimension {
    fn new(input: BufReader<File>) -> Self {
        let mut map = HashSet::new();

        for (i, line) in input.lines().enumerate() {
            let line = line.unwrap();

            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    let y = i.try_into().unwrap();
                    let x = j.try_into().unwrap();
                    map.insert(Coordinate(x, y, 0, 0));
                }
            }
        }

        Self { map }
    }

    fn get_state(&self, coord: &Coordinate) -> i64 {
        match self.map.get(coord) {
            None => 0,
            Some(_) => 1,
        }
    }

    fn with_neighbours3d(&self) -> HashSet<Coordinate> {
        let mut ret = HashSet::new();

        for coord in &self.map {
            for neighbour in coord.neighbours3d() {
                ret.insert(neighbour);
            }
        }

        ret
    }

    fn with_neighbours4d(&self) -> HashSet<Coordinate> {
        let mut ret = HashSet::new();

        for coord in &self.map {
            for neighbour in coord.neighbours4d() {
                ret.insert(neighbour);
            }
        }

        ret
    }

    fn step3d(&mut self) {
        let mut ret = HashSet::new();

        for coord in self.with_neighbours3d() {
            let activeneighbours = coord
                .neighbours3d()
                .iter()
                .filter(|&c| *c != coord)
                .map(|c| self.get_state(c))
                .fold(0, |a, s| a + s);

            if self.get_state(&coord) == 0 && activeneighbours == 3
                || self.get_state(&coord) == 1 && (activeneighbours == 2 || activeneighbours == 3)
            {
                ret.insert(coord);
            }
        }

        self.map = ret;
    }

    fn step4d(&mut self) {
        let mut ret = HashSet::new();

        for coord in self.with_neighbours4d() {
            let activeneighbours = coord
                .neighbours4d()
                .iter()
                .filter(|&c| *c != coord)
                .map(|c| self.get_state(c))
                .fold(0, |a, s| a + s);

            if self.get_state(&coord) == 0 && activeneighbours == 3
                || self.get_state(&coord) == 1 && (activeneighbours == 2 || activeneighbours == 3)
            {
                ret.insert(coord);
            }
        }

        self.map = ret;
    }

}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut dimension1 = Dimension::new(reader);
    let mut dimension2 = dimension1.clone();

    for _ in 0..6 {
        dimension1.step3d();
        dimension2.step4d();
    }

    println!("(1) There are {} cubes", dimension1.map.len());
    println!("(2) There are {} cubes", dimension2.map.len());

    Ok(())
}
