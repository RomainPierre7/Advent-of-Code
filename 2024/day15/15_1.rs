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
                        vec.push(c);
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
        let mut o_pos = new_robot;
        while grid[o_pos.0][o_pos.1] != '#' {
            if grid[o_pos.0][o_pos.1] == '.' {
                grid[new_robot.0][new_robot.1] = '@';
                grid[robot.0][robot.1] = '.';
                grid[o_pos.0][o_pos.1] = 'O';
                robot.0 = new_robot.0;
                robot.1 = new_robot.1;
                break;
            }

            o_pos.0 = (o_pos.0 as isize + dir.0) as usize;
            o_pos.1 = (o_pos.1 as isize + dir.1) as usize;
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
