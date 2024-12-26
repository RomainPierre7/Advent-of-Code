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

    for i in 0..data.len() {
        data[i] = new_secret(data[i], 2000);
    }

    let res: u64 = data.iter().sum();

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

fn new_secret(secret: u64, loop_size: u64) -> u64 {
    let mut value = secret;

    for _ in 0..loop_size {
        value = ((value << 6) ^ value) % 16777216;
        value = ((value >> 5) ^ value) % 16777216;
        value = ((value << 11) ^ value) % 16777216;
    }

    return value;
}
