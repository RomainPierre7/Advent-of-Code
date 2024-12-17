use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "16_input.txt";
    let mut data: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let vec: Vec<char> = content.chars().collect();
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

    let (min_cost, all_minimal_paths) = dijkstra(&data, start, end);

    match min_cost {
        Some(_cost) => {
            let distinct_positions: HashSet<(usize, usize)> = all_minimal_paths
                .iter()
                .flat_map(|path| path.clone())
                .collect();

            let res = distinct_positions.len();

            println!("{res}");
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

#[derive(Eq, PartialEq, Clone)]
struct State {
    position: (usize, usize),
    cost: usize,
    prev_direction: (isize, isize),
    path: Vec<(usize, usize)>,
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

fn dijkstra(
    data: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> (Option<usize>, Vec<Vec<(usize, usize)>>) {
    let mut dist = HashMap::new();
    let mut all_minimal_paths = Vec::new();
    let mut heap = BinaryHeap::new();

    dist.insert((start, (0, -1)), 0);
    heap.push(State {
        position: start,
        cost: 0,
        prev_direction: (0, 1), // Start facing east
        path: vec![start],
    });

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut min_cost = usize::MAX;

    while let Some(State {
        position,
        cost,
        prev_direction,
        path,
    }) = heap.pop()
    {
        if position == end {
            match min_cost.cmp(&cost) {
                Ordering::Greater => {
                    min_cost = cost;
                    all_minimal_paths.clear();
                    all_minimal_paths.push(path);
                }
                Ordering::Equal => {
                    all_minimal_paths.push(path);
                }
                Ordering::Less => break,
            }
            continue;
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
            if prev_direction != (dx, dy) {
                new_cost += 1000;
                let is_opposite_direction =
                    prev_direction == (-dx, dy) || prev_direction == (dx, -dy);
                if is_opposite_direction {
                    new_cost += 1000;
                }
            }

            let new_key = (new_position, (dx, dy));

            if let Some(&existing_cost) = dist.get(&new_key) {
                if new_cost > existing_cost {
                    continue;
                }
            }

            dist.insert(new_key, new_cost);

            let mut new_path = path.clone();
            new_path.push(new_position);

            heap.push(State {
                position: new_position,
                cost: new_cost,
                prev_direction: (dx, dy),
                path: new_path,
            });
        }
    }

    if min_cost == usize::MAX {
        (None, Vec::new())
    } else {
        (Some(min_cost), all_minimal_paths)
    }
}
