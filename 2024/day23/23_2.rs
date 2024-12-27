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

    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    for (c1, c2) in data {
        network.entry(c1.clone()).or_default().push(c2.clone());
        network.entry(c2).or_default().push(c1);
    }

    let mut set_network: Vec<HashSet<String>> = Vec::new();
    for (k, v) in &network {
        let mut connected = HashSet::new();
        connected.extend(v.iter().cloned());
        connected.insert(k.clone());
        set_network.push(connected);
    }

    let mut count: HashMap<Vec<String>, i32> = HashMap::new();
    for i in 0..set_network.len() {
        for j in (i + 1)..set_network.len() {
            let intersection: HashSet<_> = set_network[i]
                .intersection(&set_network[j])
                .cloned()
                .collect();
            if intersection.len() > 10 {
                let mut sorted_intersection: Vec<String> = intersection.into_iter().collect();
                sorted_intersection.sort();
                count
                    .entry(sorted_intersection)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    for (k, v) in count {
        let n = k.len();
        if (n * (n - 1)) as f64 / 2.0 == v as f64 {
            println!("{}", k.join(","));
            break;
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
