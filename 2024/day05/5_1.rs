use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "5_input.txt";

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut data: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                if content == "" {
                    continue;
                }
                // First try to collect the rules
                let mut parts = content.split('|');
                if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
                    if let (Ok(x_num), Ok(y_num)) =
                        (x.trim().parse::<i32>(), y.trim().parse::<i32>())
                    {
                        rules.push((x_num, y_num));
                    }
                } else {
                    // Collect the updates
                    let mut vec: Vec<i32> = Vec::new();
                    for number in content.split(',') {
                        if let Ok(num) = number.parse::<i32>() {
                            vec.push(num);
                        }
                    }
                    data.push(vec);
                }
            }
        }
    }

    let mut res = 0;

    for update in data {
        let mut is_valid: bool = true;

        // Check if the update is valid
        for i in 0..update.len() {
            if !check_rule(&update, i, &rules) {
                is_valid = false;
                break;
            }
        }

        // Update the result if valid
        if is_valid {
            res += update[update.len() / 2];
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

fn check_rule(update: &Vec<i32>, i: usize, rules: &Vec<(i32, i32)>) -> bool {
    let num_to_check = update[i];
    for (rule1, rule2) in rules {
        if *rule1 == num_to_check {
            for j in 0..i {
                if update[j] == *rule2 {
                    return false;
                }
            }
        }
    }
    return true;
}