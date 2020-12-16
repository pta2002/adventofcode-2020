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
            self.last_spoken.insert(self.numbers[self.iteration], self.iteration);
            self.prev = Some(self.numbers[self.iteration]);
            self.iteration += 1;
            self.numbers[self.iteration - 1]
        } else {
            // TODO rearchitect this
            let n = match self.last_spoken.get(&self.prev.unwrap()) {
                None => {
                    self.prev = Some(0);
                    0
                },
                Some(&i) => {
                    self.prev = Some(i);
                    self.iteration - i
                }
            };

            self.iteration += 1;
            self.last_spoken.insert(n, self.iteration);
            n
        }
    }
}

fn main() -> Result<(), io::Error> {
    // let file = File::open("input.txt")?;
    // let reader = BufReader::new(file);

    // let mut nums: Vec<i32> = vec![];

    // for line in reader.lines() {
    //     let line = line?;
    //     let val = line.parse().unwrap();
    //     nums.push(val);
    // }

    let mut state = State::new(vec![0, 3, 6]);

    for _ in 0..10 {
        dbg!(state.step());
    }

    println!("(1) The 2020th number is {}", state.step());

    Ok(())
}
