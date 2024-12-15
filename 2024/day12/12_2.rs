use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "12_input.txt";

    let mut data: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let vec: Vec<char> = content.chars().collect();
                data.push(vec);
            }
        }
    }

    let height = data.len();
    let width = data[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut areas: Vec<HashSet<(usize, usize)>> = Vec::new();

    for i in 0..height {
        for j in 0..width {
            if !visited.contains(&(i, j)) {
                let area = get_area(i, j, &data, &mut visited);
                areas.push(area);
            }
        }
    }

    let mut res = 0;

    for area in &areas {
        if let Some(&(i, j)) = area.iter().next() {
            println!("{}: {}", data[i][j], get_corner_count(&area));
            res += area.len() * get_corner_count(&area);
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

fn get_area(
    start_i: usize,
    start_j: usize,
    data: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut area: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    let target_value = data[start_i][start_j];
    queue.push_back((start_i, start_j));
    visited.insert((start_i, start_j));

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((i, j)) = queue.pop_front() {
        area.insert((i, j));

        for (di, dj) in &directions {
            let new_i = i.wrapping_add(*di as usize);
            let new_j = j.wrapping_add(*dj as usize);

            if new_i < data.len()
                && new_j < data[0].len()
                && !visited.contains(&(new_i, new_j))
                && data[new_i][new_j] == target_value
            {
                visited.insert((new_i, new_j));
                queue.push_back((new_i, new_j));
            }
        }
    }

    return area;
}

enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::NorthEast => (1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
            Direction::NorthWest => (-1, -1),
        }
    }
}

fn add_pos(p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn is_adj(pos: (i32, i32), delta: (i32, i32), area: &HashSet<(usize, usize)>) -> bool {
    let p2 = add_pos(pos, delta);
    area.contains(&(p2.0 as usize, p2.1 as usize))
}

fn get_corner_count(area: &HashSet<(usize, usize)>) -> usize {
    let mut corners = 0;

    for &(x, y) in area {
        let pos = (x as i32, y as i32);

        let north = is_adj(pos, Direction::North.delta(), area);
        let south = is_adj(pos, Direction::South.delta(), area);
        let east = is_adj(pos, Direction::East.delta(), area);
        let west = is_adj(pos, Direction::West.delta(), area);

        let northeast = is_adj(pos, Direction::NorthEast.delta(), area);
        let southeast = is_adj(pos, Direction::SouthEast.delta(), area);
        let southwest = is_adj(pos, Direction::SouthWest.delta(), area);
        let northwest = is_adj(pos, Direction::NorthWest.delta(), area);

        // Check corners for this cell
        if (north && east) && !northeast {
            corners += 1;
        }
        if (south && east) && !southeast {
            corners += 1;
        }
        if (south && west) && !southwest {
            corners += 1;
        }
        if (north && west) && !northwest {
            corners += 1;
        }
    }

    corners
}
