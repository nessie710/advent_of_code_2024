use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let path = "inputs/day12.txt";
    let file = read_to_string(path).unwrap();
    let content: Vec<Vec<String>> = file
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|y| {
            y.split("")
                .filter(|z| !z.is_empty())
                .map(|w| String::from(w))
                .collect()
        })
        .to_owned()
        .collect();

    let mut greyed: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;
    let mut sum_with_discount = 0;

    for i in 0..content.len() {
        for j in 0..content[0].len() {
            if !greyed.contains(&(i, j)) {
                let field = search(&content, (i, j), &mut HashSet::new());
                println!("{:?}", content[i][j]);
                sum += process(&field, &content);
                sum_with_discount += process_with_discount(&field, &content);
                for coord in field {
                    greyed.insert(coord);
                }
            }
        }
    }

    println!("The sum is {}", sum);
    println!("The sum now is {}", sum_with_discount);
}

fn search(
    map: &Vec<Vec<String>>,
    startpoint: (usize, usize),
    output: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    // let mut output = HashSet::new();
    output.insert(startpoint);

    for dir in directions {
        if (startpoint.0 as i32 + dir.1) >= 0
            && (startpoint.0 as i32 + dir.1) < map.len() as i32
            && (startpoint.1 as i32 + dir.0) >= 0
            && (startpoint.1 as i32 + dir.0) < map[0].len() as i32
        {
            let newx = startpoint.1 as i32 + dir.0;
            let newy = startpoint.0 as i32 + dir.1;
            if !output.contains(&(newy as usize, newx as usize))
                && map[newy as usize][newx as usize] == map[startpoint.0][startpoint.1]
            {
                // output.insert((newy as usize, newx as usize));
                let _newsearch = search(map, (newy as usize, newx as usize), output);
            }
        }
    }

    return output.clone();
}

fn process(field: &HashSet<(usize, usize)>, map: &Vec<Vec<String>>) -> u32 {
    let area = field.iter().count();
    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut count_sides = 0;
    let mut perimeter = 0;

    for f in field {
        for dir in &directions {
            if (f.0 as i32 + dir.1) >= 0
                && (f.0 as i32 + dir.1) < map.len() as i32
                && (f.1 as i32 + dir.0) >= 0
                && (f.1 as i32 + dir.0) < map[0].len() as i32
            {
                let newx = f.1 as i32 + dir.0;
                let newy = f.0 as i32 + dir.1;

                if map[newy as usize][newx as usize] != map[f.0][f.1] {
                    count_sides += 1;
                }
            } else {
                count_sides += 1;
            }
        }
        perimeter += count_sides;
        count_sides = 0;
    }

    return perimeter * area as u32;
}

fn process_with_discount(field: &HashSet<(usize, usize)>, map: &Vec<Vec<String>>) -> u32 {
    let area = field.iter().count();

    let straight_directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let diagonal_directions: Vec<(i32, i32)> = vec![(1, 1), (-1, -1), (-1, 1), (1, -1)];

    let mut count_sides: u32 = 0;
    let mut checked_corners: HashSet<((usize, usize), (i32, i32), (i32, i32))> = HashSet::new();
    let outer = find_outer_dirs(field, map);

    let mut diag_only: Vec<(usize, usize, u32)> = vec![];
    for key in outer.keys() {
        // let mut countable = false;
        let mut count_diags = 0;
        for diag in outer
            .get(key)
            .unwrap()
            .iter()
            .filter(|x| diagonal_directions.contains(x))
        {
            let mut adjacent = false;
            for sdir in outer
                .get(key)
                .unwrap()
                .iter()
                .filter(|y| straight_directions.contains(y))
            {
                if is_adjacent(diag, sdir) {
                    adjacent = true;
                }
            }
            if !adjacent {
                count_diags += 1;
            }
        }

        if count_diags != 0 {
            diag_only.push((key.0, key.1, count_diags));
        }
    }

    for f in outer.keys() {
        let outer_dirs = outer.get(f).unwrap();

        for dir in outer_dirs {
            for dir2 in outer_dirs {
                if dir != dir2
                    && is_perpendicular(dir, dir2)
                    && !checked_corners.contains(&(*f, *dir, *dir2))
                {
                    count_sides += 1;
                    checked_corners.insert((*f, *dir, *dir2));
                    checked_corners.insert((*f, *dir2, *dir));
                    // println!("{:?}", checked_corners);
                }
            }
        }
    }
    for f2 in &diag_only {
        count_sides += f2.2;
    }
    println!("Count sides {}", count_sides);
    return count_sides * area as u32;
}

fn find_outer_dirs(
    field: &HashSet<(usize, usize)>,
    map: &Vec<Vec<String>>,
) -> HashMap<(usize, usize), Vec<(i32, i32)>> {
    let directions = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut outer: HashMap<(usize, usize), Vec<(i32, i32)>> = HashMap::new();
    let mut count_outer_directions = vec![];
    for f in field {
        for dir in &directions {
            if (f.0 as i32 + dir.1) >= 0
                && (f.0 as i32 + dir.1) < map.len() as i32
                && (f.1 as i32 + dir.0) >= 0
                && (f.1 as i32 + dir.0) < map[0].len() as i32
            {
                let newx = f.1 as i32 + dir.0;
                let newy = f.0 as i32 + dir.1;

                if map[newy as usize][newx as usize] != map[f.0][f.1] {
                    count_outer_directions.push(*dir);
                }
            } else {
                count_outer_directions.push(*dir);
            }
        }
        outer.insert(*f, count_outer_directions);
        count_outer_directions = vec![];
    }

    return outer;
}

fn is_perpendicular(dir1: &(i32, i32), dir2: &(i32, i32)) -> bool {
    if dir1.0 * dir2.0 == 0 && dir1.1 * dir2.1 == 0 {
        return true;
    } else {
        return false;
    }
}

fn is_adjacent(u: &(i32, i32), z: &(i32, i32)) -> bool {
    if (u.0 == z.0 || z.0 == 0) && (u.1 == z.1 || z.1 == 0) {
        return true;
    } else {
        return false;
    }
}
