use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "20_input.txt";

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

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == 'S' {
                start = (i, j);
            } else if data[i][j] == 'E' {
                end = (i, j);
            }
        }
    }

    let base_distances = flood_fill(&data, start, end);

    let mut res = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '#' {
                let possible_cheats = get_possible_cheats(&base_distances, (i, j));
                for c in possible_cheats {
                    if c >= 100 {
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

fn flood_fill(data: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<i32>> {
    let mut distances: Vec<Vec<i32>> = vec![vec![-1; data[0].len()]; data.len()];
    let mut heap = BinaryHeap::new();
    heap.push((0, start));

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '#' {
                distances[i][j] = -2;
            }
        }
    }

    distances[start.0][start.1] = 0;

    while !heap.is_empty() {
        let (dist, pos) = heap.pop().unwrap();

        if pos == end {
            break;
        }

        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in directions {
            let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);

            if new_pos.0 < 0
                || new_pos.0 >= data.len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= data[0].len() as i32
            {
                continue;
            }

            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

            if distances[new_pos.0][new_pos.1] == -2 {
                continue;
            }

            let new_dist = dist + 1;

            if distances[new_pos.0][new_pos.1] == -1 || new_dist < distances[new_pos.0][new_pos.1] {
                distances[new_pos.0][new_pos.1] = new_dist;
                heap.push((new_dist, new_pos));
            }
        }
    }

    return distances;
}

fn get_possible_cheats(distances: &Vec<Vec<i32>>, pos: (usize, usize)) -> Vec<u32> {
    let mut res = Vec::new();

    let north = (pos.0 as i32 - 1, pos.1 as i32);
    let south = (pos.0 as i32 + 1, pos.1 as i32);
    let east = (pos.0 as i32, pos.1 as i32 + 1);
    let west = (pos.0 as i32, pos.1 as i32 - 1);

    // Check East - West cheat
    if west.1 >= 0 && east.1 < distances[0].len() as i32 {
        let distance1 = distances[east.0 as usize][east.1 as usize];
        let distance2 = distances[west.0 as usize][west.1 as usize];

        if distance1 >= 0 && distance2 >= 0 {
            res.push((distance1 - distance2).abs() as u32 - 2);
        }
    }

    // Check North - South cheat
    if north.0 >= 0 && south.0 < distances.len() as i32 {
        let distance1 = distances[north.0 as usize][north.1 as usize];
        let distance2 = distances[south.0 as usize][south.1 as usize];

        if distance1 >= 0 && distance2 >= 0 {
            res.push((distance1 - distance2).abs() as u32 - 2);
        }
    }

    return res;
}
