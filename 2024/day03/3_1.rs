use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() -> io::Result<()> {
    let path = "3_input.txt";

    let mut data: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                data.push(content);
            }
        }
    }

    let mut res = 0;

    for s in data {
        res += parse_line(s);
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

fn parse_line(s: String) -> i32 {
    let mut res = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in re.captures_iter(&s) {
        let i = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let j = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        res += i * j;
    }
    return res;
}
