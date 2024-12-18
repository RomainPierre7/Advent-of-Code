use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "18_input.txt";

    let mut data: Vec<(i32, i32)> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        data.push((x, y));
                    } else {
                        eprintln!("Failed to parse line: {}", content);
                    }
                } else {
                    eprintln!("Invalid format: {}", content);
                }
            }
        }
    }

    let height = 71;
    let width = 71;
    let max_fallen_bytes = 1024;

    let mut grid = vec![vec![-1; width as usize]; height as usize];

    for (i, (x, y)) in data.iter().enumerate() {
        if i >= max_fallen_bytes as usize {
            break;
        }
        grid[*y as usize][*x as usize] = -2;
    }

    let start = (0, 0);
    let end = (height - 1, width - 1);
    grid[start.1 as usize][start.0 as usize] = 0;

    let mut next = Vec::new();
    next.push(start);

    while !next.is_empty() {
        let mut new_next = Vec::new();
        for (x, y) in next.iter() {
            if *x == end.0 && *y == end.1 {
                break;
            }

            let next_value = grid[*y as usize][*x as usize] + 1;
            if *x > 0 && grid[*y as usize][(*x - 1) as usize] == -1 {
                grid[*y as usize][(*x - 1) as usize] = next_value;
                new_next.push((*x - 1, *y));
            }
            if *x < width - 1 && grid[*y as usize][(*x + 1) as usize] == -1 {
                grid[*y as usize][(*x + 1) as usize] = next_value;
                new_next.push((*x + 1, *y));
            }
            if *y > 0 && grid[(*y - 1) as usize][*x as usize] == -1 {
                grid[(*y - 1) as usize][*x as usize] = next_value;
                new_next.push((*x, *y - 1));
            }
            if *y < height - 1 && grid[(*y + 1) as usize][*x as usize] == -1 {
                grid[(*y + 1) as usize][*x as usize] = next_value;
                new_next.push((*x, *y + 1));
            }
        }
        next = new_next;
    }

    let res = grid[end.0 as usize][end.1 as usize];

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
