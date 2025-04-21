use std::{io};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let path = "data/input.txt";

    let mut result = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut col1 = Vec::new();
    
    let mut col2_map: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<&str> = line.trim().split_whitespace().collect();
        if nums.len() == 2 {
            let a: i32 = nums[0].parse().expect("Invalid number in column 1");
            let b: i32 = nums[1].parse().expect("Invalid number in column 2");

            col1.push(a);
           
            *col2_map.entry(b).or_insert(0) += 1;
        }
    }
    
    for v in col1 {
        result += v * *col2_map.get(&v).unwrap_or(&0);
    }
    
    println!("{result}");
    
    Ok(())
}
