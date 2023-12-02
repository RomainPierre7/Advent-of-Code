import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, "day1_1_input.txt")
input = open(file_path, 'r')

result = 0

for line in input:
    line_digit = ""
    for i in range(len(line) - 1):
        if (line[i].isdigit()):
            line_digit = line[i]
            break
    for i in range(len(line) - 1, -1, -1):
        if (line[i].isdigit()):
            line_digit += line[i]
            break
    line_value = int(line_digit)
    
    result += line_value

print(result)