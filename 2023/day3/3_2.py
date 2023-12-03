import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '3_input.txt')
input = open(file_path, 'r')

length = len(input.readline()) - 1
input.seek(0)

M = []
for line in input:
    M.append([line[i] for i in range(length)])

def is_next_to_gear(line_number, indexes):
    neighbors = []
    neighbors_coord = []
    rows, cols = len(M), len(M[0])
    for col in indexes:    
        for i in range(max(0, line_number - 1), min(rows, line_number + 2)):
            for j in range(max(0, col - 1), min(cols, col + 2)):
                if i != line_number or j != col:
                    neighbors.append(M[i][j])
                    neighbors_coord.append((i,j))
    result_coord = []
    is_gear = False
    for i, c in enumerate(neighbors):
        if c == '*':
            is_gear = True
            result_coord.append(neighbors_coord[i])
    if is_gear:
        return True, list(set(result_coord))   
    return False, None

next_to_gear_numbers = []
gear_coord = []

for j, line in enumerate(M):
    numbers_array = []
    idxs_array = []
    number = ""
    indexes = []

    for i, char in enumerate(line):
        if char.isdigit():
            number += char
            indexes.append(i)
        if not char.isdigit() or i == length - 1:
            if number.isdigit():
                numbers_array.append(int(number))
                idxs_array.append(indexes)
                number = ""
                indexes = []

    for num, idxs in zip(numbers_array, idxs_array):
        is_next, coords = is_next_to_gear(j, idxs)
        if is_next:
            for coord in coords:
                next_to_gear_numbers.append(num)
                gear_coord.append(coord)

selected_numbers = []

for i, coord in enumerate(gear_coord):
    count = 0
    idx = 0
    for j in range(i, len(gear_coord)):
        if (gear_coord[i] == gear_coord[j]):
            count += 1
            idx = j
    if count == 2:
        selected_numbers.append(next_to_gear_numbers[i])
        selected_numbers.append(next_to_gear_numbers[idx])

result = 0
for i in range(0, len(selected_numbers), 2):
    result += (selected_numbers[i] * selected_numbers[i+1])

print(result)