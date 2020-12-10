use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug)]
struct Memoise {
    map: HashMap<u64, u64>
}

impl Memoise {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn find(&mut self, nums: &Vec<u64>, num: u64) -> u64 {
        if num == 0 {
            return 1;
        }

        match self.map.get(&num) {
            Some(count) => *count,
            None => {
                // We can optimise the hell out of this because the vector is sorted
                // We just need to count the number of ways to get here
                let mut count = 0;

                // 0, 2, 4, 7
                // 0, 1, 2, 4, 7
                //
                // find(2) = 2
                // find(4) = 2
                // find(7) = 2

                for i in nums.iter() {
                    if *i >= num {
                        break;
                    }

                    if num - i > 0 && num - i <= 3 {
                        count += self.find(nums, *i);
                    }
                }

                self.map.insert(num, count);

                count
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // The answer for part 2 is easily in the trillions, so u32 won't cut it!
    let mut nums = BinaryHeap::<u64>::new();

    for line in reader.lines() {
        let line = line?;
        let val = line.parse().unwrap();
        nums.push(val);
    }

    nums.push(0);
    let largest = nums.peek().unwrap() + 3;
    nums.push(largest);

    let nums = nums.into_sorted_vec();

    let mut last = 0;
    let mut one_diff = 0;
    let mut three_diff = 0;

    for i in nums.iter() {
        if i - last == 1 {
            one_diff += 1;
        } else if i - last == 3 {
            three_diff += 1;
        }

        last = *i;
    }

    println!("(1) The answer is {} (1) * {} (3) = {}", one_diff, three_diff, one_diff * three_diff);

    // Part 2:
    // This problem really lends itself to memoisation!
    // If we start from the top...
    // We can just peek ahead and find all the adapters we can connect to, recursively!
    // We just need to memorize a map of starting => count

    let mut memoise = Memoise::new();
    println!("(2) There are {} permutations", memoise.find(&nums, largest));

    Ok(())
}
