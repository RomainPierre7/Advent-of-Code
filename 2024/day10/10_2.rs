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
        res += count_paths(i, j, &data, 0);
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

fn count_paths(i: usize, j: usize, data: &Vec<Vec<i32>>, res: i32) -> i32 {
    return res;
}
