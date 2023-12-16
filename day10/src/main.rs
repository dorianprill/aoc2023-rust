use core::panic;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    fn opposite(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }
}

// We have two types of pipe segments: 
// straight pipe and 90 degree bent pipe
// these are oriented on a 2D grid in 4 cardinal 
// directions (NESW) with exactly one pipe 
// segment at each grid point with two connections
#[derive(Debug, Copy, Clone)]
enum PipeSegment {
    Straight(bool, bool, bool, bool),
    Corner(bool, bool, bool, bool),
    // special name for start when its unknown, will only exist once
    Cross(bool, bool, bool, bool), 
}

impl PipeSegment {
    fn is_adjacent_connected(&self, other: PipeSegment, 
                            other_direction: CardinalDirection
    ) -> bool {
        match other_direction {
            CardinalDirection::North => {
                // cross/start can be connected in all directions
                match self {
                    PipeSegment::Cross(..) 
                    | PipeSegment::Straight(true, _, _, _)
                    | PipeSegment::Corner(true, _, _, _) => {
                        match other {
                            // | can be connected to |, F, 7
                            PipeSegment::Cross(..) => true,
                            PipeSegment::Straight(true, _, true, _) => true,
                            PipeSegment::Corner(_, _, true, _) => true, //F,7
                            _ => false,
                        }
                    },
                    _ => false,
                }
            },
            CardinalDirection::East => {
                match self {
                    PipeSegment::Cross(..) 
                    | PipeSegment::Straight(_, true, _, true)
                    | PipeSegment::Corner(_, true, _, _) => {
                        match other {
                            // - can be connected to -, J, 7
                            PipeSegment::Cross(..) => true,
                            PipeSegment::Straight(_, true, _, true) => true,
                            PipeSegment::Corner(_, _, _, true) => true, // J, 7
                            _ => false,
                        }
                    },
                    _ => false,
                }
            },
            CardinalDirection::South => {
                match self {
                    PipeSegment::Cross(..) 
                    | PipeSegment::Straight(_, _, true, _)
                    | PipeSegment::Corner(_, _, true, _) => {
                        match other {
                            // | can be connected to |, L, J
                            PipeSegment::Cross(..) => true,
                            PipeSegment::Straight(true, _, _, _) => true, //'|'
                            PipeSegment::Corner(true, _, _, _) => true, // L, J
                            _ => false,
                        }
                    },
                    _ => false,
                }
            },
            CardinalDirection::West => {
                match self {
                    PipeSegment::Cross(..) 
                    | PipeSegment::Straight(_, true, _, true)
                    | PipeSegment::Corner(_, _, _, true) => {
                        match other {
                            // - can be connected to -, J, 7
                            PipeSegment::Cross(..) => true,
                            PipeSegment::Straight(_, true, _, true) => true, // -
                            PipeSegment::Corner(_, true, _, _) => true, // j, 7
                            _ => false,
                        }
                    },
                    _ => false,
                }
            },
        }
    }

}

impl From<char> for PipeSegment {
    fn from(c: char) -> Self {
        match c {
            // CARDINAL DIRECTIONS       NORTH, EAST,  SOUTH, WEST
            '|' => PipeSegment::Straight(true,  false, true,  false),
            '-' => PipeSegment::Straight(false, true,  false, true),
            'F' => PipeSegment::Corner(  false, true,  true,  false),
            '7' => PipeSegment::Corner(  false, false, true,  true),
            'J' => PipeSegment::Corner(  true,  false, false, true),
            'L' => PipeSegment::Corner(  true,  true,  false, false),
            'S' => PipeSegment::Cross(   true,  true,  true,  true),
            _ => {
                println!("PipeSegment::from(): Invalid pipe segment: {}", c);
                panic!("Invalid pipe segment: {}", c)
            }
        }
    }
}

impl std::fmt::Display for PipeSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PipeSegment::Cross(true, true, true, true) => {
                write!(f, "S")
            },
            PipeSegment::Straight(true, false, true, false) => {
                write!(f, "|")
            },
            PipeSegment::Straight(false, true, false, true) => {
                write!(f, "-")
            },
            PipeSegment::Corner(false, true, true, false) => {
                write!(f, "F")
            },
            PipeSegment::Corner(false, false, true, true) => {
                write!(f, "7")
            },
            PipeSegment::Corner(true, false, false, true) => {
                write!(f, "J")
            },
            PipeSegment::Corner(true, true, false, false) => {
                write!(f, "L")
            },
            _ => {
                println!("PipeSegment::fmt(): Invalid pipe segment: {}", self);
                panic!("Invalid pipe segment: {}", self)
            }
        }
    }
}


#[derive(Debug, Copy, Clone)]
enum MapCell {
    Empty,
    // start can be considered a special pipe segment, until we know more
    Start(PipeSegment), 
    PipeSegment(PipeSegment),
}

impl MapCell {

    fn is_empty(&self) -> bool {
        match self {
            MapCell::Empty => true,
            _ => false,
        }
    }

    fn is_start(&self) -> bool {
        match self {
            MapCell::Start(_) => true,
            _ => false,
        }
    }

    fn is_pipe_segment(&self) -> bool {
        match self {
            MapCell::Start(_) => true,
            MapCell::PipeSegment(_) => true,
            _ => false,
        }
    }


    fn is_connected(&self, other: MapCell, 
                    other_direction: CardinalDirection
    ) -> bool {
        match self {
            MapCell::Empty => false,
            MapCell::Start(p) 
            | MapCell::PipeSegment(p) => {
                match other {
                    MapCell::Empty => false,
                    MapCell::Start(p2)
                    | MapCell::PipeSegment(p2) => {
                        p.is_adjacent_connected(p2, other_direction)
                    },
                }
            },
        }
    }

}

impl From<char> for MapCell {
    fn from(c: char) -> Self {
        match c {
            '.' => MapCell::Empty,
            'S' => MapCell::Start(PipeSegment::from(c)),
              _ => MapCell::PipeSegment(PipeSegment::from(c)),
        }
    }
}

// we do this to verify that we parsed everything correctly
impl std::fmt::Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MapCell::Empty => write!(f, "."),
            MapCell::Start(..) => write!(f, "S"),
            MapCell::PipeSegment(p) => write!(f, "{}", p),
        }
    }
}


// create a grid of 141x141 MapCells (my personal input size)
struct MapGrid {
    // note: grid is column major
    grid: [[MapCell; 141]; 141],
    start: (usize, usize), //(y,x) 'S' cell
    xmax: usize, // maximum x coordinate of parsed grid
    ymax: usize, // .. y .. (useful for sizes << 140)
}

impl MapGrid {

    pub fn from_str(s: &str) -> Self {
        let mut grid = [[MapCell::Empty; 141]; 141];
        let mut start = (0, 0);


        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid[i][j] = MapCell::from(c);
                if grid[i][j].is_start() {
                    start = (i, j);
                }
            }
        }

        MapGrid {
            grid: grid,
            start: start,
            xmax: s.lines().next().unwrap().len() - 1,
            ymax: s.lines().count() - 1,
        }
    }

    fn print(&self) {
        for row in self.grid[..=self.ymax].iter() {
            for cell in row[..=self.xmax].iter() {
                print!("{}", cell);
            }
            println!();
        }
    }

    fn connected_directions(&self, x: usize, y: usize) -> Vec<CardinalDirection> {
        let mut directions = Vec::new();
        if self.grid[y][x].is_pipe_segment() || self.grid[y][x].is_start() {
            if x > 0 {
                if self.grid[y][x].is_connected(self.grid[y][x-1], CardinalDirection::West) {
                    directions.push(CardinalDirection::West);
                }
            }    
            if x < self.xmax {
                if self.grid[y][x].is_connected(self.grid[y][x+1], CardinalDirection::East) {
                    directions.push(CardinalDirection::East);
                }
            } 
            if y > 0 {
                if self.grid[y][x].is_connected(self.grid[y-1][x], CardinalDirection::North) {
                    directions.push(CardinalDirection::North);
                }
            }
            if y < self.ymax {
                if self.grid[y][x].is_connected(self.grid[y+1][x], CardinalDirection::South) {
                    //println!("{} connected to South", self.grid[y][x]);
                    directions.push(CardinalDirection::South);
                }
            }
        }
        directions
    }

    /// TODO
    /// replace the start cell with the correct pipe segment
    /// this is important for the scanline approach for part2
    fn replace_start(&mut self, pipeloop: &Vec<(usize, usize)>) {
        let (y, x) = self.start;
        // find the precursor and successor of the start
        let mut start_precursor = (0, 0);
        let mut start_successor = (0, 0);
        for (i, (y2, x2)) in pipeloop.iter().enumerate() {
            if *y2 == y && *x2 == x {
                if i == 0 {
                    start_precursor = pipeloop[pipeloop.len()-1];
                    start_successor = pipeloop[1];
                } else if i == pipeloop.len()-1 {
                    start_precursor = pipeloop[pipeloop.len()-2];
                    start_successor = pipeloop[0];
                } else {
                    start_precursor = pipeloop[i-1];
                    start_successor = pipeloop[i+1];
                }
                break;
            }
        }
        // TODO find out the cardinal direction from the coordinates
        

    }
}


fn main() -> std::io::Result<()> {
    let input_file = "sample3.txt";

    println!("Input file: {:?}", input_file);

    // read file line by line
    let file_str = read_to_string(input_file)?;

    // read MapGrid form input string
    let mut map = MapGrid::from_str(&file_str);

    println!("Input Map:");
    for line in file_str.lines() {
        println!("{}", line);
    }
    println!("Parsed MapGrid:");
    // print map
    //map.print();
    println!("Start Coordinates: {:?}", map.start);

    let (starty, startx) = map.start;

    let mut pipeloop: Vec<(usize, usize)> = Vec::new();
    let found = find_loop_dfs(&map, &mut pipeloop, startx, starty, startx, starty, None);

    if found {
        println!("Found loop: {:?}", pipeloop);
    } else {
        println!("Did not find loop");
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Did not find loop"))?;
    }

    // now find the maximum steps away from S
    let step_series: Vec<usize> = (1..=pipeloop.len()/2) // Increasing part
        .chain((1..pipeloop.len()/2).rev()) // Decreasing part
        .collect(); // Collect into a vector

    println!("step_series: {:?} len: {}", step_series, step_series.len());
    println!("Maximum Distance: {} steps", step_series.iter().max().unwrap());

    // now we apply the polygonal jordan curve theorem
    // by scanning through the lines of the map
    // and counting the number of intersections with the loop and the number of tiles
    // odd intersections mean the tiles are inside the loop
    // even intersections mean the tiles are outside the loop
    
    // TODOreplace 'S' with adequate pipe segment 
    //map.replace_start(&pipeloop);
    map.print();

    let mut area: usize = 0;
    let mut intersections: usize = 0;
    for (i, row) in map.grid.iter().enumerate() {
        if i > map.ymax {
            break;
        }
        for (j, cell) in row.iter().enumerate() {
            if j > map.xmax {
                break;
            }
            // cast out a ray from the current cell to the right
            intersections = 0;
            let mut opening_segment = PipeSegment::Corner(false, false, false, false);
            for k in j+1..=map.xmax {
                
                if map.grid[i][k].is_pipe_segment() 
                    && pipeloop.contains(&(i, k)) {

                    // because of direction changes in North-South
                    // F----7 and 
                    // L----J is 2 intersections (pockets)
                    // L----7 and 
                    // F----J is 1 intersection (saddles)

                    // match and save opening segment, skip the rest
                    match map.grid[i][k] {
                        // F
                        MapCell::PipeSegment(PipeSegment::Corner(_, true, true, _)) => {
                            opening_segment = PipeSegment::Corner(false, true, true, false);
                            println!("found opening segment: {:?}", opening_segment);
                            continue;
                        }, // L
                        MapCell::PipeSegment(PipeSegment::Corner(true, true, _, _)) => {
                            opening_segment = PipeSegment::Corner(true, true, false, false);
                            println!("found opening segment: {:?}", opening_segment);
                            continue;
                        },
                        MapCell::PipeSegment(PipeSegment::Straight(_, true, _, true)) => {
                            //println!("skipping '-' at ({}, {})", i, k);
                            continue; //ignore all '-'
                        },
                        _ => (),
                    }
                    // match closing statement on the loop and skip all '-'
                    match opening_segment {
                        // F
                        PipeSegment::Corner(false, true, true, false) => {
                                match map.grid[i][k] {
                                    // J
                                    MapCell::PipeSegment(PipeSegment::Corner(true, _, _, true)) => {
                                        intersections += 1;
                                        opening_segment = PipeSegment::Corner(false, false, false, false);
                                        println!("found closing segment: {:?}", map.grid[i][k]);
                                    },
                                    // 7
                                    MapCell::PipeSegment(PipeSegment::Corner(_, _, true, true)) => {
                                        intersections += 2;
                                        opening_segment = PipeSegment::Corner(false, false, false, false);
                                        println!("found closing segment: {:?}", map.grid[i][k]);
                                    },
                                    _ => (),
                                }
                        },
                        // L
                        PipeSegment::Corner(true, true, false, false) => {
                                match map.grid[i][k] {
                                    // 7
                                    MapCell::PipeSegment(PipeSegment::Corner(_, true, true, _)) => {
                                        intersections += 1;
                                        opening_segment = PipeSegment::Corner(false, false, false, false);
                                        println!("found closing segment: {:?}", map.grid[i][k]);
                                    },
                                    // J
                                    MapCell::PipeSegment(PipeSegment::Corner(true, true, _, _)) => {
                                        intersections += 2;
                                        opening_segment = PipeSegment::Corner(false, false, false, false);
                                        println!("found closing segment: {:?}", map.grid[i][k]);
                                    },
                                    _ => (),
                                }
                        },
                        _ => (),
                    }
                }
            }
            if !pipeloop.contains(&(i,j)) && intersections % 2 == 1 {
                println!("Cell: {:?} ({}, {}) counted due to {} intersections", cell, i, j, intersections);
                area += 1;
            }
        }
        //println!("Row: {} Intersections: {} Area: {}", i, intersections, area)
    }

    println!("Area inside loop: {}", area);

    Ok(())
}



// assumes there is only one loop connected to the start
// comment out the println!() statements to prevent stack overflow
fn find_loop_dfs(map: &MapGrid, path: &mut Vec<(usize, usize)>, x: usize, y: usize, start_x: usize, start_y: usize, from_direction: Option<CardinalDirection>) -> bool {
    if path.contains(&(y, x)) {
        return x == start_x && y == start_y;
    }

    path.push((y, x));

    let directions = map.connected_directions(x, y);
    //println!("{:?} ({}) ---> {:?}", map.grid[y][x], map.grid[y][x], directions);
    for direction in &directions {
        // Skip the direction we came from
        // (if we went East we are now coming from West)
        if let Some(from_direction) = from_direction {
            if direction.opposite() == from_direction {
                continue;
            }
        }
    
        let mut new_x = x;
        let mut new_y = y;

        match direction {
            CardinalDirection::North => new_y -= 1,
            CardinalDirection::East => new_x += 1,
            CardinalDirection::South => new_y += 1,
            CardinalDirection::West => new_x -= 1,
        }
        if new_x < map.grid[y].len() &&  new_y < map.grid.len() {
            if find_loop_dfs(map, path, new_x as usize, new_y as usize, start_x, start_y, Some(*direction)) {
                //println!("Found loop: {:?} on {} from direction {:?}", path, map.grid[y][x], direction);
                return true;
            }
        }
    }

    path.pop(); // backtrack
    //println!("Did not find loop. Backtracking: {:?}", path);
    false
}
