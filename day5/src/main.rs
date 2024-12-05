use std::{cmp::Ordering, fs::read_to_string};

fn main() {
    // Part 1
    let file = read_to_string("inputs/day5.txt").unwrap();
    let rules_and_update: Vec<&str> = file.split("\n\n").collect();
    let rules: Vec<&str> = rules_and_update[0].split("\n").collect();

    let updates: Vec<&str> = rules_and_update[1]
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    let rules: Vec<Rule> = rules.iter().map(|rule| Rule::new(rule)).collect();
    let updates: Vec<Update> = updates.iter().map(|update| Update::new(update)).collect();

    let safe: u32 = updates
        .iter()
        .filter(|update| update.is_safe(&rules))
        .map(|up| up.middle())
        .sum();

    println!("The sum is {}", safe);

    // Part 2
    //
    let mut not_safe: Vec<Update> = updates
        .into_iter()
        .filter(|update| !update.is_safe(&rules))
        .collect();

    not_safe.iter_mut().for_each(|x| *x = x.ordered(&rules));
    let not_safe_sum: u32 = not_safe.iter().map(|x| x.middle()).sum();
    println!("The sum of not safe elements is {}", not_safe_sum);
}

#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32,
}

impl Rule {
    fn new(input: &str) -> Rule {
        let x: Vec<u32> = input
            .split("|")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        return Rule {
            first: x[0],
            second: x[1],
        };
    }
}

fn custom_ordering(a: &u32, b: &u32, rules: &Vec<Rule>) -> Ordering {
    let first_numbers: Vec<u32> = rules.iter().map(|rule| rule.first).collect();
    let second_numbers: Vec<u32> = rules.iter().map(|rule| rule.second).collect();

    let count_a: i32 = first_numbers.iter().filter(|x| *x == a).count() as i32
        - second_numbers.iter().filter(|x| *x == a).count() as i32;

    let count_b: i32 = first_numbers.iter().filter(|x| *x == b).count() as i32
        - second_numbers.iter().filter(|x| *x == b).count() as i32;

    return match count_a.cmp(&count_b) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => solve_conflict(a, b, rules),
        Ordering::Greater => Ordering::Greater,
    };
}

fn solve_conflict(a: &u32, b: &u32, rules: &Vec<Rule>) -> Ordering {
    for rule in rules {
        if a == &rule.first && b == &rule.second {
            return Ordering::Less;
        }
        if a == &rule.second && b == &rule.first {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}

#[derive(Debug)]
struct Update {
    data: Vec<u32>,
}

impl Update {
    fn new(update: &str) -> Update {
        let nums: Vec<u32> = update
            .split(",")
            .filter(|num| !num.is_empty())
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        return Update { data: nums };
    }

    fn len(&self) -> usize {
        return self.data.len();
    }

    fn middle(&self) -> u32 {
        return self.data[(self.len() / 2)];
    }

    fn is_safe(&self, rules: &Vec<Rule>) -> bool {
        for rule in rules {
            if self.data.contains(&rule.first) && self.data.contains(&rule.second) {
                let chunks: Vec<_> = self.data.split(|x| x == &rule.first).collect();
                if chunks[0].contains(&rule.second) {
                    return false;
                }
            }
        }

        return true;
    }

    fn ordered(&mut self, rules: &Vec<Rule>) -> Update {
        let mut mutated_array = self.data.clone();
        mutated_array.sort_by(|a, b| custom_ordering(a, b, &rules));
        return Update {
            data: mutated_array,
        };
    }
}
