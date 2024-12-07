use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "7_input.txt";

    let mut data: Vec<(u64, Vec<u64>)> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut parts = content.split(':');
                if let (Some(part1), Some(part2)) = (parts.next(), parts.next()) {
                    let mut vec: Vec<u64> = Vec::new();
                    let target: u64 = part1.parse().expect("Failed to convert");
                    for number in part2.split_whitespace() {
                        if let Ok(num) = number.parse::<u64>() {
                            vec.push(num);
                        } else {
                            eprintln!("Line format incorrect: {}", content);
                        }
                    }
                    data.push((target, vec));
                } else {
                    eprintln!("Line format incorrect: {}", content);
                }
            }
        }
    }

    let mut res: u64 = 0;

    for (target, numbers) in data {
        if can_reach_target(target, &numbers[1..], numbers[0]) {
            res += target;
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

fn can_reach_target(target: u64, numbers: &[u64], current: u64) -> bool {
    if numbers.is_empty() || current > target {
        return current == target;
    }

    let first = numbers[0];
    let rest = &numbers[1..];

    return can_reach_target(target, rest, current + first)
        || can_reach_target(target, rest, current * first);
}
