use std::fs::{read_to_string};

#[derive(Debug, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Number {
        chars: String,
        coordinate: Coordinate,
        length: usize,
}
#[derive(Debug, Clone)] 
struct Symbol {
        char: u8,
        coordinate: Coordinate,
        // length is always one
}

impl Number {
    fn new(chars: String, coordinate: Coordinate) -> Number {
        let len = chars.len();
        Number {
            chars: chars.clone(),
            coordinate,
            length: len,
        }
    }

    /// Check if the Symbol is a neighbour of the Number
    /// A neighbour is a Symbol that is one step away in any direction
    /// also diagonally(!)
    fn is_neighbour(&self, sym: &Symbol) -> bool {
        // x coord must differ at most 1 at each end
        let x_diff_min = (sym.coordinate.x - self.coordinate.x).abs();
        let x_diff_max = (sym.coordinate.x - (self.coordinate.x + self.length as i32 - 1)).abs();
        // both coordinates must differ at most 1
        let y_diff = (sym.coordinate.y - self.coordinate.y).abs();
        if (x_diff_min.min(x_diff_max) <= 1) && y_diff <= 1 {
            return true;
        }
        return false;
    }
}


fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    let mut numbers: Vec<Number> = Vec::with_capacity(2048);
    let mut symbols: Vec<Symbol> = Vec::with_capacity(2048);
    // parts are only those Numbers/Symbols that match some rules
    let mut partsum: u64 = 0;

    println!("Input file: {:?}", input_file);
    let file_str = read_to_string(input_file).expect("Failed to read file");

    file_str.split("\n")
    .enumerate()
    .for_each(|(y, line)| {
        let mut start_index = None;
        
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if start_index.is_none() {
                    start_index = Some(x);
                }
            } else {
                if let Some(start) = start_index {
                    process_group(start, x, y, &line[start..x], &mut numbers, &mut symbols);
                    start_index = None;
                }
                if c != '.' {
                    process_group(x, x+1, y, &line[x..x+1], &mut numbers, &mut symbols);
                }
            }
        }

        if let Some(start) = start_index {
            process_group(start, line.len(), y, &line[start..], &mut numbers, &mut symbols);
        }
    });

    // Now we have all the numbers and symbols and need to 
    // check for adjacency in the sorrounding lines
    let mut gear_ratios_sum: u64 = 0;
    // a symbol can have at most 6 adjacent numbers (diagonally counts as well)
    // we will add the adjacent numbers per symbol index to this nested vec
    let mut adjacent_numbers: Vec<Vec<u64>> = Vec::with_capacity(symbols.len());
    for _ in 0..adjacent_numbers.capacity() {
        adjacent_numbers.push(Vec::new());
    }
    
    // TODO apparently result changes depening on outer/inner loop order
    for number in &numbers {
        for (i, symbol) in symbols.iter().enumerate() {
            if number.is_neighbour(symbol) {
                println!("Symbol {} is neighbour of Number {}", symbol.char as char, number.chars);
                partsum += number.chars.parse::<u64>().unwrap();

                // add adjacent number to the symbol's adjacent_numbers vector (as u64)
                adjacent_numbers[i].push(number.chars.parse::<u64>().unwrap());

                // break to avoid counting it twice when we get to the symbol's own line
                break;
            }
        }
    }


    for nums in &adjacent_numbers {
        if nums.len() == 2 {
            gear_ratios_sum += nums[0] * nums[1];
        }
    }
    println!("Part number sum: {}", partsum); //part1
    println!("Part gear ratios sum: {}", gear_ratios_sum); //part2

    Ok(())
}


/// Processes a group of characters (separated/grouped by '.').
/// Groups can either be an integer number or a special character symbol (length 1).
/// Identified items are added to either the numbers or symbols vector
fn process_group(start: usize, end: usize, y: usize, group: &str, numbers: &mut Vec<Number>, symbols: &mut Vec<Symbol>) {
    if group.chars().all(|c| c.is_digit(10)) && !group.is_empty() {
        // Group is a number
        let number = Number::new(group.to_string(), Coordinate { x: start as i32, y: y as i32 });
        println!("Found Number: {}", number.chars);
        numbers.push(number);
    } else {
        // Process each character in the group as a symbol
        for (i, ch) in group.chars().enumerate() {
            if !ch.is_digit(10) {
                let symbol = Symbol { char: ch as u8, coordinate: Coordinate { x: (start + i) as i32, y: y as i32 } };
                println!("Found Symbol: {}", symbol.char as char);
                symbols.push(symbol);
            }
        }
    }
}
