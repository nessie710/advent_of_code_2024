use std::fs::read_to_string;

fn main() {
    let path = "inputs/day9.txt";
    let file = read_to_string(path).unwrap();
    let content: Vec<Vec<&str>> = file
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|y| y.split("").filter(|z| !z.is_empty()).collect())
        .collect();
    let content: Vec<&str> = content.into_iter().flatten().collect();

    let mut expanded = construct_ids(content);

    let data: Vec<&String> = expanded.iter().filter(|x| *x != ".").collect();
    let length = expanded.len();
    println!("Expanded: {:?}", expanded.join(""));
    for i in 0..data.len() {
        if expanded[i] == "." {
            let pos = expanded.iter().rev().position(|x| x != ".").unwrap();
            expanded.swap(i, length - 1 - pos);
        }
    }
    println!("Expanded: {:?}", expanded.join(""));

    let mut sum = 0;
    for (i, str) in expanded.iter().enumerate() {
        sum += i * str.parse::<usize>().unwrap_or(0);
    }
    println!("The sum is {}", sum);

    // Part 2
    //
    let content: Vec<Vec<&str>> = file
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|y| y.split("").filter(|z| !z.is_empty()).collect())
        .collect();
    let content: Vec<&str> = content.into_iter().flatten().collect();

    let mut expanded = construct_ids(content);
    println!("Expanded: {:?}", expanded.join(""));
    let length = expanded.len();
    let mut i = length - 1;

    loop {
        let pos = &expanded[0..i + 1]
            .iter()
            .rev()
            .position(|x| x != ".")
            .unwrap();
        if i > 1 {
            //println!("Vec {:?}", &expanded[0..i + 1]);
            let temp_len = i;
            while expanded[i - 1 - pos] == expanded[i - pos] {
                i -= 1;
            }
            i -= pos;
            let chunk_length = temp_len - pos - i + 1;
            //println!("Chunk len {}", chunk_length);
            let mut spaces: Vec<usize> = vec![];
            for j in 0..i {
                if expanded[j] == "." {
                    // println!("{}", j);
                    let mut k = j;
                    while expanded[k] == "." {
                        k += 1;
                    }
                    let space_len = k - j;
                    spaces.push(space_len);
                    //println!("Space len {}", space_len);
                    if space_len >= chunk_length {
                        for h in 0..chunk_length {
                            expanded.swap(j + h, temp_len - h - pos);
                        }
                        break;
                    }
                }
            }
            if spaces.iter().all(|&x| x == 0) {
                break;
            }
            i -= 1;
        } else {
            break;
        }
    }

    println!("Expanded: {:?}", expanded.join(""));

    let mut sum = 0;
    for (i, str) in expanded.iter().enumerate() {
        sum += i * str.parse::<usize>().unwrap_or(0);
    }
    println!("The sum is now {}", sum);
}

fn construct_ids(s: Vec<&str>) -> Vec<String> {
    let nums: Vec<u8> = s.iter().map(|x| x.parse::<u8>().unwrap()).collect();
    let mut v: Vec<String> = vec![];
    for i in 0..nums.len() {
        for _ in 0..nums[i] {
            if i % 2 == 0 {
                v.push((i / 2).to_string());
            } else {
                v.push(".".to_string());
            }
        }
    }
    return v;
}
