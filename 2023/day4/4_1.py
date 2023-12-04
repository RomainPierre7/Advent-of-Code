import os
import re
script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '4_input.txt')
input_file = open(file_path, 'r')

total = 0

for line in input_file:
    line = line.split(":")[1]
    winning = re.split(r'\s+', line.split("|")[0].strip())
    numbers = re.split(r'\s+', line.split("|")[1].strip())

    count = 0
    for num in numbers:
        if num in winning:
            count += 1

    if count > 0:
        total += 2 ** (count - 1)

print(total)