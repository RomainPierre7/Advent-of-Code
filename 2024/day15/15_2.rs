use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "15_input.txt";

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                if content.starts_with('#') {
                    let mut vec: Vec<char> = Vec::new();
                    for c in content.chars() {
                        if c == '@' {
                            vec.push(c);
                            vec.push('.');
                        } else if c == 'O' {
                            vec.push('[');
                            vec.push(']');
                        } else {
                            vec.push(c);
                            vec.push(c);
                        }
                    }
                    grid.push(vec);
                } else {
                    for c in content.chars() {
                        instructions.push(c);
                    }
                }
            }
        }
    }

    let heigth = grid.len();
    let width = grid[0].len();

    println!("{:?}", instructions);
    print(grid.clone());
    println!();

    let mut robot = (0, 0);

    for i in 0..heigth {
        for j in 0..width {
            if grid[i][j] == '@' {
                robot = (i, j);
            }
        }
    }

    for instr in instructions {
        make_step(&mut grid, &mut robot, instr);
        println!("{instr}");
        print(grid.clone());
        println!();
    }

    let res = gps(grid);

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

fn make_step(grid: &mut Vec<Vec<char>>, robot: &mut (usize, usize), instr: char) {
    let mut dir = (0, 0);

    match instr {
        '>' => dir = (0, 1),
        'v' => dir = (1, 0),
        '<' => dir = (0, -1),
        '^' => dir = (-1, 0),
        _ => {}
    }

    let new_robot = (
        (robot.0 as isize + dir.0) as usize,
        (robot.1 as isize + dir.1) as usize,
    );

    if grid[new_robot.0][new_robot.1] == '#' {
        return;
    } else if grid[new_robot.0][new_robot.1] == '.' {
        grid[new_robot.0][new_robot.1] = '@';
        grid[robot.0][robot.1] = '.';
        robot.0 = new_robot.0;
        robot.1 = new_robot.1;
    } else {
        let linked_boxes = get_linked_boxes(*robot, dir, grid);
        if bloc_is_movable(&linked_boxes, dir, grid) {
            move_bloc(*robot, &linked_boxes, dir, grid);
            grid[new_robot.0][new_robot.1] = '@';
            grid[robot.0][robot.1] = '.';
            robot.0 = new_robot.0;
            robot.1 = new_robot.1;
        }
    }
}

fn gps(grid: Vec<Vec<char>>) -> u64 {
    let mut res: u64 = 0;

    let heigth = grid.len();
    let width = grid[0].len();

    for i in 0..heigth {
        for j in 0..width {
            if grid[i][j] == 'O' {
                res += 100 * i as u64 + j as u64;
            }
        }
    }

    return res;
}

fn print(grid: Vec<Vec<char>>) {
    for i in 0..grid.len() {
        println!("{:?}", grid[i]);
    }
}

fn get_linked_boxes(
    robot: (usize, usize),
    dir: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    let mut linked_boxes: HashSet<(usize, usize)> = HashSet::new();

    let mut last_added: HashSet<(usize, usize)> = HashSet::new();
    last_added.insert(robot);

    fn rec(
        last_added: HashSet<(usize, usize)>,
        linked_boxes: &mut HashSet<(usize, usize)>,
        dir: (isize, isize),
        grid: &Vec<Vec<char>>,
    ) {
        let mut new_added: HashSet<(usize, usize)> = HashSet::new();

        for pos in last_added {
            let new_pos = (
                (pos.0 as isize + dir.0) as usize,
                (pos.1 as isize + dir.1) as usize,
            );
            if grid[new_pos.0][new_pos.1] == '[' {
                linked_boxes.insert(new_pos);
                linked_boxes.insert((new_pos.0, new_pos.1 + 1));
                new_added.insert(new_pos);
            } else if grid[new_pos.0][new_pos.1] == ']' {
                linked_boxes.insert(new_pos);
                linked_boxes.insert((new_pos.0, new_pos.1 - 1));
                new_added.insert(new_pos);
            }
        }

        if new_added.len() > 0 {
            rec(new_added, linked_boxes, dir, grid);
        }
    }

    rec(last_added, &mut linked_boxes, dir, grid);

    //println!("Found {} linked boxes", linked_boxes.len());

    return linked_boxes;
}

fn bloc_is_movable(
    linked_boxes: &HashSet<(usize, usize)>,
    dir: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> bool {
    for pos in linked_boxes {
        let new_pos = (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        );
        if grid[new_pos.0][new_pos.1] == '#' {
            println!("Bloc is not movable");
            return false;
        }
    }
    println!("Bloc is movable");
    return true;
}

fn move_bloc(
    robot: (usize, usize),
    linked_boxes: &HashSet<(usize, usize)>,
    dir: (isize, isize),
    grid: &mut Vec<Vec<char>>,
) {
    let grid_copy = grid.clone();

    for pos in linked_boxes {
        let new_pos = (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        );
        grid[new_pos.0][new_pos.1] = grid_copy[pos.0][pos.1];
        grid[pos.0][pos.1] = '.';
    }
}
