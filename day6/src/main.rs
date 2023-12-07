use std::fs::read_to_string;

const BOAT_BASE_SPEED: u64 = 0; // mm/ms

fn main() -> std::io::Result<()> {
    let input_file = "sample.txt";

    println!("Input file: {:?}", input_file);

    // read file line by line
    let file_str = read_to_string(input_file)?;

    let (times_str, distances_str) = file_str.split_once("\n").unwrap();

    let (mut times, mut dist_records): (Vec<u64>, Vec<u64>) = (
        times_str.split_whitespace().filter_map(|x| x.parse().ok()).collect(),
        distances_str.split_whitespace().filter_map(|x| x.parse().ok()).collect()
    );

    println!("Course records:");
    for (time, distance) in times.iter().zip(dist_records.iter()) {
        println!("{} millimeter in {} milliseconds", time, distance);
    }

    // create an additional pair where all input digits are concatenated
    let (single_times, single_dist_records): (u64, u64) = (
        times_str.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap(),
        distances_str.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap()
    );

    // and just append it to the end of the lists
    times.push(single_times);
    dist_records.push(single_dist_records);

    // note: we will calculate solutions for part 1 and 1 seperately


    // button press for boat increases speed by 1 mm/ms per 1 ms button pressed
    // boat starts at 0 mm/ms
    // calculate all possible speeds for each time, distance combination
    let number_of_winning_strategies = times.iter().zip(dist_records.iter())
    .map(|(&time, &course_record)| {
        (BOAT_BASE_SPEED..time).filter_map(move |button_hold_time| {
            let speed = button_hold_time; // Speed increases by 1 unit for each second the button is held
            let remaining_time = time - button_hold_time; // Remaining time after holding the button
            let distance = speed * remaining_time; // Calculate distance

            if distance > course_record {
                Some(button_hold_time)
            } else {
                None
            }
        }).count()
    })
    .collect::<Vec<_>>();

    for((time, record), strats) in times.iter().zip(dist_records.iter()).zip(number_of_winning_strategies.iter()) {
        println!("Time {}, Record: {}, Strategies: {}", time, record, strats);
    }

    println!("Part 1: Total ways to win: {}", 
            number_of_winning_strategies
            .iter()
            .take(number_of_winning_strategies.len()-1)
            .product::<usize>()
    );

    // Since there is only one entry, we already have the total product of products
    println!("Part 2: Total ways to win: {:?}", 
            number_of_winning_strategies
            .iter()
            .last()
            .unwrap()
);

    Ok(())
}




