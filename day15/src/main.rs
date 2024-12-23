use std::fs::read_to_string;

fn main() {
    let path = "inputs/real.txt";
    let file = read_to_string(path).unwrap();
    let content: Vec<String> = file
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|y| String::from(y))
        .collect();

    let mut map: Vec<Vec<Location>> = content[0]
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|y| {
            y.split("")
                .filter(|z| !z.is_empty())
                .map(|w| Location::new(w).unwrap())
                .collect()
        })
        .collect();

    let moves: Vec<Direction> = content[1]
        .replace("\n", "")
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| Direction::new(x).unwrap())
        .collect();

    let mut robot_location = None;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == Location::Robot {
                robot_location = Some((i, j));
            }
        }
    }

    for dir in moves {
        if let Some((i, j)) = robot_location {
            robot_location = update(&mut map, dir, (i, j));
        }
    }

    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            map[i][j].prettyprint();
            if map[i][j] == Location::Box {
                sum += 100 * i + j;
            }
        }
        print!("\n");
    }

    println!("The sum is {}", sum);
}

enum Direction {
    Left,
    Down,
    Up,
    Right,
}

impl Direction {
    fn new(s: &str) -> Option<Direction> {
        if s == "<" {
            return Some(Direction::Left);
        } else if s == "v" {
            return Some(Direction::Down);
        } else if s == "^" {
            return Some(Direction::Up);
        } else if s == ">" {
            return Some(Direction::Right);
        } else {
            return None;
        }
    }
}

#[derive(PartialEq)]
enum Location {
    Robot,
    Wall,
    Box,
    Free,
}

impl Location {
    fn new(s: &str) -> Option<Location> {
        if s == "#" {
            return Some(Location::Wall);
        } else if s == "O" {
            return Some(Location::Box);
        } else if s == "@" {
            return Some(Location::Robot);
        } else if s == "." {
            return Some(Location::Free);
        } else {
            return None;
        }
    }

    fn prettyprint(&self) {
        match self {
            Location::Wall => print!("#"),
            Location::Box => print!("O"),
            Location::Robot => print!("@"),
            Location::Free => print!("."),
        }
    }
}

fn update(
    map: &mut Vec<Vec<Location>>,
    dir: Direction,
    robot_location: (usize, usize),
) -> Option<(usize, usize)> {
    println!("{:?}", robot_location);
    match dir {
        Direction::Left => {
            for pos in (0..robot_location.1).rev() {
                if map[robot_location.0][pos] == Location::Wall {
                    break;
                } else if map[robot_location.0][pos] == Location::Free {
                    for newpos in pos..robot_location.1 {
                        map[robot_location.0][newpos] = Location::Box;
                    }
                    map[robot_location.0][robot_location.1 - 1] = Location::Robot;
                    map[robot_location.0][robot_location.1] = Location::Free;
                    return Some((robot_location.0, robot_location.1 - 1));
                }
            }
            return Some(robot_location);
        }

        Direction::Down => {
            for pos in robot_location.0..map.len() {
                if map[pos][robot_location.1] == Location::Wall {
                    break;
                } else if map[pos][robot_location.1] == Location::Free {
                    for newpos in (robot_location.0..pos + 1).rev() {
                        map[newpos][robot_location.1] = Location::Box;
                    }
                    map[robot_location.0 + 1][robot_location.1] = Location::Robot;
                    map[robot_location.0][robot_location.1] = Location::Free;
                    return Some((robot_location.0 + 1, robot_location.1));
                }
            }
            return Some(robot_location);
        }

        Direction::Up => {
            for pos in (0..robot_location.0).rev() {
                println!("Here");
                if map[pos][robot_location.1] == Location::Wall {
                    break;
                } else if map[pos][robot_location.1] == Location::Free {
                    for newpos in pos..robot_location.0 {
                        map[newpos][robot_location.1] = Location::Box;
                    }

                    map[robot_location.0 - 1][robot_location.1] = Location::Robot;
                    map[robot_location.0][robot_location.1] = Location::Free;
                    return Some((robot_location.0 - 1, robot_location.1));
                }
            }
            return Some(robot_location);
        }

        Direction::Right => {
            for pos in robot_location.1..map[0].len() {
                if map[robot_location.0][pos] == Location::Wall {
                    break;
                } else if map[robot_location.0][pos] == Location::Free {
                    for newpos in (robot_location.1..pos + 1).rev() {
                        map[robot_location.0][newpos] = Location::Box;
                    }
                    map[robot_location.0][robot_location.1 + 1] = Location::Robot;
                    map[robot_location.0][robot_location.1] = Location::Free;
                    return Some((robot_location.0, robot_location.1 + 1));
                }
            }
            return Some(robot_location);
        }
    }
}
