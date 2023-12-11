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


// create a grid of 140x140 MapCells (my personal input size)
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
}


fn main() -> std::io::Result<()> {
    let input_file = "input.txt";

    println!("Input file: {:?}", input_file);

    // read file line by line
    let file_str = read_to_string(input_file)?;

    // read MapGrid form input string
    let map = MapGrid::from_str(&file_str);

    println!("Input Map:");
    for line in file_str.lines() {
        println!("{}", line);
    }
    println!("Parsed MapGrid:");
    // print map
    map.print();
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

    // now find the maximum steps away
    // this is an overlap convolution of two constant (=1) vectors 
    // with length of the loop (actually half, because we only care for the overlapping part)
    let step_series: Vec<usize> = (1..=pipeloop.len()/2) // Increasing part
        .chain((1..pipeloop.len()/2).rev()) // Decreasing part
        .collect(); // Collect into a vector

    println!("step_series: {:?} len: {}", step_series, step_series.len());
    println!("Maximum Distance: {} steps", step_series.iter().max().unwrap());

    Ok(())
}



// assumes there is only one loop connected to the start
// comment out the println!() statements to prevent stack overflow
fn find_loop_dfs(map: &MapGrid, path: &mut Vec<(usize, usize)>, x: usize, y: usize, start_x: usize, start_y: usize, from_direction: Option<CardinalDirection>) -> bool {
    if path.contains(&(x, y)) {
        return x == start_x && y == start_y;
    }

    path.push((x, y));

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
