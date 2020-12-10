use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

struct GroupAnswers(HashSet<char>);

impl GroupAnswers {
    fn new() -> Self {
        GroupAnswers(HashSet::new())
    }

    fn new_from_str(l: &str) -> Self {
        let mut hs = Self::new();
        hs.add_str(l);
        hs
    }


    fn add_str(&mut self, l: &str) {
        for char in l.chars() {
            self.0.insert(char);
        }
    }

    fn intersect_str(&self, l: &str) -> Self {
        let mut hs = Self::new();
        for char in l.chars() {
            if self.0.contains(&char) {
                hs.0.insert(char);
            }
        }

        hs
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let (answer1, answer2) = reader
        .lines()
        .filter_map(|l| l.ok())
        .fold(vec![(GroupAnswers::new(), None::<GroupAnswers>)],
            |mut acc, l| {
                // f u n c t i o n a l
                match l.as_str() {
                    "" => acc.push((GroupAnswers::new(), None)),
                    l => {
                        let index = acc.len() - 1;
                        let b = match &acc[index].1 {
                            None => GroupAnswers::new_from_str(l),
                            Some(v) => v.intersect_str(l)
                        };

                        acc[index].0.add_str(l);
                        acc[index].1 = Some(b);
                    }
                }

                acc
            })
        .into_iter()
        .map(|(a, b)| (a.0.len(), b.map_or(0, |b| b.0.len())))
        .fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b));

    println!("(1) Sum of answers is {}", answer1);
    println!("(2) Sum of answers is {}", answer2);

    Ok(())
}
