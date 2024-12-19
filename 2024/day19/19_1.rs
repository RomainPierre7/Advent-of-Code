use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "19_input.txt";

    let mut patterns: Vec<String> = Vec::new();
    let mut designs: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.enumerate() {
            if let Ok(content) = line {
                if content.is_empty() {
                    continue;
                }
                if i == 0 {
                    for pattern in content.split(',') {
                        patterns.push(pattern.trim().to_string());
                    }
                } else {
                    for design in content.split(',') {
                        designs.push(design.trim().to_string());
                    }
                }
            }
        }
    }

    let mut res = 0;

    for design in &designs {
        if is_possible(&patterns, design) {
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

fn is_possible(patterns: &Vec<String>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            if is_possible(patterns, &design[pattern.len()..]) {
                return true;
            }
        }
    }

    return false;
}
