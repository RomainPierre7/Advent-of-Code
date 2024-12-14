use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "13_input.txt";

    let mut button_a: Vec<(i64, i64)> = Vec::new();
    let mut button_b: Vec<(i64, i64)> = Vec::new();
    let mut prizes: Vec<(i64, i64)> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                if content.starts_with("Button A:") {
                    let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
                    if let Some(captures) = re.captures(&content) {
                        let x: i64 = captures[1].parse().unwrap();
                        let y: i64 = captures[2].parse().unwrap();
                        button_a.push((x, y));
                    }
                } else if content.starts_with("Button B:") {
                    let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
                    if let Some(captures) = re.captures(&content) {
                        let x: i64 = captures[1].parse().unwrap();
                        let y: i64 = captures[2].parse().unwrap();
                        button_b.push((x, y));
                    }
                } else if content.starts_with("Prize:") {
                    let re = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
                    if let Some(captures) = re.captures(&content) {
                        let x: i64 = captures[1].parse().unwrap();
                        let y: i64 = captures[2].parse().unwrap();
                        prizes.push((x + 10_000_000_000_000, y + 10_000_000_000_000));
                    }
                }
            }
        }
    }

    assert_eq!(button_a.len(), button_b.len());
    assert_eq!(button_b.len(), prizes.len());

    let mut res = 0;
    for i in 0..button_a.len() {
        let a1 = button_a[i].0;
        let a2 = button_a[i].1;
        let b1 = button_b[i].0;
        let b2 = button_b[i].1;
        let p1 = prizes[i].0;
        let p2 = prizes[i].1;

        if (a1 * b2 - a2 * b1) != 0 && (a1 * p2 - a2 * p1) % (a1 * b2 - a2 * b1) == 0 {
            let y = (a1 * p2 - a2 * p1) / (a1 * b2 - a2 * b1);
            if (a1 * b2 - a2 * b1) != 0 && (p1 - y * b1) % a1 == 0 {
                let x = (p1 - y * b1) / a1;
                res += x * 3 + y;
            }
        }
    }

    println! {"{res}"};

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
