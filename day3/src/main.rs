use regex::Regex;
use std::fs::read_to_string;

fn main() {
    // Read and 1st part
    let file = read_to_string("inputs/day3.txt");
    let content: String = file.unwrap_or_else(|_| "".to_string());

    let re = Regex::new(r"mul\((?<num1>\d{1,3})\,(?<num2>\d{1,3})\)").unwrap();
    let capture: Vec<_> = re.captures_iter(&content).collect();
    let mut sum = 0;
    for mul in capture {
        sum += mul["num1"].parse::<u32>().unwrap() * mul["num2"].parse::<u32>().unwrap();
    }
    println!("The sum is {}", sum);

    // 2nd part
    let dont_split: Vec<&str> = content.split("don't()").collect();
    let do_split: Vec<Vec<&str>> = dont_split
        .iter()
        .map(|x| x.split("do()").collect())
        .collect();

    let mut sum: u32 = 0;
    for (i, substr) in do_split.iter().enumerate() {
        for (j, dos) in substr.iter().enumerate() {
            if i == 0 || j != 0 {
                let capture: Vec<_> = re.captures_iter(&dos).collect();
                for mul in capture {
                    sum +=
                        mul["num1"].parse::<u32>().unwrap() * mul["num2"].parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("The sum now is {}", sum);
}
