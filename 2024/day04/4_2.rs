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

    for i in 0..height {
        for j in 0..width {
            if data[i][j] == 'A' {
                if is_x_mas(&data, i, j) {
                    res += 1;
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

fn is_x_mas(data: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let height = data.len() as i32;
    let width = data[0].len() as i32;

    const DIRECTIONS: [(i32, i32); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

    let positions = DIRECTIONS
        .iter()
        .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
        .collect::<Vec<_>>();

    for &(y, x) in &positions {
        if y < 0 || x < 0 || y >= height || x >= width {
            return false;
        }
    }

    let [nw, ne, se, sw] = [positions[0], positions[1], positions[2], positions[3]];

    let nw_char = data[nw.0 as usize][nw.1 as usize];
    let ne_char = data[ne.0 as usize][ne.1 as usize];
    let se_char = data[se.0 as usize][se.1 as usize];
    let sw_char = data[sw.0 as usize][sw.1 as usize];

    return ((nw_char == 'M' && se_char == 'S') || (nw_char == 'S' && se_char == 'M'))
        && ((ne_char == 'M' && sw_char == 'S') || (ne_char == 'S' && sw_char == 'M'));
}
