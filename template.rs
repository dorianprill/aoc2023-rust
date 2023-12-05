use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let input_file = "sample.txt";

    // Get the current executable path
    let mut exe_path = env::current_exe().expect("Failed to get the executable path");
    // remove executable name and go up two directories
    for _ in 0..3 {
        exe_path.pop();
    }
    // Append the relative path to target directory
    exe_path.push(input_file);

    println!("Input file: {:?}", exe_path);

    // read file line by line
    let file = File::open(exe_path).expect("file not found");
    let reader = BufReader::new(file);

    // iterate over lines
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        // Do stuff with the individual lines
    }

    println!("Print results here");

    Ok(())
}

