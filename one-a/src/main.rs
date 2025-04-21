use std::{io};
use std::fs::File;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let path = "data/input.txt";

    let mut result = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<&str> = line.trim().split_whitespace().collect();
        if nums.len() == 2 {
            let a: u32 = nums[0].parse().expect("Invalid number in column 1");
            let b: u32 = nums[1].parse().expect("Invalid number in column 2");

            col1.push(a);
            col2.push(b);
        }
    }

    col1.sort();
    col2.sort();

    for i in 0..col1.len() {
        let diff = col1[i].abs_diff(col2[i]);
        result += diff;
    }

    println!("Result: {result}");

    Ok(())
}
