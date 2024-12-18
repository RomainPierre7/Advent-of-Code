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

    for area in areas {
        res += area.len() * get_corner_count(&area);
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

fn get_corner_count(area: &HashSet<(usize, usize)>) -> usize {
    let directions = [
        (-1, 0),  // N
        (-1, 1),  // NE
        (0, 1),   // E
        (1, 1),   // SE
        (1, 0),   // S
        (1, -1),  // SW
        (0, -1),  // W
        (-1, -1), // NW
    ];

    let mut corner_count = 0;

    for &(i, j) in area {
        let mut has_neighbors = [false; 8];

        for (k, &(di, dj)) in directions.iter().enumerate() {
            let neighbor = (i.wrapping_add(di as usize), j.wrapping_add(dj as usize));
            if area.contains(&neighbor) {
                has_neighbors[k] = true;
            }
        }

        let n = has_neighbors[0];
        let ne = has_neighbors[1];
        let e = has_neighbors[2];
        let se = has_neighbors[3];
        let s = has_neighbors[4];
        let sw = has_neighbors[5];
        let w = has_neighbors[6];
        let nw = has_neighbors[7];

        if n && w && !nw {
            corner_count += 1;
        }
        if n && e && !ne {
            corner_count += 1;
        }
        if s && w && !sw {
            corner_count += 1;
        }
        if s && e && !se {
            corner_count += 1;
        }
        if !(n || w) {
            corner_count += 1;
        }
        if !(n || e) {
            corner_count += 1;
        }
        if !(s || w) {
            corner_count += 1;
        }
        if !(s || e) {
            corner_count += 1;
        }
    }

    return corner_count;
}
