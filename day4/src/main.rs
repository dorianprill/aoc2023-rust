use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    let mut total_points: u32 = 0;
    

    // Get the current executable path
    let mut exe_path = env::current_exe().expect("Failed to get the executable path");
    // remove executable name and go up two directories
    for _ in 0..3 {
        exe_path.pop();
    }
    // Append the relative path to target directory
    exe_path.push(input_file);

    println!("Input file: {:?}", exe_path);

    // Open file and save line count
    let mut file = File::open(exe_path).expect("file not found");
    let mut reader = BufReader::new(&file);
    let total_lines = reader.lines().count();
    let mut played = vec![0; total_lines];
    // rewind file and open again to prevent borrowing issues
    file.seek(SeekFrom::Start(0))?;
    reader = BufReader::new(&file);

    // Iterate over lines
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        played[i] += 1;
        //println!("{}", line);
        // Do stuff with the individual lines
        let game = line.split_once(": ").unwrap().1;
        //println!("{}", game);

        let winnums = game.split_once(" | ").unwrap().0;
        let ournums = game.split_once(" | ").unwrap().1;
        //println!("Winners: {} Ours: {}", winnums, ournums);    

        let winset: HashSet<u32> = winnums
            .split_ascii_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();

        let ourset: HashSet<u32> = ournums
            .split_ascii_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();

        println!("Winners: {:?} Ours: {:?}", winset, ourset);

        let matches: Vec<u32> = winset
            .intersection(&ourset)
            .map(|x| *x)
            .collect();

        println!("Matches: {:?}", matches);

        if matches.len() > 0 {
            total_points += u32::pow(2, matches.len() as u32 - 1);
        }

        // part two: play extra game cards depending on the current cards
        // note: should also increase score but apparently part2 only 
        // cares about total number of games played, so we will leave solution part 1 as is
        for w in 0..matches.len() {
            played[i+w+1] += played[i];
        }

        println!("Played this card {} times", played[i]);

    }

    let total_played: u32 = played.iter().sum();
    println!("Total points: {}", total_points);
    println!("Total played: {:?}", total_played);

    Ok(())
}

