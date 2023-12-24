import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '23_input.txt')
input_file = open(file_path, 'r')

GRID = []
for line in input_file:
    GRID.append(line.strip())

START = (0, GRID[0].index('.'))
END = (len(GRID) - 1, GRID[-1].index('.'))

RUNNING, INVALID, VALID = 0, 1, 2

paths = [[(START)]]
is_valid_path = {0: RUNNING}

def is_in_path(path, i, j):
    for x, y in path:
        if (x, y) == (i, j):
            return True
    return False

def next_step(path, path_id):
    i, j = path[-1]
    if (i, j) == END:
        is_valid_path[path_id] = VALID
        return
    if GRID[i][j] == '>':
        if (i, j-1) == path[-2]:
            paths[path_id].append((i, j+1))
        else:
            is_valid_path[path_id] = INVALID
        return
    elif GRID[i][j] == '<':
        if (i, j+1) == path[-2]:
            paths[path_id].append((i, j-1))
        else:
            is_valid_path[path_id] = INVALID
        return
    elif GRID[i][j] == 'v':
        if (i-1, j) == path[-2]:
            paths[path_id].append((i+1, j))
        else:
            is_valid_path[path_id] = INVALID
        return
    elif GRID[i][j] == '^':
        if (i+1, j) == path[-2]:
            paths[path_id].append((i-1, j))
        else:
            is_valid_path[path_id] = INVALID
        return
    add_path = False
    if i > 0 and GRID[i-1][j] in ".<>v^":
        if not is_in_path(path, i-1, j):
            add_path = True
            paths[path_id].append((i-1, j))
    if i < len(GRID)-1 and GRID[i+1][j] in ".<>v^":
        if not is_in_path(path, i+1, j):
            if add_path:
                paths.append(path[:-1])
                is_valid_path[len(paths) - 1] = RUNNING
                paths[-1].append((i+1, j))
            else:
                add_path = True
                paths[path_id].append((i+1, j))
    if j > 0 and GRID[i][j-1] in ".<>v^":
        if not is_in_path(path, i, j-1):
            if add_path:
                paths.append(path[:-1])
                is_valid_path[len(paths) - 1] = RUNNING
                paths[-1].append((i, j-1))
            else:
                add_path = True
                paths[path_id].append((i, j-1))
    if j < len(GRID[0])-1 and GRID[i][j+1] in ".<>v^":                        
        if not is_in_path(path, i, j+1):
            if add_path:
                paths.append(path[:-1])
                is_valid_path[len(paths) - 1] = RUNNING
                paths[-1].append((i, j+1))
            else:
                add_path = True
                paths[path_id].append((i, j+1))
    if not add_path:
        is_valid_path[path_id] = INVALID
    
while any(value == RUNNING for value in is_valid_path.values()):
    current_paths = paths[:]
    for idx, path in enumerate(current_paths):
        if is_valid_path[idx] == RUNNING:
            next_step(path, idx)

valid_path_lengths = [0 for _ in paths]
for idx, path in enumerate(paths):
    if is_valid_path[idx] == VALID:
        valid_path_lengths[idx] = len(path) - 1

print(max(valid_path_lengths))