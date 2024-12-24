use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "21_input.txt";

    let mut codes: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let mut vec: Vec<char> = Vec::new();
                for c in content.chars() {
                    vec.push(c);
                }
                codes.push(vec);
            }
        }
    }

    let mut res: u64 = 0;

    for code in &codes {
        let keys = code_to_keys(code);
        let numeric_str: String = code.iter().filter(|c| c.is_digit(10)).collect();
        let numeric_value: u64 = numeric_str
            .trim_start_matches('0')
            .parse()
            .expect("Invalid number");
        res += compute_fewest(keys, 3) * numeric_value;
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

#[derive(Debug, PartialEq, Clone, Hash, Eq, Copy)]
enum Key {
    None,
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Up,
    Down,
    Left,
    Right,
}

fn code_to_keys(code: &Vec<char>) -> Vec<Key> {
    let mut keys: Vec<Key> = Vec::new();

    for c in code {
        match c {
            'A' => keys.push(Key::A),
            '0' => keys.push(Key::Zero),
            '1' => keys.push(Key::One),
            '2' => keys.push(Key::Two),
            '3' => keys.push(Key::Three),
            '4' => keys.push(Key::Four),
            '5' => keys.push(Key::Five),
            '6' => keys.push(Key::Six),
            '7' => keys.push(Key::Seven),
            '8' => keys.push(Key::Eight),
            '9' => keys.push(Key::Nine),
            _ => (),
        }
    }

    return keys;
}

fn get_key_coords(numeric: bool) -> HashMap<Key, (i32, i32)> {
    let mut coords = HashMap::new();
    if numeric {
        // Numeric keypad coordinates
        let layout = vec![
            vec![Key::Seven, Key::Eight, Key::Nine],
            vec![Key::Four, Key::Five, Key::Six],
            vec![Key::One, Key::Two, Key::Three],
            vec![Key::None, Key::Zero, Key::A],
        ];
        for (y, row) in layout.iter().enumerate() {
            for (x, key) in row.iter().enumerate() {
                coords.insert(*key, (x as i32, y as i32));
            }
        }
    } else {
        // Directional keypad coordinates
        let layout = vec![
            vec![Key::None, Key::Up, Key::A],
            vec![Key::Left, Key::Down, Key::Right],
        ];
        for (y, row) in layout.iter().enumerate() {
            for (x, key) in row.iter().enumerate() {
                coords.insert(*key, (x as i32, y as i32));
            }
        }
    }
    return coords;
}

fn get_path_keys(start: (i32, i32), end: (i32, i32)) -> Vec<Key> {
    let mut path = Vec::new();
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;

    // Horizontal movement
    if dx > 0 {
        path.extend(vec![Key::Right; dx.abs() as usize]);
    } else if dx < 0 {
        path.extend(vec![Key::Left; dx.abs() as usize]);
    }

    // Vertical movement
    if dy > 0 {
        path.extend(vec![Key::Down; dy.abs() as usize]);
    } else if dy < 0 {
        path.extend(vec![Key::Up; dy.abs() as usize]);
    }

    return path;
}

fn fewest_presses(layer: u32, keys: &[Key], cache: &HashMap<(u32, Key, Key), u64>) -> u64 {
    let mut total = 0;
    let mut current = Key::A;

    for &next_key in keys {
        if let Some(&presses) = cache.get(&(layer, current, next_key)) {
            total += presses;
            current = next_key;
        }
    }
    total
}

fn compute_fewest(keys: Vec<Key>, n_robot: u32) -> u64 {
    let mut cache: HashMap<(u32, Key, Key), u64> = HashMap::new();

    // Initialize base case (layer 0)
    let dir_coords = get_key_coords(false);
    for (&k1, _) in &dir_coords {
        for (&k2, _) in &dir_coords {
            cache.insert((0, k1, k2), 1);
        }
    }

    // Build up through layers
    for layer in 1..=n_robot {
        let coords = if layer == n_robot {
            get_key_coords(true) // numeric keypad for final layer
        } else {
            get_key_coords(false) // directional keypad for other layers
        };

        let forbidden_case = coords.get(&Key::None).unwrap();

        for (&ki, &(xi, yi)) in &coords {
            for (&kf, &(xf, yf)) in &coords {
                // Try horizontal then vertical
                let hor_first: u64;
                let mid_point = (xf, yi);
                if mid_point != *forbidden_case {
                    let mut path = get_path_keys((xi, yi), mid_point);
                    path.extend(get_path_keys(mid_point, (xf, yf)));
                    path.push(Key::A);
                    hor_first = fewest_presses(layer - 1, &path, &cache);
                } else {
                    hor_first = u64::MAX;
                }

                // Try vertical then horizontal
                let ver_first: u64;
                let mid_point = (xi, yf);
                if mid_point != *forbidden_case {
                    let mut path = get_path_keys((xi, yi), mid_point);
                    path.extend(get_path_keys(mid_point, (xf, yf)));
                    path.push(Key::A);
                    ver_first = fewest_presses(layer - 1, &path, &cache);
                } else {
                    ver_first = u64::MAX;
                }

                if hor_first == u64::MAX && ver_first == u64::MAX {
                    continue;
                }

                let min_presses = std::cmp::min(hor_first, ver_first);

                cache.insert((layer, ki, kf), min_presses);
            }
        }
    }
    return fewest_presses(n_robot, &keys, &cache);
}
