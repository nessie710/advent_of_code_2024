use std::fs::read_to_string;

fn main() {
    let filename = String::from("inputs/day7.txt");
    let file = read_to_string(filename).unwrap();
    let content: Vec<&str> = file.split("\n").filter(|x| !x.is_empty()).collect();
    let equations: Vec<Vec<&str>> = content.iter().map(|eq| eq.split(":").collect()).collect();
    let equations: Vec<Equation> = equations
        .iter()
        .map(|eq| Equation::new(eq.to_vec()))
        .collect();

    let sum: u64 = equations
        .iter()
        .filter(|eq| eq.is_correct(eq.terms.clone(), eq.result))
        .map(|eq| eq.result)
        .sum();

    println!("The sum is {:?}", sum);

    let sum: u64 = equations
        .iter()
        .filter(|eq| eq.is_correct_wtop(eq.terms.clone(), eq.result))
        .map(|eq| eq.result)
        .sum();

    println!("The sum now is {:?}", sum);
}

fn third_operator(x: u64, y: u64) -> u64 {
    let num = y.to_string();
    let num_digits = num.chars().count();
    return x * 10u64.pow(num_digits as u32) + y;
}

fn is_operable(res: u64, div: u64) -> bool {
    let div_str = div.to_string();
    let res_str = res.to_string();
    if res_str.ends_with(&div_str) {
        return true;
    } else {
        return false;
    }
}

fn inverse_top(res: u64, div: u64) -> u64 {
    let num = div.to_string();
    let num_digits = num.chars().count();
    return (res - div) / 10u64.pow(num_digits as u32);
}

#[derive(Debug)]
struct Equation {
    result: u64,
    terms: Vec<u64>,
}

impl Equation {
    fn new(info: Vec<&str>) -> Equation {
        let result = info[0].parse::<u64>().unwrap();
        let numbers = info[1]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|y| y.parse::<u64>().unwrap())
            .collect();
        return Equation {
            result: result,
            terms: numbers,
        };
    }

    fn is_correct(&self, nums: Vec<u64>, temp_result: u64) -> bool {
        let last = nums[nums.len() - 1];
        if nums.len() == 1 && last == temp_result {
            return true;
        } else if nums.len() == 1 && last != temp_result {
            return false;
        } else if temp_result % last != 0 && temp_result >= last {
            // println!("{}", temp_result - last);
            return self.is_correct(nums[0..nums.len() - 1].to_vec(), temp_result - last);
        } else if temp_result % last == 0 && temp_result >= last {
            // println!("Divide {}", temp_result / last);
            return self.is_correct(nums[0..nums.len() - 1].to_vec(), temp_result - last)
                || self.is_correct(nums[0..nums.len() - 1].to_vec(), temp_result / last);
        } else {
            return false;
        }
    }

    fn is_correct_wtop(&self, nums: Vec<u64>, temp_result: u64) -> bool {
        let last = nums[nums.len() - 1];
        if nums.len() == 1 && last == temp_result {
            return true;
        } else if nums.len() == 1 && last != temp_result {
            return false;
        } else if temp_result % last != 0 && temp_result >= last && !is_operable(temp_result, last)
        {
            return self.is_correct_wtop(nums[0..nums.len() - 1].to_vec(), temp_result - last);
        } else if temp_result % last != 0 && temp_result >= last && is_operable(temp_result, last) {
            return self.is_correct_wtop(nums[0..nums.len() - 1].to_vec(), temp_result - last)
                || self.is_correct_wtop(
                    nums[0..nums.len() - 1].to_vec(),
                    inverse_top(temp_result, last),
                );
        } else if temp_result % last == 0 && temp_result >= last && !is_operable(temp_result, last)
        {
            return self.is_correct_wtop(nums[0..nums.len() - 1].to_vec(), temp_result - last)
                || self.is_correct_wtop(nums[0..nums.len() - 1].to_vec(), temp_result / last);
        } else if temp_result % last == 0 && temp_result >= last && is_operable(temp_result, last) {
            return self.is_correct_wtop(nums[0..nums.len() - 1].to_vec(), temp_result - last)
                || self.is_correct_wtop(nums[0..nums.len() - 1].to_vec(), temp_result / last)
                || self.is_correct_wtop(
                    nums[0..nums.len() - 1].to_vec(),
                    inverse_top(temp_result, last),
                );
        } else {
            return false;
        }
    }
}
