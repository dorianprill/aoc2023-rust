use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut sumpossible: usize = 0;
    let mut min_power: u64 = 0;
    let mut sum_powers: u64 = 0;

    // Get the current executable path
    let mut exe_path = env::current_exe().expect("Failed to get the executable path");
    // remove executable name and go up two directories
    for _ in 0..3 {
        exe_path.pop();
    }
    // Append the relative path to target directory
    exe_path.push("input.txt");

    println!("Input file: {:?}", exe_path);

    // read file line by line
    let file = File::open(exe_path).expect("file not found");
    let reader = BufReader::new(file);

    // iterate over lines
    for line in reader.lines() {
        let mut still_possible = true;
        let line = line?;
        let game_id = get_game_id(line.as_str());
        let processed_chunks = process_line(&line);
        let results = process_chunks(processed_chunks);

        println!("Game ID: {:?}", game_id);
        for &(red, green, blue) in &results {
            println!("Red: {}, Green: {}, Blue: {}", red, green, blue);
            if red > max_red || green > max_green || blue > max_blue {
                still_possible = false;
            }
        }
        if still_possible {
            sumpossible += game_id.unwrap() as usize;
            println!("-> possible");
        } else {
            println!("-> not possible");
        }

        // part two
        // add up the number of red, green, and blue in one game
        let (max_red, max_green, max_blue) = results.iter().fold(
            (0, 0, 0),
            |(max_red, max_green, max_blue), &(red, green, blue)| {
                (max_red.max(red), max_green.max(green), max_blue.max(blue))
            },
        );
        min_power = max_red as u64 * max_green as u64 * max_blue as u64;
        println!("Min power: {}", min_power);
        sum_powers += min_power;
    }

    println!("Sum of possible game IDs: {}", sumpossible);
    println!("Sum of powers: {}", sum_powers);

    Ok(())
}

fn get_game_id(line: &str) -> Option<i32> {
    line.split_once(':')
        .and_then(|(left, _)| left.split_whitespace().nth(1)?.parse::<i32>().ok())
}

fn process_line(line: &str) -> Vec<&str> {
    // Split the line at the colon and keep the right side
    let right_side = line.split_once(':').map_or("", |(_, right)| right.trim());

    // Split the right side into chunks separated by ';'
    let chunks = right_side.split(';').map(|chunk| chunk.trim()).collect();

    chunks
}

fn process_chunks(chunks: Vec<&str>) -> Vec<(i32, i32, i32)> {
    let mut results = Vec::new();

    for chunk in chunks {
        let parts = chunk.split(',').collect::<Vec<&str>>();
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for part in parts {
            let color_parts = part.trim().split_whitespace().collect::<Vec<&str>>();
            if color_parts.len() == 2 {
                let count: i32 = color_parts[0].parse().unwrap_or(0);
                match color_parts[1] {
                    "red" => red_count = count,
                    "green" => green_count = count,
                    "blue" => blue_count = count,
                    _ => {}
                }
            }
        }

        results.push((red_count, green_count, blue_count));
    }

    results
}
