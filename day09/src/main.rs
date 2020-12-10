use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::vec::Vec;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut nums: Vec<usize> = vec![];

    for line in reader.lines() {
        let line = line?;
        let val = line.parse().unwrap();
        nums.push(val);
    }

    let mut answer1 = 0;
    let mut answer2 = 0;

    for i in 25..nums.len() {
        let mut found = false;
        for j in i-25..i {
            for k in j+1..i {
                if nums[j] + nums[k] == nums[i] {
                    found = true;
                }
            }
        }

        if !found {
            answer1 = nums[i];
            break;
        }
    }

    for i in 0..nums.len() {
        let mut sum = 0;

        let mut min = None;
        let mut max = None;

        let mut j = i;
        while sum < answer1 {
            sum += nums[j];

            match min {
                None => min = Some(nums[j]),
                Some(k) => if nums[j] < k { min = Some(nums[j]) }
            }

            match max {
                None => max = Some(nums[j]),
                Some(k) => if nums[j] > k { max = Some(nums[j]) }
            }

            if sum >= answer1 {
                break;
            }

            j += 1;
        }

        if sum == answer1 {
            answer2 = max.unwrap() + min.unwrap();
            break;
        }
    }

    println!("(1) Answer is {}", answer1);
    println!("(2) XMAS weakness is {}", answer2);

    Ok(())
}
