use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input_file = "sample.txt";

    println!("Input file: {:?}", input_file);

    // read file line by line
    let file_str = read_to_string(input_file)?;

    // iterate over lines
    for (i, line) in file_str.lines().enumerate() {
        println!("{}", line);
        // Do stuff with the individual lines
    }

    println!("Print results");

    Ok(())
}

