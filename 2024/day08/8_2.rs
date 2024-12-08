use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "8_input.txt";

    let mut data: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut vec: Vec<char> = Vec::new();
                for c in content.chars() {
                    vec.push(c);
                }
                data.push(vec);
            }
        }
    }

    let height = data.len();
    let width = data[0].len();

    let antennas = get_antennas(data);

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, positions) in &antennas {
        for &(i1, j1) in positions {
            for &(i2, j2) in positions {
                let diff_i = i2 as i32 - i1 as i32;
                let diff_j = j2 as i32 - j1 as i32;
                if diff_i != 0 && diff_j != 0 {
                    let mut antinode_i = i2 as i32;
                    let mut antinode_j = j2 as i32;

                    while antinode_i >= 0
                        && antinode_j >= 0
                        && antinode_i < height as i32
                        && antinode_j < width as i32
                    {
                        antinodes.insert((antinode_i as usize, antinode_j as usize));

                        antinode_i += diff_i;
                        antinode_j += diff_j;
                    }
                }
            }
        }
    }

    println!("{:?}", antinodes.len());

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_antennas(data: Vec<Vec<char>>) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut antennas: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    for (i, row) in data.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_insert_with(HashSet::new)
                    .insert((i, j));
            }
        }
    }

    return antennas;
}
