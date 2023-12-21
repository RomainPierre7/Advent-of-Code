import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '21_input.txt')
input_file = open(file_path, 'r')

MAP = []
for line in input_file:
    MAP.append([c for c in line.strip()])

for i in range(len(MAP)):
    for j in range(len(MAP[0])):
        if MAP[i][j] == 'S':
            START = (i, j)
            MAP[i][j] = '.'

positions = {START}

for _ in range(64):
    new_positions = set()
    for pos in positions:
        i, j = pos
        if i > 0 and MAP[i-1][j] == '.':
            new_positions.add((i-1, j))
        if i < len(MAP) - 1 and MAP[i+1][j] == '.':
            new_positions.add((i+1, j))
        if j > 0 and MAP[i][j-1] == '.':
            new_positions.add((i, j-1))
        if j < len(MAP[0]) - 1 and MAP[i][j+1] == '.':
            new_positions.add((i, j+1))
    positions = new_positions

print(len(positions))