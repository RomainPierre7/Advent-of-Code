use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "14_input.txt";

    let mut pos: Vec<(i32, i32)> = Vec::new();
    let mut vel: Vec<(i32, i32)> = Vec::new();

    let height = 103;
    let width = 101;
    let times = 100;

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

    for i in 0..pos.len() {
        pos[i].0 = (pos[i].0 + (vel[i].0 + width) * times) % width;
        pos[i].1 = (pos[i].1 + (vel[i].1 + height) * times) % height;
    }

    let count_top_left = pos
        .iter()
        .filter(|&&(x, y)| x < width / 2 && y < height / 2)
        .count();

    let count_top_right = pos
        .iter()
        .filter(|&&(x, y)| x > width / 2 && y < height / 2)
        .count();

    let count_bottom_left = pos
        .iter()
        .filter(|&&(x, y)| x < width / 2 && y > height / 2)
        .count();

    let count_bottom_right = pos
        .iter()
        .filter(|&&(x, y)| x > width / 2 && y > height / 2)
        .count();

    let res = count_top_left * count_top_right * count_bottom_left * count_bottom_right;

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
