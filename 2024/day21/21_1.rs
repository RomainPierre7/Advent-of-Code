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

    let numeric_keypad = vec![
        [Key::Seven, Key::Eight, Key::Nine],
        [Key::Four, Key::Five, Key::Six],
        [Key::One, Key::Two, Key::Three],
        [Key::None, Key::Zero, Key::A],
    ];

    let directional_keypad = vec![
        [Key::None, Key::Up, Key::A],
        [Key::Left, Key::Down, Key::Right],
    ];

    for code in &codes {
        let code_in_keys = code_to_keys(&code);
        println!("Codes: {:?}", code_in_keys);
        // Third robot sequence (numeric keypad)
        let mut seq = get_sequence(&code_in_keys, &numeric_keypad);
        println!("Seq 3rd: {:?}", seq);
        // Second robot sequence (directional keypad)
        seq = get_sequence(&seq, &directional_keypad);
        println!("Seq 2nd: {:?}", seq);
        // First robot sequence (directional keypad)
        seq = get_sequence(&seq, &directional_keypad);
        println!("Seq 1st: {:?}", seq);
        // Sequence to type (directional keypad)
        seq = get_sequence(&seq, &directional_keypad);
        println!("Seq to type: {:?}", seq);
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

#[repr(i8)]
#[derive(Debug, PartialEq)]
enum Key {
    None = -2,
    A = -1,
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Up = 10,
    Down = 11,
    Left = 12,
    Right = 13,
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

fn get_sequence(code: &Vec<Key>, keypad: &Vec<[Key; 3]>) -> Vec<Key> {
    let mut seq: Vec<Key> = Vec::new();
    let height = keypad.len();
    let width = keypad[0].len();
    let mut start = (3, 2);

    fn add_seq(start: (usize, usize), target: (usize, usize), seq: &mut Vec<Key>) {
        let row_diff = target.0 as isize - start.0 as isize;
        let col_diff = target.1 as isize - start.1 as isize;

        // If we start from column 0 we start by moving by col to avoid the hole
        if start.1 == 0 {
            if col_diff > 0 {
                for _ in 0..col_diff {
                    seq.push(Key::Right);
                }
            } else {
                for _ in 0..-col_diff {
                    seq.push(Key::Left);
                }
            }
            if row_diff > 0 {
                for _ in 0..row_diff {
                    seq.push(Key::Down);
                }
            } else {
                for _ in 0..-row_diff {
                    seq.push(Key::Up);
                }
            }
        } else {
            if row_diff > 0 {
                for _ in 0..row_diff {
                    seq.push(Key::Down);
                }
            } else {
                for _ in 0..-row_diff {
                    seq.push(Key::Up);
                }
            }
            if col_diff > 0 {
                for _ in 0..col_diff {
                    seq.push(Key::Right);
                }
            } else {
                for _ in 0..-col_diff {
                    seq.push(Key::Left);
                }
            }
        }
    }

    for key in code {
        let mut target = (0, 0);
        let mut found = false;

        for row in 0..height {
            for col in 0..width {
                if keypad[row][col] == *key {
                    target = (row, col);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        add_seq(start, target, &mut seq);
        seq.push(Key::A);

        start = target;
    }

    return seq;
}

fn get_sequence_from_directional(seq: &Vec<Key>, keypad: &Vec<[Key; 3]>) -> Vec<Key> {
    return Vec::new();
}
