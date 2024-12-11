use std::collections::HashMap;
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

    let res = blink(&data, 25);

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

fn blink(data: &Vec<u64>, times: u32) -> u64 {
    fn count(num: u64, depth: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
        if depth == 0 {
            return 1;
        }

        if let Some(&cached_result) = cache.get(&(num, depth)) {
            return cached_result;
        }

        let result = if num == 0 {
            count(1, depth - 1, cache)
        } else if num.to_string().len() % 2 == 0 {
            let str_num = num.to_string();
            let mid = str_num.len() / 2;
            let s1: u64 = str_num[..mid].parse().expect("Failed to convert to u64");
            let s2: u64 = str_num[mid..].parse().expect("Failed to convert to u64");
            count(s1, depth - 1, cache) + count(s2, depth - 1, cache)
        } else {
            count(num * 2024, depth - 1, cache)
        };

        cache.insert((num, depth), result);
        return result;
    }

    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    let mut res: u64 = 0;

    for &value in data.iter() {
        res += count(value, times, &mut cache);
    }

    return res;
}
