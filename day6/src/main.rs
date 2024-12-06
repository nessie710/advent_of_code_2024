use itertools::Itertools;
use nalgebra as na;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let filename = "inputs/day6.txt";
    let mut map = Map::new(filename);
    map.initialize_guard();

    while match map.move_guard() {
        Some(_) => true,
        None => false,
    } {}

    let positions: Vec<(usize, usize)> = map.visited_positions.iter().map(|x| x.0).collect();
    let sum = positions.iter().unique().count();
    println!("The sum of the visited places is {}", sum);

    // Part 2

    let map = Map::new(filename);

    let mut count = 0;
    for i in 0..map.rows {
        for j in 0..map.columns {
            if positions.contains(&(i, j))
                && match Location::from_numeric(map.map[(i, j)]) {
                    Location::Free => true,
                    _ => false,
                }
            {
                let mut map = Map::new(filename);
                //println!("{:?}", map.visited_positions);
                map.initialize_guard();
                map.map[(i, j)] = Location::Obstacle.to_numeric();
                println!("{:?}", (i, j));
                while match map.move_guard() {
                    Some(_) => true,
                    None => false,
                } {
                    match map.find_guard() {
                        Some(x) => {
                            if map.visited_positions.contains(&(x, map.direction)) {
                                count += 1;
                                println!("Count: {}", count);
                                break;
                            }
                        }
                        None => continue,
                    }
                }
            }
        }
    }

    println!("The count of loops is {}", count);
}

#[derive(Debug)]
enum Location {
    Obstacle,
    Free,
    Guard,
}

impl Location {
    // Map Location to a numeric representation
    fn to_numeric(&self) -> u8 {
        match self {
            Location::Obstacle => 1,
            Location::Free => 0,
            Location::Guard => 2,
        }
    }

    // Map numeric representation back to Location
    fn from_numeric(num: u8) -> Location {
        match num {
            1 => Location::Obstacle,
            0 => Location::Free,
            _ => Location::Guard,
        }
    }
}

#[derive(Debug)]
struct Map {
    map: na::DMatrix<u8>,
    rows: usize,
    columns: usize,
    direction: (i32, i32),
    initial_guard: (usize, usize),
    visited_positions: HashSet<((usize, usize), (i32, i32))>,
    last_intersection: (usize, usize),
    last_direction: (i32, i32),
}

impl Map {
    fn new(file_path: &str) -> Map {
        let file = read_to_string(file_path).unwrap();
        let content: Vec<Vec<u8>> = file
            .split("\n")
            .filter(|row| !row.is_empty())
            .map(|x| {
                x.split("")
                    .filter(|char| !char.is_empty())
                    .map(|y| match y {
                        "#" => Location::Obstacle.to_numeric(),
                        "." => Location::Free.to_numeric(),
                        _ => Location::Guard.to_numeric(),
                    })
                    .collect()
            })
            .collect();
        let rows = content.len();
        let columns = content[0].len();
        let flattened_content = content.into_iter().flatten();

        let matrix = na::DMatrix::from_row_iterator(rows, columns, flattened_content);
        return Map {
            map: matrix,
            rows: rows,
            columns: columns,
            direction: (0, -1),
            initial_guard: (0, 0),
            visited_positions: HashSet::new(),
            last_intersection: (0, 0),
            last_direction: (0, -1),
        };
    }

    fn initialize_guard(&mut self) {
        self.initial_guard = self.find_guard().unwrap();
        self.last_intersection = self.initial_guard;
    }

    fn find_guard(&self) -> Option<(usize, usize)> {
        // println!("Searching for guard:");
        for (i, row) in self.map.row_iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                // println!("Checking cell ({}, {}) = {}", i, j, value); // Debug
                if *value == Location::Guard.to_numeric() {
                    // println!("Guard found at ({}, {})", i, j); // Debug
                    return Some((i, j));
                }
            }
        }
        // println!("Guard not found."); // Debug
        return None;
    }

    fn rotate(&self) -> Option<(i32, i32)> {
        if self.direction == (0, -1) {
            return Some((1, 0));
        } else if self.direction == (1, 0) {
            return Some((0, 1));
        } else if self.direction == (0, 1) {
            return Some((-1, 0));
        } else if self.direction == (-1, 0) {
            return Some((0, -1));
        }
        return None;
    }

    fn move_guard(&mut self) -> Option<u8> {
        let current = match self.find_guard() {
            Some(x) => x,
            None => return None,
        };

        let mut newx = current.1 as i32 + self.direction.0;
        let mut newy = current.0 as i32 + self.direction.1;

        for _ in 0..4 {
            if newx < self.columns as i32
                && newx >= 0
                && newy >= 0
                && newy < self.rows as i32
                && (self.map[(newy as usize, newx as usize)] == Location::Obstacle.to_numeric())
            {
                self.direction = match self.rotate() {
                    Some(x) => x,
                    None => self.direction,
                };
                //return Some(1);
                newx = current.1 as i32 + self.direction.0;
                newy = current.0 as i32 + self.direction.1;
            } else {
                break;
            }
        }
        // println!("Direction : {:?}", self.direction);
        self.map[current] = Location::Free.to_numeric();
        self.visited_positions.insert((current, self.direction));
        if newx < self.columns as i32 && newx >= 0 && newy >= 0 && newy < self.rows as i32 {
            self.map[(newy as usize, newx as usize)] = Location::Guard.to_numeric()
        };
        return Some(1);
    }
}
