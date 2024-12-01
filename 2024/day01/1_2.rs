use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "1_input.txt";

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut parts = content.split_whitespace();
                if let (Some(str1), Some(str2)) = (parts.next(), parts.next()) {
                    let int1: i32 = str1.parse().expect("Failed to convert");
                    let int2: i32 = str2.parse().expect("Failed to convert");

                    vec1.push(int1);
                    vec2.push(int2);
                } else {
                    eprintln!("Line format incorrect: {}", content);
                }
            }
        }
    }

    assert_eq!(vec1.len(), vec2.len());

    vec1.sort();
    vec2.sort();

    let mut res: i32 = 0;

    for i in 0..vec1.len() {
        let mut count: i32 = 0;
        let num_to_check = vec1[i];
        for j in 0..vec2.len() {
            if num_to_check == vec2[j] {
                count += 1;
            }
        }
        res += num_to_check * count;
    }

    println!("{:?}", res);

    Ok(())
}

// Function to read a file and return an iterator over its lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
