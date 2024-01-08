use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardinalDirection {
    North,
}

struct ReflectorGrid {
    rocks: HashSet<(usize, usize)>,
    walls: HashSet<(usize, usize)>,
    maxrow: usize,
    maxcol: usize,
}

impl ReflectorGrid {
    fn print(&self) {
        // ..= because the sets contain indices, not lengths
        for i in 0..=self.maxrow {
            for j in 0..=self.maxcol {
                if self.rocks.contains(&(i, j)) {
                    print!("O");
                } else if self.walls.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn rock_sorted(&self) -> Vec<(usize, usize)> {
        // rocks indices are sorted by row, then by column
        // collected from the HashSet into a Vec 
        let mut rocks: Vec<(usize, usize)> = self.rocks.iter().cloned().collect();
        rocks.sort_by(|(row1, col1), (row2, col2)| {
            if row1 == row2 {
                col1.cmp(col2)
            } else {
                row1.cmp(row2)
            }
        });
        rocks
    }

    fn walls_sorted(&self) -> Vec<(usize, usize)> {
        // walls indices are sorted by row, then by column
        // collected from the HashSet into a Vec 
        let mut walls: Vec<(usize, usize)> = self.walls.iter().cloned().collect();
        walls.sort_by(|(row1, col1), (row2, col2)| {
            if row1 == row2 {
                col1.cmp(col2)
            } else {
                row1.cmp(row2)
            }
        });
        walls
    }

    fn tilt(&self, dir: CardinalDirection) -> ReflectorGrid {
        // tilts the grid into a direction, causing the rocks to roll
        // in that direction until they hit a wall or another rock '#' or 
        // a rock 'O' that has already come to a stop 
        // (i.e. the rock is in the same row or column as another rock that has stopped)
        // the rocks are sorted by row, then by column
        let rocks = &mut self.rock_sorted();
        let walls = &mut self.walls_sorted();

        if dir == CardinalDirection::North {
            // roll the rocks upwards
            for (row, col) in rocks.clone().iter_mut() {
                let mut new_location: Option<(usize, usize)> = None;
                for potential_row in (0..*row).rev() {
                    let potential_location = (potential_row, *col);
                    // if potential position is already occupied by a rock or a wall
                    if walls.contains(&potential_location) 
                        || rocks.contains(&potential_location) {
                        break;
                    }
                    new_location = Some(potential_location);
                }
                match new_location {
                    Some(new_location) => {
                        rocks.retain(|x| *x != (*row, *col));
                        rocks.push(new_location);
                    },
                    None => {},
                }
            }
        }           
        // TODO build new Grid from the new rocks
        ReflectorGrid {
            rocks: rocks.clone().into_iter().collect(),
            walls: walls.clone().into_iter().collect(),
            maxrow: self.maxrow,
            maxcol: self.maxcol,
        }
        
    }

    fn calculate_load(&self) -> usize {
        let grid_height = self.maxrow + 1;
        self.rocks.iter().map(|(row, _)| grid_height - row).sum()
    }
}

impl From<String> for ReflectorGrid {
    fn from(s: String) -> Self {
        let mut rocks: HashSet<(usize, usize)> = HashSet::new();
        let mut walls: HashSet<(usize, usize)> = HashSet::new();

        // parse the rocks and obstacles from the grid
        for (i, line) in s.lines().enumerate() {
            for (j, chr) in line.chars().enumerate() {
                match chr {
                    'O' => {
                        rocks.insert((i, j));
                    },
                    '#' => {
                        walls.insert((i, j));
                    },
                    _ => {},
                }
            }   
        }
        // get the maximum occuring indices of the rocks and walls
        let maxrow_rocks = rocks.iter().map(|(row, _)| row).max().unwrap();
        let maxrow_walls = walls.iter().map(|(row, _)| row).max().unwrap();
        let maxrow = maxrow_rocks.max(maxrow_walls);

        let maxcol_rocks = rocks.iter().map(|(_, col)| col).max().unwrap();
        let maxcol_walls = walls.iter().map(|(_, col)| col).max().unwrap();
        let maxcol = maxcol_rocks.max(maxcol_walls);

        ReflectorGrid {
            rocks: rocks.clone(),
            walls: walls.clone(),
            maxrow: *maxrow,
            maxcol: *maxcol,
        }
    }
}


fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    println!("Input file: {:?}", input_file);

    let file_str = read_to_string(input_file)?;

    // parse the rocks and obstacles from the grid
    let grid = ReflectorGrid::from(file_str);
    // Print the parsed grid for debugging
    println!("Parsed ReflectorGrid:");
    grid.print();
    let grid_tilted = grid.tilt(CardinalDirection::North);
    println!("Tilted ReflectorGrid");
    grid_tilted.print();

    println!("Total tilt load: {}", grid_tilted.calculate_load());

    Ok(())
}
