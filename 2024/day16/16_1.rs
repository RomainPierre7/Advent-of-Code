use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "16_input.txt";

    let mut data: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut vec: Vec<char> = Vec::new();
                for c in content.chars() {
                    vec.push(c);
                }
                data.push(vec);
            }
        }
    }

    println!("{:?}", data);

    let mut distances = vec![vec![0; data.len()]; data.len()];

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..data.len() {
        for j in 0..data.len() {
            if data[i][j] == '#' {
                distances[i][j] = -1;
            } else if data[i][j] == 'S' {
                start = (i, j);
            } else if data[i][j] == 'E' {
                end = (i, j);
            }
        }
    }

    let mut queue = Vec::new();

    queue.push(start);

    while !queue.is_empty() {
        let (i, j) = queue.remove(0);

        if i > 0 && distances[i - 1][j] == 0 {
            distances[i - 1][j] = distances[i][j] + 1;
            queue.push((i - 1, j));
        }

        if i < data.len() - 1 && distances[i + 1][j] == 0 {
            distances[i + 1][j] = distances[i][j] + 1;
            queue.push((i + 1, j));
        }

        if j > 0 && distances[i][j - 1] == 0 {
            distances[i][j - 1] = distances[i][j] + 1;
            queue.push((i, j - 1));
        }

        if j < data.len() - 1 && distances[i][j + 1] == 0 {
            distances[i][j + 1] = distances[i][j] + 1;
            queue.push((i, j + 1));
        }
    }

    println!("{:?}", distances);

    let mut path = Vec::new();

    let mut i = end.0;
    let mut j = end.1;

    while i != start.0 || j != start.1 {
        path.push((i, j));

        if i > 0 && distances[i - 1][j] == distances[i][j] - 1 {
            i -= 1;
        } else if i < data.len() - 1 && distances[i + 1][j] == distances[i][j] - 1 {
            i += 1;
        } else if j > 0 && distances[i][j - 1] == distances[i][j] - 1 {
            j -= 1;
        } else if j < data.len() - 1 && distances[i][j + 1] == distances[i][j] - 1 {
            j += 1;
        }
    }

    path.push(start);

    println!("{:?}", path);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
