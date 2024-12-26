use std::collections::{BinaryHeap, HashMap};
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
    let best_path = get_path_from_flood_fill(&base_distances, start, end);

    let possible_cheats = get_possible_cheats(&base_distances, &best_path);

    let mut res = 0;
    for c in possible_cheats.values() {
        if *c >= 100 {
            res += 1;
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

fn get_path_from_flood_fill(
    distances: &Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut best_path: Vec<(usize, usize)> = Vec::new();
    best_path.push(end);
    let mut current = end;
    let mut current_dist = distances[end.0][end.1];
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    while current != start {
        for dir in &directions {
            let new_pos = (current.0 as i32 + dir.0, current.1 as i32 + dir.1);
            if new_pos.0 < 0
                || new_pos.0 >= distances.len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= distances[0].len() as i32
            {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if distances[new_pos.0][new_pos.1] == current_dist - 1 {
                best_path.push(new_pos);
                current = new_pos;
                current_dist -= 1;
                break;
            }
        }
    }
    return best_path;
}

fn get_possible_cheats(
    distances: &Vec<Vec<i32>>,
    path: &Vec<(usize, usize)>,
) -> HashMap<((usize, usize), (usize, usize)), u32> {
    fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> u32 {
        return ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as u32;
    }

    let mut res: HashMap<((usize, usize), (usize, usize)), u32> = HashMap::new();

    for &start in path {
        for &end in path {
            let manhattan_dist = manhattan_distance(start, end);
            if manhattan_dist <= 20 {
                let gain = distances[end.0][end.1] - distances[start.0][start.1];
                if gain <= manhattan_dist as i32 {
                    continue;
                }
                res.insert((start, end), gain as u32 - manhattan_dist as u32);
            }
        }
    }

    return res;
}
