use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "20_input.txt";

    let mut data: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut vec: Vec<i32> = Vec::new();
                for number in content.split_whitespace() {
                    if let Ok(num) = number.parse::<i32>() {
                        vec.push(num);
                    } else {
                        eprintln!("Line format incorrect: {}", content);
                    }
                }
                data.push(vec);
            }
        }
    }

    println!("{:?}", data);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

