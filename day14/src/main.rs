use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let path = "inputs/day14.txt";
    let file = read_to_string(path).unwrap();
    let content: Vec<String> = file
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|y| String::from(y))
        .collect();

    let re = Regex::new(
        r"p\=(?<rowp>-?\d{1,3})\,(?<colp>-?\d{1,3}) v\=(?<rowv>-?\d{1,3})\,(?<colv>-?\d{1,3})",
    )
    .unwrap();

    let mut robots = vec![];
    for robot in &content {
        let capture: Vec<_> = re.captures_iter(robot).collect();
        let position: (usize, usize) = (
            capture[0]["colp"].parse::<usize>().unwrap(),
            capture[0]["rowp"].parse::<usize>().unwrap(),
        );
        let velocity: (i32, i32) = (
            capture[0]["colv"].parse::<i32>().unwrap(),
            capture[0]["rowv"].parse::<i32>().unwrap(),
        );

        let rob = Robot { position, velocity };
        robots.push(rob);
    }

    let rows = 103;
    let columns = 101;

    for _ in 0..100 {
        update_robots(&mut robots, rows, columns);
    }

    let middle_row = rows / 2;
    let middle_col = columns / 2;
    println!("middle row {}", middle_row);
    println!("middle col {}", middle_col);

    let first_quardant_robots = robots
        .iter()
        .filter(|rob| rob.position.0 < middle_row as usize && rob.position.1 > middle_col as usize)
        .count();
    let second_quardant_robots = robots
        .iter()
        .filter(|rob| rob.position.0 < middle_row as usize && rob.position.1 < middle_col as usize)
        .count();
    let third_quardant_robots = robots
        .iter()
        .filter(|rob| rob.position.0 > middle_row as usize && rob.position.1 < middle_col as usize)
        .count();
    let fourth_quardant_robots = robots
        .iter()
        .filter(|rob| rob.position.0 > middle_row as usize && rob.position.1 > middle_col as usize)
        .count();

    println!(
        "The quadrants are Q1 {}, Q2 {}, Q3 {}, Q4 {}",
        first_quardant_robots,
        second_quardant_robots,
        third_quardant_robots,
        fourth_quardant_robots
    );
    println!(
        "The sum is {}",
        first_quardant_robots
            * second_quardant_robots
            * third_quardant_robots
            * fourth_quardant_robots
    );

    let path = "inputs/day14.txt";
    let file = read_to_string(path).unwrap();
    let content: Vec<String> = file
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|y| String::from(y))
        .collect();

    let re = Regex::new(
        r"p\=(?<rowp>-?\d{1,3})\,(?<colp>-?\d{1,3}) v\=(?<rowv>-?\d{1,3})\,(?<colv>-?\d{1,3})",
    )
    .unwrap();

    let mut robots = vec![];
    for robot in &content {
        let capture: Vec<_> = re.captures_iter(robot).collect();
        let position: (usize, usize) = (
            capture[0]["colp"].parse::<usize>().unwrap(),
            capture[0]["rowp"].parse::<usize>().unwrap(),
        );
        let velocity: (i32, i32) = (
            capture[0]["colv"].parse::<i32>().unwrap(),
            capture[0]["rowv"].parse::<i32>().unwrap(),
        );

        let rob = Robot { position, velocity };
        robots.push(rob);
    }

    let rows = 103;
    let columns = 101;
    let mut minimum = 0;
    let mut minimum_safety = 0;

    for i in 0..1000000 {
        update_robots(&mut robots, rows, columns);
        let first_quardant_robots = robots
            .iter()
            .filter(|rob| {
                rob.position.0 < middle_row as usize && rob.position.1 > middle_col as usize
            })
            .count();
        let second_quardant_robots = robots
            .iter()
            .filter(|rob| {
                rob.position.0 < middle_row as usize && rob.position.1 < middle_col as usize
            })
            .count();
        let third_quardant_robots = robots
            .iter()
            .filter(|rob| {
                rob.position.0 > middle_row as usize && rob.position.1 < middle_col as usize
            })
            .count();
        let fourth_quardant_robots = robots
            .iter()
            .filter(|rob| {
                rob.position.0 > middle_row as usize && rob.position.1 > middle_col as usize
            })
            .count();

        let safety = first_quardant_robots
            * second_quardant_robots
            * third_quardant_robots
            * fourth_quardant_robots;

        if i == 0 || minimum_safety > safety {
            minimum = i;
            minimum_safety = safety;
        }
    }
    println!("The minimum is {}", minimum);
}

struct Robot {
    position: (usize, usize),
    velocity: (i32, i32),
}

impl Robot {
    fn move_robot(&mut self, rows: u32, cols: u32) {
        let new_row = (self.position.0 as i32 + self.velocity.0).rem_euclid(rows as i32);
        let new_col = (self.position.1 as i32 + self.velocity.1).rem_euclid(cols as i32);

        self.position.0 = new_row as usize;
        self.position.1 = new_col as usize;
    }
}

fn update_robots(robots: &mut Vec<Robot>, rows: u32, columns: u32) {
    for robot in robots {
        robot.move_robot(rows, columns);
    }
}
