use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "22_input.txt";
    let mut data: Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                for number in content.split_whitespace() {
                    if let Ok(num) = number.parse::<u64>() {
                        data.push(num);
                    } else {
                        eprintln!("Line format incorrect: {}", content);
                    }
                }
            }
        }
    }

    let mut ranges: HashMap<String, Vec<u64>> = HashMap::new();

    for &seed in &data {
        let mut current_seed = seed;
        let mut visited: HashSet<String> = HashSet::new();
        let mut changes: Vec<i32> = Vec::new();

        for _ in 0..2000 {
            let next_seed = new_secret(current_seed);
            changes.push((next_seed as i32 % 10) - (current_seed as i32 % 10));
            current_seed = next_seed;

            if changes.len() == 4 {
                let key = changes
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                if !visited.contains(&key) {
                    ranges
                        .entry(key.clone())
                        .or_insert_with(Vec::new)
                        .push(next_seed % 10);
                    visited.insert(key);
                }
                changes.remove(0);
            }
        }
    }

    let res = ranges
        .values()
        .map(|range_values| range_values.iter().sum::<u64>())
        .max()
        .unwrap_or(0);

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

fn new_secret(secret: u64) -> u64 {
    let mut value = secret;
    value = ((value << 6) ^ value) % 16777216;
    value = ((value >> 5) ^ value) % 16777216;
    value = ((value << 11) ^ value) % 16777216;
    value
}
