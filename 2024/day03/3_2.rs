use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() -> io::Result<()> {
    let path = "3_input.txt";

    let mut data: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                data.push(content);
            }
        }
    }

    let mut res = 0;
    let mut state = State::Enabled;

    for s in data {
        let (new_res, new_state) = parse_line(s, state);
        res += new_res;
        state = new_state;
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

#[derive(PartialEq)]
enum State {
    Enabled,
    Disabled,
}

fn parse_line(s: String, state: State) -> (i32, State) {
    // Capture "do()" indexes
    let re = Regex::new(r"do\(\)").unwrap();
    let mut do_vec: Vec<usize> = Vec::new();

    for cap in re.captures_iter(&s) {
        let start: usize = if let Some(pos) = cap.get(0) {
            pos.start()
        } else {
            continue;
        };
        do_vec.push(start);
    }

    // Capture "don't" indexes
    let re = Regex::new(r"don't\(\)").unwrap();
    let mut dont_vec: Vec<usize> = Vec::new();

    for cap in re.captures_iter(&s) {
        let start: usize = if let Some(pos) = cap.get(0) {
            pos.start()
        } else {
            continue;
        };
        dont_vec.push(start);
    }

    // Capture "mul(i,j)" and their indexes
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut mul_vec: Vec<(usize, i32, i32)> = Vec::new();

    for cap in re.captures_iter(&s) {
        let start: usize = if let Some(pos) = cap.get(0) {
            pos.start()
        } else {
            continue;
        };
        let i = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let j = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        mul_vec.push((start, i, j));
    }

    // Only consider the enabled mul
    let mut res = 0;
    let mut state = state;

    for idx in 0..s.len() {
        if do_vec.contains(&idx) {
            state = State::Enabled;
        } else if dont_vec.contains(&idx) {
            state = State::Disabled;
        }

        if let Some((_, i, j)) = mul_vec.iter().find(|(x, _, _)| *x == idx) {
            if state == State::Enabled {
                res += i * j;
            }
        }
    }

    return (res, state);
}
