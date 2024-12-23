use regex::Regex;
use std::fs::read_to_string;

fn main() {
    // Read and 1st part
    let file = read_to_string("inputs/day13.txt").unwrap();
    let content: Vec<String> = file
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|y| String::from(y))
        .collect();

    let re1 = Regex::new(r"Button A: X\+(?<num1>\d{1,3})\, Y\+(?<num2>\d{1,3})").unwrap();
    let re2 = Regex::new(r"Button B: X\+(?<num1>\d{1,3})\, Y\+(?<num2>\d{1,3})").unwrap();
    let re3 = Regex::new(r"Prize: X\=(?<num1>\d{1,9})\, Y\=(?<num2>\d{1,9})").unwrap();

    let mut sum = 0.0;
    for config in &content {
        let capture_a: Vec<_> = re1.captures_iter(config).collect();
        let capture_b: Vec<_> = re2.captures_iter(config).collect();
        let capture_prize: Vec<_> = re3.captures_iter(config).collect();
        let xa = capture_a[0]["num1"].parse::<i32>().unwrap();
        let ya = capture_a[0]["num2"].parse::<i32>().unwrap();
        let xb = capture_b[0]["num1"].parse::<i32>().unwrap();
        let yb = capture_b[0]["num2"].parse::<i32>().unwrap();
        let xtarget = capture_prize[0]["num1"].parse::<i32>().unwrap();
        let ytarget = capture_prize[0]["num2"].parse::<i32>().unwrap();

        // xa*k1 + xb*k2 = xtarget
        // ya*k1 + yb*k2 = ytarget

        let determinant = (xa * yb) - (xb * ya);
        //println!("Determinant: {}", determinant);
        if determinant != 0 {
            let k1: f32 = ((xtarget * yb) - (xb * ytarget)) as f32 / determinant as f32;
            let k2: f32 = ((xa * ytarget) - (xtarget * ya)) as f32 / determinant as f32;
            if k1 >= 0.0 && k2 >= 0.0 && k1.fract() == 0.0 && k2.fract() == 0.0 {
                //println!("k1: {} k2: {}", k1, k2);
                sum += k1 * 3.0 + k2;
            }
        } else {
            if (xtarget * yb - xb * ytarget) == 0 && (xa * ytarget - xtarget * ya) == 0 {
                continue;
            } else {
                continue;
            }
        }
    }

    println!("The sum is {}", sum);

    let file = read_to_string("inputs/day13.txt").unwrap();
    let content: Vec<String> = file
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|y| String::from(y))
        .collect();

    let re1 = Regex::new(r"Button A: X\+(?<num1>\d{1,3})\, Y\+(?<num2>\d{1,3})").unwrap();
    let re2 = Regex::new(r"Button B: X\+(?<num1>\d{1,3})\, Y\+(?<num2>\d{1,3})").unwrap();
    let re3 = Regex::new(r"Prize: X\=(?<num1>\d{1,9})\, Y\=(?<num2>\d{1,9})").unwrap();

    let mut sum = 0.0;
    for config in &content {
        let capture_a: Vec<_> = re1.captures_iter(config).collect();
        let capture_b: Vec<_> = re2.captures_iter(config).collect();
        let capture_prize: Vec<_> = re3.captures_iter(config).collect();
        let xa = capture_a[0]["num1"].parse::<i64>().unwrap();
        let ya = capture_a[0]["num2"].parse::<i64>().unwrap();
        let xb = capture_b[0]["num1"].parse::<i64>().unwrap();
        let yb = capture_b[0]["num2"].parse::<i64>().unwrap();
        let xtarget = capture_prize[0]["num1"].parse::<i64>().unwrap() + 10000000000000;
        let ytarget = capture_prize[0]["num2"].parse::<i64>().unwrap() + 10000000000000;

        // xa*k1 + xb*k2 = xtarget
        // ya*k1 + yb*k2 = ytarget

        let determinant = (xa * yb) - (xb * ya);
        println!("Determinant: {}", determinant);
        if determinant != 0 {
            let k1: f64 = ((xtarget * yb) - (xb * ytarget)) as f64 / determinant as f64;
            let k2: f64 = ((xa * ytarget) - (xtarget * ya)) as f64 / determinant as f64;
            if k1 >= 0.0 && k2 >= 0.0 && k1.fract() == 0.0 && k2.fract() == 0.0 {
                println!("k1: {} k2: {}", k1, k2);
                sum += k1 * 3.0 + k2;
            }
        } else {
            if (xtarget * yb - xb * ytarget) == 0 && (xa * ytarget - xtarget * ya) == 0 {
                continue;
            } else {
                continue;
            }
        }
    }

    println!("The sum now is {}", sum);
}
