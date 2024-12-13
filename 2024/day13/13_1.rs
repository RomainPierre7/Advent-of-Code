use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "13_input.txt";

    let mut button_a: Vec<(i32, i32)> = Vec::new();
    let mut button_b: Vec<(i32, i32)> = Vec::new();
    let mut prizes: Vec<(i32, i32)> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                if content.starts_with("Button A:") {
                    let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
                    if let Some(captures) = re.captures(&content) {
                        let x: i32 = captures[1].parse().unwrap();
                        let y: i32 = captures[2].parse().unwrap();
                        button_a.push((x, y));
                    }
                } else if content.starts_with("Button B:") {
                    let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
                    if let Some(captures) = re.captures(&content) {
                        let x: i32 = captures[1].parse().unwrap();
                        let y: i32 = captures[2].parse().unwrap();
                        button_b.push((x, y));
                    }
                } else if content.starts_with("Prize:") {
                    let re = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
                    if let Some(captures) = re.captures(&content) {
                        let x: i32 = captures[1].parse().unwrap();
                        let y: i32 = captures[2].parse().unwrap();
                        prizes.push((x, y));
                    }
                }
            }
        }
    }

    println!("{:?}", button_a);
    println!("{:?}", button_b);
    println!("{:?}", prizes);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
