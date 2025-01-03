use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "6_input.txt";

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

    let guard_pos = find_guard_pos(&data);
    let guard_path_set = path_set(&data, &guard_pos);

    let mut res = 0;

    for (i, j) in guard_path_set {
        let mut data_try = data.clone();
        data_try[i as usize][j as usize] = '#';
        if has_cycle(&data_try, &guard_pos) {
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn to_usize(&self) -> usize {
        match *self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3,
        }
    }

    fn next(&self) -> Direction {
        match *self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

fn find_guard_pos(data: &[Vec<char>]) -> (usize, usize, Direction) {
    for (i, row) in data.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            let direction = match ch {
                '^' => Direction::N,
                '>' => Direction::E,
                'v' => Direction::S,
                '<' => Direction::W,
                _ => continue,
            };
            return (i, j, direction);
        }
    }
    return (0, 0, Direction::N);
}

fn path_set(
    data: &[Vec<char>],
    initial_pos: &(usize, usize, Direction),
) -> HashSet<(isize, isize)> {
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut i = initial_pos.0 as isize;
    let mut j = initial_pos.1 as isize;
    let mut dir = initial_pos.2.clone();

    let height = data.len() as isize;
    let width = data[0].len() as isize;

    let mut set: HashSet<(isize, isize)> = HashSet::new();

    set.insert((i, j));

    loop {
        let next_i = i + directions[dir.to_usize()].0;
        let next_j = j + directions[dir.to_usize()].1;

        if next_i < 0 || next_j < 0 || next_i >= height || next_j >= width {
            break;
        }

        if data[next_i as usize][next_j as usize] == '#' {
            dir = dir.next();
        } else {
            i = next_i;
            j = next_j;
            set.insert((i, j));
        }
    }

    return set;
}

fn has_cycle(data: &[Vec<char>], initial_pos: &(usize, usize, Direction)) -> bool {
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut i = initial_pos.0 as isize;
    let mut j = initial_pos.1 as isize;
    let mut dir = initial_pos.2.clone();

    let height = data.len() as isize;
    let width = data[0].len() as isize;

    let mut set: HashSet<(isize, isize, Direction)> = HashSet::new();

    set.insert((i, j, dir.clone()));

    loop {
        let next_i = i + directions[dir.to_usize()].0;
        let next_j = j + directions[dir.to_usize()].1;

        if next_i < 0 || next_j < 0 || next_i >= height || next_j >= width {
            break;
        }

        if data[next_i as usize][next_j as usize] == '#' {
            dir = dir.next();
        } else {
            i = next_i;
            j = next_j;
            let element = (i, j, dir.clone());
            if !set.contains(&element) {
                set.insert(element);
            } else {
                return true;
            }
        }
    }

    return false;
}
