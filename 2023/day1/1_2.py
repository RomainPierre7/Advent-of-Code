import os
import re

spelled_digit = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
spelled_pattern = [re.compile(re.escape(i)) for i in spelled_digit]

def is_spelled_digit(buffer):
    for pattern, word in zip(spelled_pattern, spelled_digit):
        if pattern.search(re.escape(buffer)):
            return True, word
    return False, None

def spelled_digit_to_str_digit(word):
    match word:
        case "one":
            return "1"
        case "two":
            return "2"
        case "three":
            return "3"
        case "four":
            return "4"
        case "five":
            return "5"
        case "six":
            return "6"
        case "seven":
            return "7"
        case "eight":
            return "8"
        case "nine":
            return "9"
        case _:
            return "ERROR"

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, "1_input.txt")
input = open(file_path, 'r')

result = 0

for line in input:
    line_digit = ""
    
    for i in range(len(line) - 1):
        line_buffer = line[:i+1]
        check, word = is_spelled_digit(line_buffer)
        if (check):
            line_digit = spelled_digit_to_str_digit(word)
            break
        if (line[i].isdigit()):
            line_digit = line[i]
            break

    for i in range(len(line) - 1, -1, -1):
        line_buffer = line[i:]
        check, word = is_spelled_digit(line_buffer)
        if (check):
            line_digit += spelled_digit_to_str_digit(word)
            break
        if (line[i].isdigit()):
            line_digit += line[i]
            break

    line_value = int(line_digit)
    
    result += line_value

print(result)