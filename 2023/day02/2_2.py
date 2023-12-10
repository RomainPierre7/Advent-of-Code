import os
import re

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, "2_input.txt")
input = open(file_path, 'r')

red_pattern = re.compile(r'(\d+)\s(red)')
green_pattern = re.compile(r'(\d+)\s(green)')
blue_pattern = re.compile(r'(\d+)\s(blue)')

total = 0

for id, line in enumerate(input, start=1):
    sets_line  = line.split(':', 1)[-1].strip()
    sets = re.split(r';\s*', sets_line)

    red_result, green_result, blue_result = [], [], []

    for set in sets:
        red_number = red_pattern.search(set)
        green_number = green_pattern.search(set)
        blue_number = blue_pattern.search(set)

        red_number = int(red_number.group(1)) if red_number else 0
        green_number = int(green_number.group(1)) if green_number else 0
        blue_number = int(blue_number.group(1)) if blue_number else 0

        red_result.append(red_number)
        green_result.append(green_number)
        blue_result.append(blue_number)

    max_red = max(red_result)
    max_green = max(green_result)
    max_blue = max(blue_result)

    total += max_red * max_green * max_blue

print(total)