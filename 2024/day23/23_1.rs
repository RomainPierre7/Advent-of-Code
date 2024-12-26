use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "23_input.txt";
    let mut data: Vec<(String, String)> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut parts = content.split("-");
                let computer1 = parts.next().unwrap().to_string();
                let computer2 = parts.next().unwrap().to_string();
                data.push((computer1, computer2));
            }
        }
    }

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for (computer1, computer2) in data {
        let set1 = connections
            .entry(computer1.clone())
            .or_insert(HashSet::new());
        set1.insert(computer2.clone());

        let set2 = connections
            .entry(computer2.clone())
            .or_insert(HashSet::new());
        set2.insert(computer1.clone());
    }

    let mut triangles: HashSet<Vec<String>> = HashSet::new();

    for (computer1, neighbors1) in &connections {
        for computer2 in neighbors1 {
            if let Some(neighbors2) = connections.get(computer2) {
                for computer3 in neighbors2 {
                    if computer3 != computer1 && neighbors1.contains(computer3) {
                        let mut start_by_t = false;
                        let mut triangle =
                            vec![computer1.clone(), computer2.clone(), computer3.clone()];
                        for computer in &triangle {
                            if computer.starts_with('t') {
                                start_by_t = true;
                                break;
                            }
                        }
                        if start_by_t {
                            triangle.sort();
                            triangles.insert(triangle);
                        }
                    }
                }
            }
        }
    }

    let res = triangles.len();

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
