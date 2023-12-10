import os
import re

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, "2_input.txt")
input = open(file_path, 'r')

max_red = 12
max_green = 13
max_blue = 14

red_pattern = re.compile(r'(\d+)\s(red)')
green_pattern = re.compile(r'(\d+)\s(green)')
blue_pattern = re.compile(r'(\d+)\s(blue)')

total = 0

for id, line in enumerate(input, start=1):
    sets_line  = line.split(':', 1)[-1].strip()
    sets = re.split(r';\s*', sets_line)
    for set in sets:
        red_number = red_pattern.search(set)
        green_number = green_pattern.search(set)
        blue_number = blue_pattern.search(set)

        red_number = int(red_number.group(1)) if red_number else 0
        green_number = int(green_number.group(1)) if green_number else 0
        blue_number = int(blue_number.group(1)) if blue_number else 0

        if (red_number > max_red or green_number > max_green or blue_number > max_blue):
            id = 0
            break

    total += id

print(total)