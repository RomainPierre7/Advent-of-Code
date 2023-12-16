import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '16_input.txt')
input_file = open(file_path, 'r')

MAP = []
for line in input_file:
    MAP.append([c for c in line.strip()])

def move_beam(i, j, di, dj, visited=None):
    visited = {} if visited == None else visited

    while i >= 0 and i < len(MAP) and j >= 0 and j < len(MAP[0]):
        dir = visited.setdefault((i, j), [])
        if (di, dj) in dir:
            break
        dir.append((di, dj))
        if MAP[i][j] == '-' and di != 0:
            move_beam(i, j+1, 0, 1, visited)
            di, dj = 0, -1
        elif MAP[i][j] == '|' and dj != 0:
            move_beam(i+1, j, 1, 0, visited)
            di, dj = -1, 0
        elif MAP[i][j] in '\\':
            di, dj = dj, di
        elif MAP[i][j] in '/':
            di, dj = -dj, -di
        i += di
        j += dj
    return len(visited)

energies = []
for i in range(len(MAP)):
    energies.append(move_beam(i, 0, 0, 1))
    energies.append(move_beam(i, len(MAP[0])-1, 0, -1))
for j in range(len(MAP[0])):
    energies.append(move_beam(0, j, 1, 0))
    energies.append(move_beam(len(MAP)-1, j, -1, 0))
print(max(energies))