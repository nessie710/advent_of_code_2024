use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let path = "inputs/day11_test.txt";
    let file = read_to_string(path).unwrap();
    let stones: Vec<u64> = file
        .strip_suffix("\n")
        .unwrap()
        .split(" ")
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut stone_map: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        stone_map.insert(stone, 1);
    }

    for i in 0..6 {
        stone_map = update(stone_map);
    }

    let sum: u64 = stone_map.values().sum();

    println!("The sum is {}", sum);
}

fn update(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut output = stones.clone();
    println!("{:?}", stones);
    for stone in stones.keys() {
        if *stone == 0 {
            let num = output.get(stone).unwrap().clone();
            if let Some(val) = output.get_mut(stone) {
                *val = 0;
            }
            match output.get_mut(&1) {
                Some(val) => {
                    *val += num;
                }
                None => {
                    output.insert(1, num);
                }
            }
        } else if count_digits(stone) % 2 == 0 {
            let left = digits(*stone).0;
            let right = digits(*stone).1;
            let num = output.get(stone).unwrap().clone();
            if let Some(val) = output.get_mut(stone) {
                *val = 0;
            }
            match output.get_mut(&left) {
                Some(val) => {
                    *val += num;
                }
                None => {
                    output.insert(left, num);
                }
            }
            match output.get_mut(&right) {
                Some(val) => {
                    *val += num;
                }
                None => {
                    output.insert(right, num);
                }
            }
        } else {
            let num = output.get(stone).unwrap().clone();
            if let Some(val) = output.get_mut(stone) {
                *val = 0;
            }
            match output.get_mut(&(*stone * 2024)) {
                Some(val) => {
                    *val += num;
                }
                None => {
                    output.insert(*stone * 2024, num);
                }
            }
        }
    }
    return output;
}

fn count_digits(num: &u64) -> u32 {
    return num.to_string().chars().count() as u32;
}

fn digits(num: u64) -> (u64, u64) {
    let chars: Vec<char> = num.to_string().chars().collect();
    let num_chars = chars.len();
    let left_digits = chars[0..num_chars / 2].iter().collect::<String>();
    let right_digits = chars[num_chars / 2..].iter().collect::<String>();
    return (
        left_digits.parse::<u64>().unwrap(),
        right_digits.parse::<u64>().unwrap(),
    );
}
