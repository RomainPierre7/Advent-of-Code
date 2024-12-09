use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "9_input.txt";

    let mut data: Vec<(u32, i32)> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let mut id = 0;
            let mut is_free = false;
            if let Ok(content) = line {
                for number in content.chars() {
                    if let Ok(num) = number.to_string().parse::<u32>() {
                        if is_free {
                            data.push((num, -1));
                            is_free = false;
                        } else {
                            data.push((num, id));
                            id += 1;
                            is_free = true;
                        }
                    } else {
                        eprintln!("Line format incorrect: {}", content);
                    }
                }
            }
        }
    }

    let compressed = compress_data(data);

    let res = checksum(compressed);

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

fn compress_data(data: Vec<(u32, i32)>) -> Vec<(u32, u32)> {
    let mut compressed: Vec<(u32, u32)> = Vec::new();

    let mut idx = 0;
    for i in (0..data.len()).rev() {
        let (size1, id1) = data[i];
        if id1 != -1 {
            let mut filled = 0;
            for j in 0..data.len() {
                let (size2, id2) = data[j];
                if id2 == -1 {
                    if filled == size1 {
                        break;
                    } else {
                        println!("{size1} {filled} {size2}");
                        let to_fill: i32 = size1 as i32 - filled as i32 - size2 as i32;
                        if to_fill >= 0 {
                            compressed.push((to_fill as u32, id1 as u32));
                            filled += to_fill as u32;
                        }
                    }
                } else {
                    if j == idx {
                        compressed.push((size2, id2 as u32));
                        idx += 2;
                    }
                }
            }
        }
    }

    println!("RESULT: {:?}", compressed);

    println!(
        "TARGET: {:?}",
        vec![
            (2, 0),
            (2, 9),
            (1, 8),
            (3, 1),
            (3, 8),
            (1, 2),
            (3, 7),
            (3, 3),
            (1, 6),
            (2, 4),
            (1, 6),
            (4, 5),
            (2, 6),
        ]
    );

    return compressed;
}

fn checksum(data: Vec<(u32, u32)>) -> u32 {
    let mut res: u32 = 0;
    let mut idx = 0;
    for (size, id) in data {
        for _ in 0..size {
            res += idx * id;
            idx += 1;
        }
    }
    return res;
}
