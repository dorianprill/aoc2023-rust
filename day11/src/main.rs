use std::fs::read_to_string;

// we already count the original row/col when calculating the L1 norm
// so we need to subtract one for
const EXPANSION_P1: usize = 2 - 1; 
const EXPANSION_P2: usize = 1000000 - 1;

type Galaxy = (usize, usize);

struct Universe {
    // TODO can replace the grid with just the precomputed
    // expansion rows and cols indices to save space for larger grids
    galaxies: Vec<Galaxy>,
    max_rows: usize,
    max_cols: usize,
    galaxy_grid: Vec<Vec<bool>>,
}

impl Universe {

        fn shortest_l1_paths_sum(&self, expansion: usize) -> usize {
            let mut shortest_paths_sum: usize = 0;
            // check if any of the rows is empty, if yes, save the coordinate
            let expanded_rows = self.galaxy_grid
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if v.iter().all(|b| *b == false) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();
            // check if any of the columns is empty
            let mut expanded_cols: Vec<usize> = Vec::with_capacity(self.max_cols);
            for l in 0..self.max_cols {
                let mut col = false;
                for k in 0..self.max_rows {
                    if self.galaxy_grid[k][l] {
                        col = true;
                        break;
                    }
                }
                // if whole col was empty, push the coordinate
                if col == false {
                    expanded_cols.push(l);
                }
            }
            println!("expanded_cols: {:?}", expanded_cols);
            println!("expanded_rows: {:?}", expanded_rows);

            for (start_outer, g1) in self.galaxies.iter().enumerate() {
                for (start_inner, g2) in self.galaxies.iter().enumerate().skip(start_outer+1) {
                    // calculate shortest path in L1 norm
                    let path = (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs();
                    // adjust path for expansion by adding the 
                    // collected expansion rows and cols (works because we're using L1)
                    let path_exp: usize = path as usize 
                        // x direction
                        + expanded_rows
                        .iter()
                        .filter(|i| {
                            **i > g1.0.min(g2.0) && **i < g1.0.max(g2.0) 
                        })
                        .count() 
                        * expansion
                        // y direction
                        + expanded_cols
                        .iter()
                        .filter(|i| {
                            **i > g1.1.min(g2.1) && **i < g1.1.max(g2.1) 
                        })
                        .count() 
                        * expansion;
                        // uncomment for debugging
                        // println!("G{} --> G{} = {}", start_outer+1, start_inner+1, path_exp);
                    
                    shortest_paths_sum += path_exp;
                }
            }
            shortest_paths_sum
        }
}


impl From<String> for Universe {
    fn from(str: String) -> Self {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let max_rows: usize = str.lines().count();
        let max_cols: usize = str.lines().next().unwrap().chars().count();
        let mut galaxy_grid: Vec<Vec<bool>>;

        // pre-initialize the grid vectors
        galaxy_grid = vec![vec![false; max_cols]; max_rows];


        for (i, line) in str.lines().enumerate() {
            for (j, chr) in line.chars().enumerate() {
                match chr {
                    '#' => {
                        galaxies.push((i,j));
                        galaxy_grid[i][j] = true;
                    },
                    '.' => galaxy_grid[i][j] = false,
                    _ => (),
                }
            }
        }

        Universe{
            galaxies,
            max_rows,
            max_cols,
            galaxy_grid: galaxy_grid.clone()
        }
    }

}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Implement the logic to convert the Universe instance to a string
        // This is just a placeholder. Replace it with your actual logic.
        for i in 0..self.max_rows {
            for j in 0..self.max_cols {
                if self.galaxies.contains(&(i,j)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    println!("Input file: {:?}", input_file);

    // read file line by if it is emptyline
    let file_str = read_to_string(input_file)?;

    println!("Input Universe:\n{}", file_str);

    // iterate over lines
    let universe = Universe::from(file_str);

    println!("Parsed Universe:\n{universe}");

    println!("Expanded Universe Sum of Shortest Paths:");
    println!("Part 1: Factor {} --> Sum {}", EXPANSION_P1+1, universe.shortest_l1_paths_sum(EXPANSION_P1));
    println!("Part 2: Factor {} --> Sum {}", EXPANSION_P2+1, universe.shortest_l1_paths_sum(EXPANSION_P2));


    Ok(())
}

