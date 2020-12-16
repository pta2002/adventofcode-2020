// use std::fs::File;
// use std::io::{self, BufReader, BufRead};
use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct State {
    iteration: usize,
    last_spoken: HashMap<usize, usize>,
    numbers: Vec<usize>,
    prev: Option<usize>
}

impl State {
    fn new(numbers: Vec<usize>) -> Self {
        Self { numbers, iteration: 0, last_spoken: HashMap::new(), prev: None }
    }

    fn step(&mut self) -> usize {
        if self.iteration < self.numbers.len() {
            if let Some(i) = self.prev {
                self.last_spoken.insert(i, self.iteration);
            }

            self.prev = Some(self.numbers[self.iteration]);
            self.iteration += 1;
            return self.numbers[self.iteration - 1];
        } else {
            match self.last_spoken.insert(self.prev.unwrap(), self.iteration) {
                None => {
                    self.prev = Some(0);
                    self.iteration += 1;
                    return 0;
                },
                Some(i) => {
                    self.prev = Some(self.iteration - i);
                    self.iteration += 1;
                    return self.prev.unwrap();
                }
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Honestly, why bother reading the file, the input is really small
    let mut state = State::new(vec![15, 5, 1, 4, 7, 0]);

    for i in 0..30000000 {
        let n = state.step();
        if i == 2019 {
            println!("(1) the 2020th number is {}", n);
        } else if i == 30000000 - 1 {
            println!("(2) the 30000000th number is {}", n);
        }
    }

    Ok(())
}
