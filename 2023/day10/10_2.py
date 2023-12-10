import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '10_input.txt')
input_file = open(file_path, 'r')

map = []
for line in input_file:
    map.append([c for c in line.strip()])

shift = {
    ("|", (1, 0)): (1, 0),
    ("|", (-1, 0)): (-1, 0),
    ("F", (0, -1)): (1, 0),
    ("F", (-1, 0)): (0, 1),
    ("-", (0, 1)): (0, 1),
    ("-", (0, -1)): (0, -1),
    ("L", (0, -1)): (-1, 0),
    ("L", (1, 0)): (0, 1),
    ("J", (0, 1)): (-1, 0),
    ("J", (1, 0)): (0, -1),
    ("7", (0, 1)): (1, 0),
    ("7", (-1, 0)): (0, -1),
}

start = None
for i in range(len(map)):
    for j in range(len(map[0])):
        if map[i][j] == 'S':
            start = (i, j)

def find_first_move(start):
    start_row, start_col = start
    if start_row > 0 and map[start_row-1][start_col] in ['7', '|', 'F']:
        return '|', (-1, 0)
    if start_row < len(map) - 1 and map[start_row+1][start_col] in ['J', '|', 'L']:
        return '|', (1, 0)
    if start_col < len(map[0]) - 1 and map[start_row][start_col+1] in ['J', '-', '7']:
        return '-', (0, 1)
    if start_col > 0 and map[start_row][start_col-1] in ['L', '-', 'F']:
        return '-', (0, -1)

def find_path():
    sym, dir = find_first_move(start)
    i, j = start
    res = [start]
    while True:
        dir = shift[sym, dir]
        di, dj = dir
        i, j = i + di, j + dj
        if (i, j) == start:
            break
        sym = map[i][j]
        res.append((i, j))
    return res

def shoelace_area(path):
    res = 0
    for i in range(len(path)):
        x1,y1 = path[i]
        x2, y2 = path[(i + 1) % len(path)]
        res += (x1 * y2) - (x2 * y1)
    return abs(res) // 2

path = find_path()

# https://en.wikipedia.org/wiki/Shoelace_formula
area = shoelace_area(path)

# https://en.wikipedia.org/wiki/Pick%27s_theorem
print(area - (len(path) // 2) + 1)