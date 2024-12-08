use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let path = "inputs/day8.txt";
    let file = read_to_string(path).unwrap();
    let map: Vec<Vec<&str>> = file
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|st| st.split("").filter(|y| !y.is_empty()).collect())
        .collect();
    let columns = map[0].len() as i32;
    let rows = map.len() as i32;
    let mut antennas: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashMap<(usize, usize), bool> = HashMap::new();

    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != "." {
                if !antennas.contains_key(cell) {
                    antennas.insert(cell, vec![(i, j)]);
                } else {
                    antennas.get_mut(cell).unwrap().push((i, j));
                }
            }
            antinodes.insert((i, j), false);
        }
    }

    for frequency in antennas.keys() {
        //println!("{}", frequency);
        let antenna_locations = antennas[frequency].clone();
        if antenna_locations.len() > 1 {
            let combinations: Vec<Vec<(usize, usize)>> =
                antenna_locations.into_iter().combinations(2).collect();
            for comb in combinations {
                let row_distance = comb[0].0 as i32 - comb[1].0 as i32;
                let col_distance = comb[0].1 as i32 - comb[1].1 as i32;
                let row_ant1 = comb[0].0 as i32 + row_distance;
                let col_ant1 = comb[0].1 as i32 + col_distance;
                let row_ant2 = comb[1].0 as i32 - row_distance;
                let col_ant2 = comb[1].1 as i32 - col_distance;
                if row_ant1 >= 0 && row_ant1 < rows && col_ant1 >= 0 && col_ant1 < columns {
                    let antinode1 = (row_ant1 as usize, col_ant1 as usize);
                    *antinodes.get_mut(&antinode1).unwrap() = true;
                };
                if row_ant2 >= 0 && row_ant2 < rows && col_ant2 >= 0 && col_ant2 < columns {
                    let antinode2 = (row_ant2 as usize, col_ant2 as usize);
                    *antinodes.get_mut(&antinode2).unwrap() = true;
                };
            }
        }
    }

    let sum = antinodes.values().filter(|x| **x).count();
    println!("The sum of antinodes is {}", sum);

    // Part 2

    for frequency in antennas.keys() {
        let antenna_locations = antennas[frequency].clone();
        if antenna_locations.len() > 1 {
            let combinations: Vec<Vec<(usize, usize)>> =
                antenna_locations.into_iter().combinations(2).collect();
            for comb in combinations {
                let row_distance = comb[0].0 as i32 - comb[1].0 as i32;
                let col_distance = comb[0].1 as i32 - comb[1].1 as i32;
                let mut current = comb[0];
                *antinodes.get_mut(&comb[0]).unwrap() = true;
                loop {
                    let next = (
                        current.0 as i32 + row_distance,
                        current.1 as i32 + col_distance,
                    );
                    if next.0 >= 0 && next.0 < rows && next.1 >= 0 && next.1 < columns {
                        *antinodes
                            .get_mut(&(next.0 as usize, next.1 as usize))
                            .unwrap() = true;
                        current = (next.0 as usize, next.1 as usize);
                    } else {
                        break;
                    }
                }
                loop {
                    let next = (
                        current.0 as i32 - row_distance,
                        current.1 as i32 - col_distance,
                    );
                    if next.0 >= 0 && next.0 < rows && next.1 >= 0 && next.1 < columns {
                        *antinodes
                            .get_mut(&(next.0 as usize, next.1 as usize))
                            .unwrap() = true;
                        current = (next.0 as usize, next.1 as usize);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let sum = antinodes.values().filter(|x| **x).count();
    println!("The sum of antinodes now is {}", sum);
}
