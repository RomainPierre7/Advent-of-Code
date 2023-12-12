import os
from functools import cache

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '12_input.txt')
input_file = open(file_path, 'r')

@cache
def combinations_count(springs, groups, current_size = 0, has_to_be_valid = False):
    if springs == "":
        if len(groups) == 0:
            if current_size == 0:
                return 1
            return 0
        else:
            if len(groups) == 1 and groups[0] == current_size:
                return 1
            return 0
        
    if len(groups) == 0:
        if current_size != 0 or '#' in springs:
            return 0
        return 1
    
    char = springs[0]
    next = springs[1:]

    if char == '?':
        return combinations_count('#' + next, groups, current_size, has_to_be_valid) + combinations_count('.' + next, groups, current_size, has_to_be_valid)
    
    if char == '#':
        if has_to_be_valid:
            return 0
        current_size += 1
        if current_size > groups[0]:
            return 0
        if current_size == groups[0]:
            return combinations_count(next, groups[1:], 0, True)
        return combinations_count(next, groups, current_size, False)
    
    if char == '.':
        if has_to_be_valid or current_size == 0:
            return combinations_count(next, groups, 0, False)
        if current_size != groups[0]:
            return 0
        if current_size == groups[0]:
            return combinations_count(next, groups[1:], 0, False)

result = 0

for line in input_file:
    springs, groups = line.split()
    groups = tuple(int(c) for c in groups.split(","))

    springs = springs + ('?' + springs) * 4
    groups = groups * 5

    result += combinations_count(springs, groups)

print(result)