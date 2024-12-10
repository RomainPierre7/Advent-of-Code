use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "10_input.txt";

    let mut data: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut vec: Vec<i32> = Vec::new();
                for number in content.chars() {
                    if let Ok(num) = number.to_string().parse::<i32>() {
                        vec.push(num);
                    } else {
                        eprintln!("Line format incorrect: {}", content);
                    }
                }
                data.push(vec);
            }
        }
    }

    let mut starting_set: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 0 {
                starting_set.insert((i, j));
            }
        }
    }

    let mut res = 0;

    for (i, j) in starting_set {
        res += count_paths(i, j, 0, &data);
    }

    println!("{res}");

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_paths(i: usize, j: usize, value: i32, data: &Vec<Vec<i32>>) -> i32 {
    if value == 9 {
        return 1;
    }

    let steps = vec![
        (i as isize - 1, j as isize),
        (i as isize, j as isize + 1),
        (i as isize + 1, j as isize),
        (i as isize, j as isize - 1),
    ];

    let height = data.len();
    let width = data[0].len();

    let mut res = 0;

    for (new_i, new_j) in steps {
        if new_i >= 0 && new_j >= 0 && (new_i as usize) < height && (new_j as usize) < width {
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            if data[new_i][new_j] == value + 1 {
                res += count_paths(new_i, new_j, value + 1, data);
            }
        }
    }

    return res;
}
