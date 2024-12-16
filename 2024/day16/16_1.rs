use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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

    for row in &data {
        println!("{:?}", row);
    }

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'S' {
                start = (i, j);
            } else if data[i][j] == 'E' {
                end = (i, j);
            }
        }
    }

    let res = dijkstra(&mut data, start, end);

    match res {
        Some((distance, path)) => {
            println!("Shortest path distance: {}", distance);
            println!("Path: {:?}", path);
            println!("Grid with path:");
            print_grid(&data);
        }
        None => println!("No path found"),
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Eq, PartialEq)]
struct State {
    position: (usize, usize),
    cost: usize,
    prev_direction: Option<(isize, isize)>, // Add previous direction to the state
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

fn is_changing_direction(prev_dir: Option<(isize, isize)>, new_dir: (isize, isize)) -> bool {
    match prev_dir {
        Some(prev) => prev != new_dir,
        None => false, // No previous direction, so no change
    }
}

fn dijkstra(
    data: &mut Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(usize, Vec<(usize, usize)>)> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new(); // To track the best path
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State {
        position: start,
        cost: 0,
        prev_direction: None, // Starting position, no previous direction
    });

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up

    while let Some(State {
        position,
        cost,
        prev_direction,
    }) = heap.pop()
    {
        if position == end {
            let mut path = Vec::new();
            let mut current = end;

            // Reconstruct the path by following the predecessors
            while let Some(&prev_pos) = prev.get(&current) {
                path.push(current);
                current = prev_pos;
            }
            path.push(start);
            path.reverse(); // Reverse the path to get it from start to end

            // Mark the path on the grid
            for &p in &path {
                if data[p.0][p.1] != 'S' && data[p.0][p.1] != 'E' {
                    data[p.0][p.1] = '*'; // Mark the path with '*'
                }
            }

            return Some((cost, path));
        }

        for &(dx, dy) in &directions {
            let (nx, ny) = (position.0 as isize + dx, position.1 as isize + dy);

            if nx < 0 || ny < 0 || nx as usize >= data.len() || ny as usize >= data[0].len() {
                continue;
            }

            let new_position = (nx as usize, ny as usize);

            // Check if the new position is a wall '#', and skip it if true
            if data[new_position.0][new_position.1] == '#' {
                continue;
            }

            let mut new_cost = cost + 1; // Assuming uniform cost for each move

            // Check if the direction is changing and add penalty if needed
            if is_changing_direction(prev_direction, (dx, dy)) {
                new_cost += 1000;
            }

            if let Some(&existing_cost) = dist.get(&new_position) {
                if new_cost >= existing_cost {
                    continue;
                }
            }

            dist.insert(new_position, new_cost);
            prev.insert(new_position, position); // Store the predecessor
            heap.push(State {
                position: new_position,
                cost: new_cost,
                prev_direction: Some((dx, dy)), // Store the direction taken
            });
        }
    }

    None
}

// Function to print the grid
fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
