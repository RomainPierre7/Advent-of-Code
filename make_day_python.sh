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
    echo "import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '${day}_input.txt')
input_file = open(file_path, 'r')" > "$year/day$formatted_day/${day}_1.py"
    echo "Created file ${day}_1.py"
fi

if [ ! -f "$year/day$formatted_day/${day}_2.py" ]; then
    touch "$year/day$formatted_day/${day}_2.py"
    echo "Created file ${day}_2.py"
fi

echo "Done !"