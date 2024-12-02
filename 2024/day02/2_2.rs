use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "2_input.txt";

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

    let mut count = 0;

    for vec in data {
        for i in 0..vec.len() {
            if is_safe(&[&vec[0..i], &vec[i + 1..vec.len()]].concat()) == Report::Safe {
                count += 1;
                break;
            }
        }
    }

    println!("{count}");

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq)]
enum Report {
    Safe,
    Unsafe,
}

#[derive(PartialEq)]
enum State {
    Increasing,
    Decreasing,
}

fn is_safe(vec: &[i32]) -> Report {
    if vec.len() < 2 {
        return Report::Safe;
    }

    if vec[0] == vec[1] || (vec[0] - vec[1]).abs() > 3 {
        return Report::Unsafe;
    }

    let mut trend: State = State::Increasing;
    if vec[0] > vec[1] {
        trend = State::Decreasing;
    }

    for i in 1..vec.len() - 1 {
        let dist = vec[i] - vec[i + 1];
        if (dist.abs() < 1)
            || (dist.abs() > 3)
            || (trend == State::Increasing && dist > 0)
            || (trend == State::Decreasing && dist < 0)
        {
            return Report::Unsafe;
        }
    }

    return Report::Safe;
}
