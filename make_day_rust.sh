#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 year day"
    exit 1
fi

year="$1"
day="$2"
formatted_day=$(printf "%02d" $day)

if [ ! -d "$year" ]; then
    mkdir "$year"
    echo "Created directory $year"
fi

if [ ! -d "$year/day$formatted_day" ]; then
    mkdir "$year/day$formatted_day"
    echo "Created directory day$formatted_day"
fi

if [ ! -f "$year/day$formatted_day/${day}_input.txt" ]; then
    touch "$year/day$formatted_day/${day}_input.txt"
    echo "Created file ${day}_input.txt"
fi

if [ ! -f "$year/day$formatted_day/${day}_1.py" ]; then
    echo 'use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "i_input.txt";

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                // Get content
            }
        }
    }

    // Process

    Ok(())
}

// Function to read a file and return an iterator over its lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
' > "$year/day$formatted_day/${day}_1.rs"
    echo "Created file ${day}_1.rs"
fi

if [ ! -f "$year/day$formatted_day/${day}_2.rs" ]; then
    touch "$year/day$formatted_day/${day}_2.rs"
    echo "Created file ${day}_2.rs"
fi

echo "Done !"