use nalgebra;
use std::fs::read_to_string;

fn main() {
    // Read and 1st part
    let file = read_to_string("inputs/day4.txt");
    let content: String = file.unwrap_or_else(|_| "".to_string());

    let rows: Vec<Vec<_>> = content
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split("")
                .filter(|y| !y.is_empty())
                .map(String::from)
                .collect()
        })
        .collect();

    let c = rows.len();
    let r = rows[0].len();

    let _m = nalgebra::DMatrix::from_iterator(r, c, rows.into_iter().flatten());
    let m = _m.transpose();
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut sum = 0;
    for d in directions {
        for i in 0..c {
            for j in 0..r {
                if m[(i, j)] == "X" && !conflicting_border(c, r, (i, j), d) {
                    if m[((i as i32 + d.0) as usize, (j as i32 + d.1) as usize)] == "M"
                        && m[((i as i32 + 2 * d.0) as usize, (j as i32 + 2 * d.1) as usize)] == "A"
                        && m[((i as i32 + 3 * d.0) as usize, (j as i32 + 3 * d.1) as usize)] == "S"
                    {
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("Sum is {}", sum);
    sum = 0;
    let m_config = vec![
        ((-1, -1), (-1, 1)),
        ((-1, -1), (1, -1)),
        ((1, 1), (-1, 1)),
        ((1, 1), (1, -1)),
    ];

    for conf in m_config {
        for i in 0..c {
            for j in 0..r {
                if m[(i, j)] == "A" && !on_border((i, j), c, r) {
                    if m[(
                        (i as i32 + conf.0 .0) as usize,
                        (j as i32 + conf.0 .1) as usize,
                    )] == "M"
                        && m[(
                            (i as i32 + conf.1 .0) as usize,
                            (j as i32 + conf.1 .1) as usize,
                        )] == "M"
                        && m[(
                            (i as i32 - conf.0 .0) as usize,
                            (j as i32 - conf.0 .1) as usize,
                        )] == "S"
                        && m[(
                            (i as i32 - conf.1 .0) as usize,
                            (j as i32 - conf.1 .1) as usize,
                        )] == "S"
                    {
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("Sum is now {}", sum);
}

fn conflicting_border(
    rows: usize,
    columns: usize,
    (i, j): (usize, usize),
    (x, y): (i32, i32),
) -> bool {
    if i as i32 + 3 * x < 0
        || i as i32 + 3 * x > columns as i32 - 1
        || j as i32 + 3 * y < 0
        || j as i32 + 3 * y > rows as i32 - 1
    {
        return true;
    } else {
        return false;
    }
}

fn on_border((i, j): (usize, usize), columns: usize, rows: usize) -> bool {
    if i == 0 || j == 0 || i == columns - 1 || j == rows - 1 {
        return true;
    } else {
        return false;
    }
}
