import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '3_input.txt')
input = open(file_path, 'r')

length = len(input.readline()) - 1
input.seek(0)

M = []
for line in input:
    M.append([line[i] for i in range(length)])

def is_part_number(line_number, indexes):
    neighbors = []
    rows, cols = len(M), len(M[0])
    for col in indexes:    
        for i in range(max(0, line_number - 1), min(rows, line_number + 2)):
            for j in range(max(0, col - 1), min(cols, col + 2)):
                if i != line_number or j != col:
                    neighbors.append(M[i][j])
    for c in neighbors:
        if not c.isdigit() and c != '.':
            return True
    return False

total = 0

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
        if is_part_number(j, idxs):
            total += num

print(total)