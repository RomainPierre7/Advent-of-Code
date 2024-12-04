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

    for i in 0..height {
        for j in 0..width {
            if data[i][j] == 'X' {
                for d in &directions {
                    if is_mas(&data, *d, i, j) {
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
    let height: usize = data.len();
    let width: usize = data[0].len();

    let word = "MAS";

    for c in word.chars() {
        let new_i = i as i32 + y;
        let new_j = j as i32 + x;

        if new_i < 0 || new_j < 0 || new_i >= height as i32 || new_j >= width as i32 {
            return false;
        }

        i = new_i as usize;
        j = new_j as usize;

        if data[i][j] != c {
            return false;
        }
    }

    return true;
}
