use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "14_input.txt";

    let mut pos: Vec<(i32, i32)> = Vec::new();
    let mut vel: Vec<(i32, i32)> = Vec::new();

    let height = 103;
    let width = 101;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut robot = content.split_whitespace();
                let robot_pos: Vec<_> = robot
                    .next()
                    .map(|s| &s[2..])
                    .unwrap_or("")
                    .split(',')
                    .collect();
                let robot_vel: Vec<_> = robot
                    .next()
                    .map(|s| &s[2..])
                    .unwrap_or("")
                    .split(',')
                    .collect();

                let (px, py): (i32, i32) = (
                    robot_pos[0].parse().expect("Failed to convert"),
                    robot_pos[1].parse().expect("Failed to convert"),
                );
                let (vx, vy): (i32, i32) = (
                    robot_vel[0].parse().expect("Failed to convert"),
                    robot_vel[1].parse().expect("Failed to convert"),
                );

                pos.push((px, py));
                vel.push((vx, vy));
            }
        }
    }

    let mut res = 1;
    loop {
        if res > height * width {
            println!("No solution found");
            break;
        }
        for i in 0..pos.len() {
            pos[i].0 = (pos[i].0 + vel[i].0 + width) % width;
            pos[i].1 = (pos[i].1 + vel[i].1 + height) % height;
        }
        if flood_fill(&pos, width, height, 50) {
            display_robots(&pos, width, height);
            break;
        }
        res += 1;
    }

    println!("{}", res);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn display_robots(pos: &Vec<(i32, i32)>, width: i32, height: i32) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for &(x, y) in pos {
        if x >= 0 && x < width && y >= 0 && y < height {
            grid[y as usize][x as usize] = '#';
        }
    }

    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn flood_fill(pos: &Vec<(i32, i32)>, width: i32, height: i32, n: usize) -> bool {
    fn flood_fill_from_pos(
        pos: &Vec<(i32, i32)>,
        start_x: i32,
        start_y: i32,
        width: i32,
        height: i32,
        n: usize,
    ) -> bool {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut count = 0;

        fn dfs(
            x: i32,
            y: i32,
            pos: &Vec<(i32, i32)>,
            width: i32,
            height: i32,
            visited: &mut HashSet<(i32, i32)>,
            count: &mut usize,
        ) {
            if x < 0
                || x >= width
                || y < 0
                || y >= height
                || visited.contains(&(x, y))
                || !pos.contains(&(x, y))
            {
                return;
            }

            visited.insert((x, y));
            *count += 1;

            dfs(x, y - 1, pos, width, height, visited, count); // Up
            dfs(x + 1, y - 1, pos, width, height, visited, count); // Up-right
            dfs(x + 1, y, pos, width, height, visited, count); // Right
            dfs(x + 1, y + 1, pos, width, height, visited, count); // Down-right
            dfs(x, y + 1, pos, width, height, visited, count); // Down
            dfs(x - 1, y + 1, pos, width, height, visited, count); // Down-left
            dfs(x - 1, y, pos, width, height, visited, count); // Left
            dfs(x - 1, y - 1, pos, width, height, visited, count); // Up-left
        }

        dfs(
            start_x,
            start_y,
            pos,
            width,
            height,
            &mut visited,
            &mut count,
        );

        return count >= n;
    }

    for &(x, y) in pos {
        if flood_fill_from_pos(pos, x, y, width, height, n) {
            return true;
        }
    }

    return false;
}
