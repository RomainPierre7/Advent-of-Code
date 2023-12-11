import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '11_input.txt')
input_file = open(file_path, 'r')

UNIVERSE = []
for line in input_file:
    UNIVERSE.append([c for c in line.strip()])

expand_rows = []
expand_cols = []
EXPAND_TIME = 1000000

for i in range(len(UNIVERSE[0]) - 1, -1, -1):
    if all(l[i] == '.' for l in UNIVERSE):
        expand_cols.append(i)

for i in range(len(UNIVERSE) - 1, -1, -1):
    if all(z == '.' for z in UNIVERSE[i]):
        expand_rows.append(i)

galaxy_number = 0
for i in range(len(UNIVERSE)):
    for j in range(len(UNIVERSE[0])):
        if UNIVERSE[i][j] == '#':
            galaxy_number += 1
            UNIVERSE[i][j] = galaxy_number

result = 0
b1, b2 = False, False
percent = None
for i in range(1, galaxy_number + 1):
    for j in range(i + 1, galaxy_number + 1):
        for k in range(len(UNIVERSE)):
            if i in UNIVERSE[k]:
                row1 = k
                col1 = UNIVERSE[k].index(i)
                b1 = True
            
            if j in UNIVERSE[k]:
                row2 = k
                col2 = UNIVERSE[k].index(j)
                b2 = True
            
            if b1 and b2:
                break
        b1, b2 = False, False

        exp_row = 0
        exp_col = 0

        for row in expand_rows:
            if row1 < row < row2 or row2 < row < row1:
                exp_row += 1

        for col in expand_cols:
            if col1 < col < col2 or col2 < col < col1:
                exp_col += 1

        new_percent = i * 100 // galaxy_number
        if new_percent != percent:
            print(new_percent, "%")
            percent = new_percent

        result += (abs(row2 - row1) + abs(col2 - col1) + (EXPAND_TIME - 1) * (exp_row + exp_col))

print(result)