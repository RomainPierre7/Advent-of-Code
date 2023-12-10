import os
import re
script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '4_input.txt')
input_file = open(file_path, 'r')

length = len(input_file.readlines())
input_file.seek(0)

cards = [1 for _ in range(length)]

for i, line in enumerate(input_file):
    line = line.split(":")[1]
    winning = re.split(r'\s+', line.split("|")[0].strip())
    numbers = re.split(r'\s+', line.split("|")[1].strip())

    count = 0
    for num in numbers:
        if num in winning:
            count += 1

    for j in range(i+1, i + 1 + count):
        cards[j] += cards[i]

print(sum(cards))