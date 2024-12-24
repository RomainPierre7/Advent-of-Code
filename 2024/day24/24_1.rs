use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "24_input.txt";

    let mut wires: HashMap<String, bool> = HashMap::new();
    let mut operations: Vec<(String, Gate, String, String)> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                if content.contains(':') {
                    let parts: Vec<&str> = content.split(':').collect();
                    if parts.len() == 2 {
                        let wire = parts[0].trim();
                        let value = parts[1].trim();
                        wires.insert(wire.to_string(), value == "1");
                    } else {
                        eprintln!("Incorrect format: {}", content);
                    }
                } else if content.contains("->") {
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if parts.len() == 5 && parts[3] == "->" {
                        let wire1 = parts[0].trim();
                        let gate = match parts[1] {
                            "AND" => Gate::AND,
                            "OR" => Gate::OR,
                            "XOR" => Gate::XOR,
                            _ => {
                                eprintln!("Unknown gate: {}", parts[1]);
                                continue;
                            }
                        };
                        let wire2 = parts[2].trim();
                        let wire3 = parts[4].trim();
                        operations.push((
                            wire1.to_string(),
                            gate,
                            wire2.to_string(),
                            wire3.to_string(),
                        ));
                    } else {
                        eprintln!("Incorrect format: {}", content);
                    }
                }
            }
        }
    }

    while !operations.is_empty() {
        let mut to_remove: Vec<usize> = Vec::new();
        for (i, (wire1, gate, wire2, wire3)) in operations.iter().enumerate() {
            if wires.contains_key(wire1) && wires.contains_key(wire2) {
                let value1 = wires.get(wire1).unwrap();
                let value2 = wires.get(wire2).unwrap();
                let result = match gate {
                    Gate::AND => value1 & value2,
                    Gate::OR => value1 | value2,
                    Gate::XOR => value1 ^ value2,
                };
                wires.insert(wire3.to_string(), result);
                to_remove.push(i);
            }
        }
        for i in to_remove.iter().rev() {
            operations.remove(*i);
        }
    }

    let mut z_wires: Vec<(String, bool)> = Vec::new();
    for (wire, value) in wires.iter() {
        if wire.starts_with('z') {
            z_wires.push((wire.to_string(), *value));
        }
    }

    z_wires.sort_by(|a, b| a.0.cmp(&b.0));

    let res = wires_to_binary(&z_wires);

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

enum Gate {
    AND,
    OR,
    XOR,
}

fn wires_to_binary(wires: &Vec<(String, bool)>) -> u64 {
    let mut res: u64 = 0;
    for (i, (_, value)) in wires.iter().enumerate() {
        if *value {
            res |= 1 << i;
        }
    }
    return res;
}
