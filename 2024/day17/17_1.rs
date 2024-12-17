use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "17_input.txt";

    let mut program: Vec<i32> = Vec::new();
    let mut register_a: i32 = 0;
    let mut register_b: i32 = 0;
    let mut register_c: i32 = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                if let Some(value) = content.strip_prefix("Register A: ") {
                    register_a = value
                        .trim()
                        .parse::<i32>()
                        .expect("Failed to parse Register A value");
                } else if let Some(value) = content.strip_prefix("Register B: ") {
                    register_b = value
                        .trim()
                        .parse::<i32>()
                        .expect("Failed to parse Register B value");
                } else if let Some(value) = content.strip_prefix("Register C: ") {
                    register_c = value
                        .trim()
                        .parse::<i32>()
                        .expect("Failed to parse Register C value");
                } else if let Some(value) = content.strip_prefix("Program: ") {
                    for number in value.trim().split(',') {
                        if let Ok(num) = number.trim().parse::<i32>() {
                            program.push(num);
                        } else {
                            eprintln!("Failed to parse program number: '{}'", number.trim());
                        }
                    }
                }
            }
        }
    }

    println!("Register A: {}", register_a);
    println!("Register B: {}", register_b);
    println!("Register C: {}", register_c);
    println!("Program: {:?}", program);

    Ok(())
}

// Função auxiliar para ler linhas de um arquivo
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
