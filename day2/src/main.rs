use std::fs::read_to_string;

fn main() {
    // Read and 1st part
    let file = read_to_string("inputs/day2_test.txt");
    let content: String = file.unwrap_or_else(|_| "".to_string());

    let rows: Vec<&str> = content.split("\n").filter(|&x| !x.is_empty()).collect();

    let numbers: Vec<Vec<u32>> = rows
        .iter()
        .map(|y| y.split(" ").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();

    let sum = numbers.iter().filter(|row| is_safe(row)).count();
    println!("The sum is {:?}", sum);

    let sum_damp = numbers
        .iter()
        .filter(|row| is_safe_with_dampener(row))
        .count();
    println!("The sum with dampener is {}", sum_damp);
}

fn is_growing(row: &Vec<u32>) -> bool {
    if row[0] < row[1] {
        return true;
    } else {
        return false;
    }
}

fn is_safe(row: &Vec<u32>) -> bool {
    if is_growing(row) {
        return row
            .windows(2)
            .all(|x| x[0] < x[1] && x[0].abs_diff(x[1]) >= 1 && x[0].abs_diff(x[1]) <= 3);
    } else {
        return row
            .windows(2)
            .all(|x| x[0] > x[1] && x[0].abs_diff(x[1]) >= 1 && x[0].abs_diff(x[1]) <= 3);
    }
}

fn is_safe_with_dampener(row: &Vec<u32>) -> bool {
    for i in 0..row.len() {
        let mut pruned_row = row.clone();
        pruned_row.remove(i);
        if is_safe(&pruned_row) {
            return true;
        }
    }
    return false;
}
