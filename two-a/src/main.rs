use std::io;
use std::fs::File;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let path = "data/input.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut safe_reports = 0;
    
    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid number"))
            .collect();
        
        if levels.len() < 2 {
            continue
        }
       
        let first_diff = levels[1] - levels[0];
        if first_diff == 0 || first_diff.abs() > 3 {
            continue;
        }
        
        let increasing = first_diff > 0;
        let mut is_safe = true;
        
        for pair in levels.windows(2) {
            let diff = pair[1] - pair[0];
            
            if diff == 0 || diff.abs() > 3 {
                is_safe = false;
                break;
            }
            if (diff > 0) != increasing {
                is_safe = false;
                break;
            }
        }
        
        if is_safe {
            safe_reports += 1;
        }
    }
    
    println!("{}", safe_reports);
    Ok(())
}
