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

    let mut res: u64 = 0;

    for code in &codes {
        let mut seq: Vec<Vec<Key>> = Vec::new();
        // Third robot sequence (numeric keypad)
        let code_in_keys = code_to_keys(&code);
        println!("Codes: {:?}", code_in_keys);
        seq.push(code_in_keys.clone());
        // Second robot sequence (directional keypad)
        seq = get_sequence(&seq, &numeric_keypad, (3, 2), (3, 0));
        println!("Seq 2rd: {:?}", seq);
        // First robot sequence (directional keypad)
        seq = get_sequence(&seq, &directional_keypad, (0, 2), (0, 0));
        println!("Seq 1st: {:?}", seq);
        // Sequence to type (directional keypad)
        seq = get_sequence(&seq, &directional_keypad, (0, 2), (0, 0));
        println!("Seq to type: {:?}", seq);

        println!("LEN SEQ: {:?}", seq.len());

        let min_seq = seq
            .iter()
            .min_by_key(|v| v.len())
            .expect("No min sequence found");

        res += score(&min_seq, &code);
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

#[repr(i8)]
#[derive(Debug, PartialEq, Clone)]
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

fn get_sequence(
    codes: &Vec<Vec<Key>>,
    keypad: &Vec<[Key; 3]>,
    init_start: (usize, usize),
    forbidden_case: (usize, usize),
) -> Vec<Vec<Key>> {
    let mut sequences: Vec<Vec<Key>> = Vec::new();
    let height = keypad.len();
    let width = keypad[0].len();

    fn add_seq(
        start: (usize, usize),
        target: (usize, usize),
        seq: &mut Vec<Key>,
        forbidden_case: (usize, usize),
    ) {
        let row_diff = target.0 as isize - start.0 as isize;
        let col_diff = target.1 as isize - start.1 as isize;

        // Make sure that we do not pass by the forbidden case
        if (start.1 == forbidden_case.1 && target.0 == forbidden_case.0)
            || !(start.0 == forbidden_case.0 && target.1 == forbidden_case.1)
        {
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

    for code in codes {
        let mut seq: Vec<Key> = Vec::new();
        let mut start = init_start;

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

            add_seq(start, target, &mut seq, forbidden_case);
            seq.push(Key::A);

            start = target;
        }
        sequences.push(seq);
    }

    return sequences;
}

fn score(seq: &Vec<Key>, code: &Vec<char>) -> u64 {
    let numeric_str: String = code.iter().filter(|c| c.is_digit(10)).collect();
    let numeric_value: u64 = numeric_str
        .trim_start_matches('0')
        .parse()
        .expect("Invalid number");
    println!("Numeric value: {}", numeric_value);
    println!("Sequence: {:?}", seq.len());
    return seq.len() as u64 * numeric_value;
}
