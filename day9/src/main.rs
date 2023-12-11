use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    println!("Input file: {:?}", input_file);

    // read file line by line
    let file_str = read_to_string(input_file)?;
    let mut end_extrapolation_sum: i64 = 0;
    let mut start_extrapolation_sum: i64 = 0;
    // create 2D vector of integers to store the difference series
    // for one line to prevent allocations in main loop e.g. 
    // height/width is max length of series in input file
    // 0   3   6   9  12  15 // <- original series
    // 3   3   3   3   3
    // 0   0   0   0
    let mut diff_series: Vec<Vec<i64>> = Vec::with_capacity(128);
    for _ in 0..128 {
        diff_series.push(Vec::with_capacity(128));
    }

    // iterate over lines
    for (i, line) in file_str.lines().enumerate() {

        // parse the original series of integers
        diff_series[0] = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // extrapolate the last value
        let k = extrapolate_diff_series(&mut diff_series);
        // extrapolate the first value in the same way by flipping the series
        diff_series[0].reverse();
        extrapolate_diff_series(&mut diff_series);
        // flip back to original order and print
        diff_series[0].reverse();

        // print all difference sequences as a triangle of numbers
        println!("Series {}:", i);
        for series in diff_series[..=k].iter() {
            println!("  {:?}", series);
        }

        end_extrapolation_sum += diff_series[0].last().unwrap();
        start_extrapolation_sum += diff_series[0].first().unwrap();
    }

    println!("Sum of extrapolated ends {}", end_extrapolation_sum);
    println!("Sum of extrapolated starts {}", start_extrapolation_sum);

    Ok(())
}


fn extrapolate_diff_series(diff_series: &mut Vec<Vec<i64>>) -> usize {
    // calculate the first running difference of the series
    // apparently we're not supposed to use the absolute value
    // which wasn't mentioned anywhere in the problem description
    // this is also important for the test for zeros (sum vs. any())!
    diff_series[1] = diff_series[0]
        .windows(2)
        .map(|w| (w[1] - w[0]))
        .collect();
    // now do this while the sum of the running difference is not zero
    // and the length of the series is less than the max length
    let mut k = 1;
    while diff_series[k].iter().any(|x| *x != 0) && diff_series[k].len() > 1 {
        diff_series[k+1] = diff_series[k]
            .windows(2)
            .map(|w| (w[1] - w[0]))
            .collect();
        k += 1;
    }
    // extrapolate a single additional value to the series
    // by add a zero the the end of the zero series and then 
    // computing the sum of said zero and the last value of the series
    // A before it, appending the result to series A.  
    diff_series[k].push(0);
    for i in (0..k).rev() {
        let last_i_plus_1 = *diff_series[i+1].last().unwrap();
        let last_i = *diff_series[i].last().unwrap();
        diff_series[i].push(last_i_plus_1 + last_i);
    }
    // k is the index of the last series (zero series) 
    // just for easy printing outside of this function
    return k; 
}

