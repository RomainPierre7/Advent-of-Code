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

points = []
N = 26501365
Q = N // len(MAP)

positions = {(START, (0, 0))}
for step in range(1, N+1):
    new_positions = set()
    for pos in positions:
        coord, grid_number = pos
        i, j = coord
        gi, gj = grid_number
        if MAP[(i-1) % len(MAP)][j] == '.':
            if i > 0:
                new_positions.add(((i-1 , j), grid_number))
            else:
                new_positions.add((((i-1) % len(MAP), j), (gi-1, gj)))
        if MAP[(i+1) % len(MAP)][j] == '.':
            if i < len(MAP) - 1:
                new_positions.add(((i+1, j), grid_number))
            else:
                new_positions.add((((i+1) % len(MAP), j), (gi+1, gj)))
        if MAP[i][(j-1) % len(MAP[0])] == '.':
            if j > 0:
                new_positions.add(((i, j-1), grid_number))
            else:
                new_positions.add(((i, (j-1) % len(MAP[0])), (gi, gj-1)))
        if MAP[i][(j+1) % len(MAP[0])] == '.':
            if j < len(MAP[0]) - 1:
                new_positions.add(((i, j+1), grid_number))
            else:
                new_positions.add(((i, (j+1) % len(MAP[0])), (gi, gj+1)))
    positions = new_positions
    if step % len(MAP) == N % len(MAP):
        points.append(len(positions))
        if len(points) == 3:
            break

a = (points[2] + points[0] - 2 * points[1]) // 2
b = points[1] - points[0] - a
c = points[0]

print(a * Q**2 + b * Q + c)