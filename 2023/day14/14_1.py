import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '14_input.txt')
input_file = open(file_path, 'r')

PLATFORM = []
for line in input_file:
    PLATFORM.append([c for c in line.strip()])

for i in range(len(PLATFORM)):
    for j in range(len(PLATFORM[0])):
        if PLATFORM[i][j] == 'O':
            row, col = i, j
            while row > 0:
                if PLATFORM[row-1][col] == '.':
                    row -= 1
                else:
                    break
            PLATFORM[i][j], PLATFORM[row][col] = PLATFORM[row][col], PLATFORM[i][j]

res = 0
for i in range(len(PLATFORM)):
    for j in range(len(PLATFORM[0])):
        if PLATFORM[i][j] == 'O':
            res += len(PLATFORM) - i

print(res)