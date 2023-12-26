import os
from collections import defaultdict

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '23_input.txt')
input_file = open(file_path, 'r')

GRID = []
for line in input_file:
    GRID.append([c for c in line.strip()])

START = (0, GRID[0].index('.'))
END = (len(GRID) - 1, GRID[-1].index('.'))

g = defaultdict(list)
q = [(START, START, {START, (0, 1)})]

while q:
    current, prev_node, visited = q.pop()

    if current == END:
        last_node = prev_node
        last_steps = len(visited)-1
        continue

    i, j = current
    neighbors = []
    if i > 0 and GRID[i-1][j] != '#' and (i-1, j) not in visited:
        neighbors.append((i-1, j))
    if i < len(GRID)-1 and GRID[i+1][j] != '#' and (i+1, j) not in visited:
        neighbors.append((i+1, j))
    if j > 0 and GRID[i][j-1] != '#' and (i, j-1) not in visited:
        neighbors.append((i, j-1))
    if j < len(GRID[0])-1 and GRID[i][j+1] != '#' and (i, j+1) not in visited:
        neighbors.append((i, j+1))

    if len(neighbors) == 1:
        next_coord = neighbors.pop()
        q.append((next_coord, prev_node, visited|{next_coord}))

    elif len(neighbors) > 1:
        steps = len(visited) - 1
        if (current, steps) in g[prev_node]:
            continue
        g[prev_node].append((current, steps))
        g[current].append((prev_node, steps))
        while neighbors:
            next_coord = neighbors.pop()
            q.append((next_coord, current, {current, next_coord}))

max_steps = 0
queue = [(START, 0, {START})]
while queue:
    current, steps, visited = queue.pop()
    if current == last_node:
        max_steps = max(steps, max_steps)
        continue
    for next_node, distance in g[current]:
        if next_node not in visited:
            queue.append((next_node, steps+distance, visited|{next_node}))

print(max_steps + last_steps)