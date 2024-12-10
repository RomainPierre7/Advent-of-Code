use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "9_input.txt";

    let mut data: Vec<(u32, i32)> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let mut id_data = 0;
            let mut id_free = -1;
            let mut is_free = false;
            if let Ok(content) = line {
                for number in content.chars() {
                    if let Ok(num) = number.to_string().parse::<u32>() {
                        if is_free {
                            data.push((num, id_free));
                            id_free -= 1;
                            is_free = false;
                        } else {
                            data.push((num, id_data));
                            id_data += 1;
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

fn compress_data(data: Vec<(u32, i32)>) -> Vec<(u32, i32)> {
    let mut compressed: Vec<(u32, i32)> = data.clone();
    let mut current_id = compressed[compressed.len() - 1].1;

    // Take rightmost data block and try to move it to the left
    while current_id >= 0 {
        let (current_idx, (size1, id1)) = compressed
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &(_, id))| id == current_id)
            .map(|(index, &(s, id))| (index, (s, id)))
            .unwrap();

        for i in 0..current_idx {
            let (size2, id2) = compressed[i];
            if id2 < 0 {
                // Free block found
                if size2 >= size1 {
                    // Free block is enough to contain data block
                    compressed[i] = (size2 - size1, id2);
                    compressed.insert(i, (size1, id1));
                    // Free old data block
                    let idx_to_free = compressed
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, (_, id))| *id == id1)
                        .unwrap()
                        .0;
                    compressed[idx_to_free] = (size1, -1);
                    break;
                }
            }
        }
        current_id -= 1;
    }

    return compressed;
}

fn checksum(data: Vec<(u32, i32)>) -> u64 {
    let mut res: u64 = 0;
    let mut idx: u64 = 0;
    for (size, id) in data {
        for _ in 0..size {
            if id >= 0 {
                res += idx * id as u64;
            }
            idx += 1;
        }
    }
    return res;
}
