use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "4_input.txt";

    let mut data: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut vec: Vec<char> = Vec::new();
                for letter in content.chars() {
                    vec.push(letter);
                }
                data.push(vec);
            }
        }
    }

    let mut res: i32 = 0;

    let height: usize = data.len();
    let width: usize = data[0].len();

    let directions = vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];

    let mut first = true;

    for i in 0..height {
        for j in 0..width {
            if data[i][j] == 'X' {
                for d in &directions {
                    if first && is_mas(&data, *d, i, j) {
                        first = false;
                        res += 1;
                    }
                }
            }
        }
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

fn is_mas(data: &Vec<Vec<char>>, direction: (i32, i32), mut i: usize, mut j: usize) -> bool {
    let (y, x) = direction;

    let word = "MAS";
    for c in word.chars() {
        println!("{c}");
        i = (i as i32 + y).max(0).min(data.len() as i32 - 1) as usize;
        j = (j as i32 + x).max(0).min(data[0].len() as i32 - 1) as usize;

        if data[i][j] != c {
            return false;
        }
    }

    return true;
}
