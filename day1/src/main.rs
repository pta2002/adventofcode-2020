use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::vec::Vec;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut nums: Vec<i32> = vec![];

    for line in reader.lines() {
        let line = line?;
        let val = line.parse().unwrap();
        nums.push(val);
    }

    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            let x = nums[i];
            let y = nums[j];
            if x + y == 2020 {
                println!("Found! (1) {} + {} = 2020, {} * {} = {}", x, y, x, y, x * y);
            }

            for k in j+1..nums.len() {
                let z = nums[k];
                if x + y + z == 2020 {
                    println!("Found! (2) {} + {} + {} = 2020, {} * {} * {} = {}", x, y, z, x, y, z, x * y * z);
                }
            }
        }
    }

    Ok(())
}
