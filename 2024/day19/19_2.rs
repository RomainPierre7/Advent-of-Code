use std::collections::HashMap;
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
    let mut cache: HashMap<String, u64> = HashMap::new();

    for design in &designs {
        res += possible_combinations(&patterns, design, &mut cache);
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

fn possible_combinations(
    patterns: &[String],
    design: &str,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(&cached_result) = cache.get(design) {
        return cached_result;
    }

    if design.is_empty() {
        return 1;
    }

    let mut res = 0;

    for pattern in patterns {
        if design.starts_with(pattern) {
            res += possible_combinations(patterns, &design[pattern.len()..], cache);
        }
    }

    cache.insert(design.to_string(), res);

    return res;
}
