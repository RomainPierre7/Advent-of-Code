use std::cmp::Ordering;
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

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            print!("{}", data[i][j]);
        }
        println!();
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

    println!("{:?}", start);
    println!("{:?}", end);

    let base_path = dijkstra(&data, start, end);
    let base_path_len = base_path.len() - 1;

    println!("{}", base_path_len);
    let mut res = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '#' {
                if has_possible_cheats(&data, (i, j)) {
                    let mut data_copy = data.clone();
                    data_copy[i][j] = '.';
                    let path = dijkstra(&data_copy, start, end);
                    let path_len = path.len() - 1;
                    if path_len + 100 <= base_path_len {
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

fn dijkstra(
    data: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let rows = data.len();
    let cols = data[0].len();

    let mut distances = vec![vec![usize::MAX; cols]; rows];
    let mut priority_queue = BinaryHeap::new();
    let mut predecessors = HashMap::new();

    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];

    distances[start.0][start.1] = 0;
    priority_queue.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = priority_queue.pop() {
        if position == end {
            break;
        }

        if cost > distances[position.0][position.1] {
            continue;
        }

        for dir in &directions {
            let next_row = position.0.wrapping_add(dir.0);
            let next_col = position.1.wrapping_add(dir.1);

            if next_row < rows && next_col < cols && data[next_row][next_col] != '#' {
                let next_cost = cost + 1;
                if next_cost < distances[next_row][next_col] {
                    distances[next_row][next_col] = next_cost;
                    predecessors.insert((next_row, next_col), position);
                    priority_queue.push(State {
                        cost: next_cost,
                        position: (next_row, next_col),
                    });
                }
            }
        }
    }

    let mut path = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        if let Some(&prev) = predecessors.get(&current) {
            current = prev;
        } else {
            break;
        }
    }
    path.push(start);
    path.reverse();
    path
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn has_possible_cheats(data: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    let rows = data.len();
    let cols = data[0].len();

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    for dir in &directions {
        let next_row = pos.0 as isize + dir.0;
        let next_col = pos.1 as isize + dir.1;

        if next_row >= 0 && next_row < rows as isize && next_col >= 0 && next_col < cols as isize {
            let next_row = next_row as usize;
            let next_col = next_col as usize;
            if data[next_row][next_col] == '.' {
                return true;
            }
        }
    }

    return false;
}
