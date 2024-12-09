use std::cmp::min;
use std::collections::HashSet;
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

fn compress_data(data: Vec<(u32, i32)>) -> Vec<(u32, u32)> {
    let mut compressed: Vec<(u32, u32)> = Vec::new();
    let mut used_free: HashSet<i32> = HashSet::new();
    let mut moved_data: HashSet<i32> = HashSet::new();

    let mut idx = 0;
    let mut offset = 0; // Useful to know how much the first free is already used
    let mut last_added: (u32, u32) = (0, 0); // Useful in the second loop to know if the first block has already an added part
    for i in (0..data.len()).rev() {
        let (mut size1, id1) = data[i];
        if id1 >= 0 && !moved_data.contains(&id1) {
            // Is a data block => search free block
            for j in 0..data.len() {
                let (mut size2, id2) = data[j];
                if id2 < 0 && !used_free.contains(&id2) {
                    // Free block found
                    size2 -= offset;
                    let to_fill = min(size1, size2);
                    if to_fill > 0 {
                        compressed.push((to_fill as u32, id1 as u32));
                    }
                    if to_fill == size1 {
                        // All the data has been moved
                        offset += size1;
                        moved_data.insert(id1);
                        break;
                    } else {
                        // Only a part of the data has been moved
                        used_free.insert(id2);
                        size1 -= size2;
                        last_added = (last_added.0 + size2, id1 as u32); // USEFUL
                        offset = 0;
                    }
                } else {
                    // Add data when no free available on the left
                    if id2 != id1 && j == idx && !moved_data.contains(&id2) {
                        compressed.push((size2, id2 as u32));
                        moved_data.insert(id2);
                        idx += 2;
                    }
                }
            }
        }
    }

    // Second loop to add the last blocks (no more free blocks)
    for (mut size1, id1) in data {
        if id1 >= 0 && !moved_data.contains(&id1) {
            if last_added.1 == id1 as u32 && last_added.0 > 0 {
                size1 -= last_added.0;
                last_added = (0, 0);
            }
            compressed.push((size1, id1 as u32));
            moved_data.insert(id1);
        }
    }

    return compressed;
}

fn checksum(data: Vec<(u32, u32)>) -> u64 {
    let mut res: u64 = 0;
    let mut idx: u64 = 0;
    for (size, id) in data {
        for _ in 0..size {
            res += idx * id as u64;
            idx += 1;
        }
    }
    return res;
}
