use std::fs::read_to_string;

fn main() {
    let path = "inputs/day10.txt";
    let mut map = Map::new(path);
    // for row in &map.map {
    //     for num in row {
    //         print!("{}", num);
    //     }
    //     println!("")
    // }

    let mut sum = 0;
    let mut sum2 = 0;
    for start in map.startpoints.clone() {
        let count = map.count_paths(start);
        map.visited_endpoints = vec![];
        sum += count.0;
        sum2 += count.1;
    }

    println!("The sum is {}", sum);
    println!("The sum now is {}", sum2);
}

struct Map {
    map: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    startpoints: Vec<(usize, usize)>,
    visited_endpoints: Vec<(usize, usize)>,
}

impl Map {
    fn new(path: &str) -> Map {
        let file = read_to_string(path).unwrap();
        let content: Vec<Vec<u8>> = file
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|y| {
                y.split("")
                    .filter(|z| !z.is_empty())
                    .map(|w| w.parse::<u8>().unwrap())
                    .collect()
            })
            .collect();

        let mut startpoints = vec![];
        for (i, row) in content.iter().enumerate() {
            for (j, point) in row.iter().enumerate() {
                if *point == 0 {
                    startpoints.push((i, j));
                }
            }
        }
        let rows = content.len();
        let cols = content[0].len();

        return Map {
            map: content,
            rows: rows,
            cols: cols,
            startpoints: startpoints,
            visited_endpoints: vec![],
        };
    }

    fn count_paths(&mut self, start: (usize, usize)) -> (u32, u32) {
        let directions = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
        let mut available_dirs = vec![];

        for dir in &directions {
            if !(dir.0 == -1 && start.0 == 0)
                && !(dir.1 == -1 && start.1 == 0)
                && !(dir.0 == 1 && start.0 == self.rows - 1)
                && !(dir.1 == 1 && start.1 == self.cols - 1)
            {
                if self.map[(start.0 as i32 + dir.0) as usize][(start.1 as i32 + dir.1) as usize]
                    == self.map[start.0][start.1] + 1
                {
                    available_dirs.push(dir);
                }
            }
        }
        let mut count = 0;
        let mut count2 = 0;
        for dir in available_dirs {
            let next = (
                (start.0 as i32 + dir.0) as usize,
                (start.1 as i32 + dir.1) as usize,
            );
            if self.map[next.0][next.1] == 9 {
                if !self.visited_endpoints.contains(&next) {
                    self.visited_endpoints.push(next);
                    count += 1;
                }
                count2 += 1;
            } else {
                count += self.count_paths(next).0;
                count2 += self.count_paths(next).1;
            }
        }
        return (count, count2);
    }
}
