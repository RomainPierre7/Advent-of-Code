use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "17_input.txt";

    let mut program: Vec<i64> = Vec::new();
    let mut register_b: i64 = 0;
    let mut register_c: i64 = 0;

    let mut output: Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                if let Some(value) = content.strip_prefix("Register B: ") {
                    register_b = value.trim().parse::<i64>().expect("Failed to parse");
                } else if let Some(value) = content.strip_prefix("Register C: ") {
                    register_c = value.trim().parse::<i64>().expect("Failed to parse");
                } else if let Some(value) = content.strip_prefix("Program: ") {
                    for number in value.trim().split(',') {
                        if let Ok(num) = number.trim().parse::<i64>() {
                            program.push(num);
                        } else {
                            eprintln!("Failed to parse program number: '{}'", number.trim());
                        }
                    }
                }
            }
        }
    }

    let mut possibilities: Vec<Vec<i64>> = Vec::new();
    possibilities.push((0..8).collect::<Vec<i64>>());
    for exp in 1..program.len() {
        let mut current_possibilities = Vec::new();
        for &p in &possibilities[exp - 1] {
            for q in 0..8 {
                if p == 0 {
                    continue;
                }
                let ra = 8 * p + q;
                let mut register_a = ra;
                output.clear();
                run(program.clone(), &mut register_a, &mut register_b, &mut register_c, &mut output);
                let len = output.len();
                if output == program[program.len() - len..] {
                    current_possibilities.push(ra);
                }
                if output == program {
                    let res = ra;
                    println!("{res}");
                    return Ok(());
                }
            }
        }
        possibilities.push(current_possibilities);
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

fn get_operand(operand: i64,register_a: i64, register_b: i64, register_c: i64) -> i64 {
        return match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => -1,
    };
}

fn adv(operand: i64, register_a: &mut i64) {
    *register_a = *register_a / (1 << operand);
}

fn bxl(operand: i64, register_b: &mut i64) {
    *register_b = *register_b ^ operand;
}

fn bst(operand: i64, register_b: &mut i64) {
    *register_b = operand % 8;
}

fn jnz(operand: i64, register_a: &mut i64, instruction_pointer: &mut usize) -> bool {
    if *register_a != 0 {
        *instruction_pointer = operand as usize;
        return true;
    }
    return false;
}

fn bxc(register_b: &mut i64, register_c: &mut i64) {
    *register_b = *register_b ^ *register_c;
}

fn out(operand: i64, output: &mut Vec<i64>) {
    output.push((operand % 8 + 8) % 8);
}

fn bdv(operand: i64, register_a: &mut i64, register_b: &mut i64) {
    *register_b = *register_a / (1 << operand);
}

fn cdv(operand: i64, register_a: &mut i64, register_c: &mut i64) {
    *register_c = *register_a / (1 << operand);
}

fn run(program: Vec<i64>, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64, output: &mut Vec<i64>) {
    let mut instruction_pointer: usize = 0;
    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = get_operand(program[instruction_pointer + 1], *register_a, *register_b, *register_c);
        match opcode {
            0 => adv(operand, register_a),
            1 => bxl(operand, register_b),
            2 => bst(operand, register_b),
            3 => if jnz(operand, register_a, &mut instruction_pointer) { continue; },
            4 => bxc(register_b, register_c),
            5 => out(operand, output),
            6 => bdv(operand, register_a, register_b),
            7 => cdv(operand, register_a, register_c),
            _ => eprintln!("Unknown opcode: {}", opcode),
        }
        instruction_pointer += 2;
    }
}