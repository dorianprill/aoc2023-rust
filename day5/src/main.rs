use std::fs::{read_to_string};

#[derive(Debug)]
struct RangeMap {
    dest_range_start: u64,
    src_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct CategoryMaps {
    category_name: String,
    maps: Vec<RangeMap>,
}

impl RangeMap {
    fn map_value(&self, input: u64) -> u64 {
        // if number is not contained in the range, return the input
        if input < self.src_range_start {
            return input;
        }
        if input >= self.src_range_start + self.range_length {
            return input;
        }
        let offset = input - self.src_range_start;
        let dest = self.dest_range_start + offset;
        dest
    }

    fn contains(&self, input: u64) -> bool {
        if input < self.src_range_start {
            return false;
        }
        if input >= self.src_range_start + self.range_length {
            return false;
        }
        true
    }
}


impl CategoryMaps {
    pub fn parse_maps(category_chunk: &str) -> CategoryMaps {
        let mut lines = category_chunk.lines();

        // Extract the category name from the first line
        let category_name = lines.next().unwrap_or_default().split(" map:").nth(0).unwrap_or_default().to_string();

        let maps: Vec<RangeMap> = lines
            .filter_map(|line| {
                let mut split = line.split_whitespace();
                let dest_range_start = split.next()?.parse::<u64>().ok()?;
                let src_range_start = split.next()?.parse::<u64>().ok()?;
                let range_length = split.next()?.parse::<u64>().ok()?;
                Some(RangeMap {
                    dest_range_start,
                    src_range_start,
                    range_length,
                })
            })
            .collect();

        CategoryMaps { category_name, maps }
    }

    /// Maps the value through the first applicable map in each category
    /// because the way they are generated means there can be multiple 
    /// overlapping ranges but apparently this is what AoC authors meant?
    pub fn map_value(&self, input: u64) -> u64 {
        let mut current_value = input;
        for map in &self.maps {
            if map.contains(current_value) {
                current_value = map.map_value(current_value);
                break; // Stop after finding the first applicable map
            }
        }
        //print!(" -> {} {}", self.category_name.split("-").last().unwrap(), current_value);
        current_value
    }
}

fn main() -> std::io::Result<()> {
    let input_file = "input.txt";
    // read whole file into memory
    let file_str = read_to_string(input_file).expect("Failed to read file");

    // split input string into seed and maps portions
    let (seed_str, maps_str) = file_str.split_once("\n").unwrap();
    // split and parse seeds (split once keep right part)
    let seeds: Vec<u64> = seed_str
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();

    //println!("Seeds: {:?}", seeds);

    // part two, interpret seed pairs as ranges (start, length)
    let seed_ranges: Vec<(u64, u64)> = seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    
    //println!("Seed Ranges: {:?}", seed_ranges);

    // parse the RangeMaps per Category into Vec<CategoryMaps>
    let category_maps: Vec<CategoryMaps> = maps_str
        .trim() // aligns start to first category name
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|x| CategoryMaps::parse_maps(x))
        .collect();

    // now we start mapping seeds through the range maps
    let mut mapped_seeds: Vec<u64> = Vec::new();
    for seed in seeds {
        let mut mapped_seed = seed;
        //print!("Seed {seed}");
        for category_map in &category_maps {
            mapped_seed = category_map.map_value(mapped_seed);
        }
        //print!("\n");
        mapped_seeds.push(mapped_seed);
    }

    let min_location = mapped_seeds.iter().min().unwrap();
    let mut min_values = Vec::new();


    // TODO this is horribly slow, i probably overlooked some (obvious?) optimization
    for (start, length) in seed_ranges {
        let mut mapped_values = Vec::new();

        //print!("Range({}..{}):", start, start+length);
    
        for seed in start..start + length {
            let mut mapped_seed = seed;
            //print!("\nSeed {seed} ");
            for category_map in &category_maps {
                mapped_seed = category_map.map_value(mapped_seed);
            }
            mapped_values.push(mapped_seed);
        }

        //println!(" -> Min {}",*mapped_values.iter().min().unwrap());
    
        min_values.push(*mapped_values.iter().min().unwrap());
    }
    
    let min_location_rangeseed = min_values.iter().min().unwrap();
    
    println!("Single Seeds: Minimum Location: {min_location}");
    println!("Seed Ranges: Minimum location: {:?}",min_location_rangeseed);


    Ok(())
}

