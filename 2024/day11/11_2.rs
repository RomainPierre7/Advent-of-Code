use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "11_input.txt";

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

    for i in 0..25 {
        blink(&mut data);
        println!("{i}");
    }

    let res = data.len();

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

fn blink(data: &mut Vec<u64>) {
    for i in 0..data.len() {
        if data[i] == 0 {
            data[i] = 1
        } else if data[i].to_string().len() % 2 == 0 {
            let str = data[i].to_string();
            let str1 = &str[..str.len() / 2];
            let str2 = &str[str.len() / 2..];
            data[i] = str1.parse().expect("Failed to convert");
            data.push(str2.parse().expect("Failed to convert"));
        } else {
            data[i] *= 2024;
        }
    }
}
