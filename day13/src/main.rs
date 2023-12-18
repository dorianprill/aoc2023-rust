// i read so many spoilers for day 13 and found this neat:
// https://advent-of-code.xavd.id/writeups/2023/day/13/
// so i decided to implement it in rust for you all to
// see how awesome rust's zero cost abstractions are
use std::fs::read_to_string;

/// calculates the one hot distance between two strings
/// used for comparing both sides of a reflection line
fn distance(l: &Vec<char>, r: &Vec<char>) -> usize {
    l.iter().zip(r.iter()).filter(|(a, b)| a != b).count()
}

/// calculates the reflection line for a given block
/// is reused for columns by rotating the input
fn reflection_row(block: Vec<Vec<char>>, distance_to_match: usize) -> usize {
    for idx in 1..block.len() {
        if block[..idx].iter().rev().zip(block[idx..].iter())
            .map(|(l, r)| distance(l, r)).sum::<usize>() == distance_to_match {
            return idx;
        }
    }
    0
}

/// scores the columns left of reflection line
/// or the rows * 100 above the reflection line
fn score_block(block: &str, distance_to_match: usize) -> Result<usize, &'static str> {
    let rows: Vec<Vec<char>> = block.split("\n").map(|row| row.chars().collect()).collect();
    if let row @ 1..=usize::MAX = reflection_row(rows.clone(), distance_to_match) {
        return Ok(100 * row);
    }

    let cols: Vec<Vec<char>> = (0..rows[0].len()).map(|i| rows.iter().map(|row| row[i]).collect()).collect();
    if let col @ 1..=usize::MAX = reflection_row(cols, distance_to_match) {
        return Ok(col);
    }

    //println!("{:?}", block);
    Err("no reflection found (shouldn't happen).")
}


fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    println!("Input file: {:?}", input_file);

    let input = read_to_string(input_file).expect("Failed to read file");

    let part_1: usize = input.split("\n\n")
        .map(|block| score_block(block, 0).unwrap())
        .sum();
    println!("Part 1: {}", part_1);

    let part_2: usize = input.split("\n\n")
        .map(|block| score_block(block, 1).unwrap())
        .sum();
    println!("Part 2: {}", part_2);

    Ok(())
}

