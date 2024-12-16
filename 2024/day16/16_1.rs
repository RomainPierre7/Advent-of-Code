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

    let res = dijkstra(&data, start, end);

    match res {
        Some(distance) => {
            println!("{}", distance);
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
    prev_direction: (isize, isize),
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

fn is_changing_direction(prev_dir: (isize, isize), new_dir: (isize, isize)) -> bool {
    return prev_dir != new_dir;
}

fn dijkstra(data: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State {
        position: start,
        cost: 0,
        prev_direction: (0, -1),
    });

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(State {
        position,
        cost,
        prev_direction,
    }) = heap.pop()
    {
        if position == end {
            return Some(cost);
        }

        for &(dx, dy) in &directions {
            let (nx, ny) = (position.0 as isize + dx, position.1 as isize + dy);

            if nx < 0 || ny < 0 || nx as usize >= data.len() || ny as usize >= data[0].len() {
                continue;
            }

            let new_position = (nx as usize, ny as usize);

            if data[new_position.0][new_position.1] == '#' {
                continue;
            }

            let mut new_cost = cost + 1;
            if is_changing_direction(prev_direction, (dx, dy)) {
                new_cost += 1000;
            }

            if let Some(&existing_cost) = dist.get(&new_position) {
                if new_cost >= existing_cost {
                    continue;
                }
            }

            dist.insert(new_position, new_cost);
            prev.insert(new_position, position);
            heap.push(State {
                position: new_position,
                cost: new_cost,
                prev_direction: (dx, dy),
            });
        }
    }

    return None;
}
