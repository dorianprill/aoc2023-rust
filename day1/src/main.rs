use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    let mut calibration_sum = 0;
    let lut = [
        ("zero",0), 
        ("one",1), 
        ("two",2), 
        ("three",3), 
        ("four",4), 
        ("five",5), 
        ("six",6), 
        ("seven",7), 
        ("eight",8), 
        ("nine",9)
    ]; 

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
        let mut min_x = -1;
        let mut min_digit: i32 = -1;
        let mut max_x = -1;
        let mut max_digit = -1;
        let mut two_digit_number = 0;
        let line = line?;
        println!("{}", line);
        // beforehand, scan the line for textual digits contained in lookup table lut
        // set the digit and index of first occurence of digit
        for (x, digit) in lut.iter() {
            if line.contains(x) {
                //let idx_found: Option<(usize, &str)>;
                // is digit text at least contained once?
                let mut indices_found = line.match_indices(x);
                // get first occurence of digit text
                while let Some(found) = indices_found.next() {
                    
                    println!("{} found at index {}", found.1, found.0);
                    // if min_x is not set, set it to first occurence of digit
                    if min_x == -1 {
                        min_x = found.0 as i32;
                        min_digit = *digit;
                    }
                    // if max_x is not set, set it to first occurence of digit
                    if max_x == -1 {
                        max_x = found.0 as i32;
                        max_digit = *digit;
                    }
                    // if min_x is set, check if current index is smaller
                    if min_x > found.0 as i32 {
                        min_x = found.0 as i32;
                        min_digit = *digit;
                    }
                    // if max_x is set, check if current index is larger
                    if max_x < found.0 as i32 {
                        max_x = found.0 as i32;
                        max_digit = *digit;
                    }

                    
                }
                
            }
        }
        // now we do the same for actual single digits
        for (x, char) in line.chars().enumerate() {
            let digit = match char.to_digit(10){
                Some(d) => d,
                None => continue,
            } as i32;
            // if min_x is not set, set it to first occurence of digit
            if min_x == -1 {
                min_x = x as i32;
                min_digit = digit;
            }
            // if max_x is not set, set it to first occurence of digit
            if max_x == -1 {
                max_x = x as i32;
                max_digit = digit;
            }
            // if min_x is set, check if current index is smaller
            if min_x > x as i32 {
                min_x = x as i32;
                min_digit = digit;
            }
            // if max_x is set, check if current index is larger
            if max_x < x as i32 {
                max_x = x as i32;
                max_digit = digit;
            }
        }

        if min_x > -1 && max_x > -1 {
            two_digit_number = min_digit * 10 + max_digit;
            println!("Min: {}, Max: {}, Number: {}", min_x, max_x, two_digit_number);
            calibration_sum += two_digit_number;
        } else {
            println!("No digits found");
            continue;
        }
        
    }

    println!("Calibration Value: {}", calibration_sum);

    Ok(())
}

