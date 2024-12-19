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
        res += possible_combinations(&patterns, design);
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

fn possible_combinations(patterns: &Vec<String>, design: &str) -> u32 {
    let mut res = 0;

    if design.is_empty() {
        return 1;
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            res += possible_combinations(patterns, &design[pattern.len()..]);
        }
    }

    return res;
}
