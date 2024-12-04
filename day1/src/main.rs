use std::fs::read_to_string;

fn main() {
    // Read and 1st part
    let file = read_to_string("inputs/day1.txt");
    let content: String = file.unwrap_or_else(|_| "".to_string());

    let rows: std::str::Split<'_, &str> = content.split("\n");
    let rows: Vec<&str> = rows.collect();
    let mut v1: Vec<u32> = Vec::with_capacity(rows.len());
    let mut v2: Vec<u32> = Vec::with_capacity(rows.len());

    for row in rows {
        let numbers: std::str::Split<'_, &str> = row.split("   ");
        let numbers: Vec<&str> = numbers.collect();
        if numbers.len() == 2 {
            let n1 = numbers[0].parse::<u32>().unwrap_or_else(|_| 0);
            let n2 = numbers[1].parse::<u32>().unwrap_or_else(|_| 0);
            v1.push(n1);
            v2.push(n2);
        }
    }
    v1.sort();
    v1.reverse();
    v2.sort();
    v2.reverse();
    let mut sum: u32 = 0;
    let mut v1temp = v1.clone();
    let mut v2temp = v2.clone();
    for _ in 0..v1temp.len() {
        sum += v1temp.pop().unwrap().abs_diff(v2temp.pop().unwrap());
    }

    println!("The sum is {}", sum);

    // Second part
    sum = 0;
    for el in v1.iter() {
        let n = v2.iter().filter(|&x| x == el).count();
        sum += (n as u32) * el;
    }

    println!("The sum is now {}", sum);
}
