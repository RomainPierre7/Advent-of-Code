import os
import heapq

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '17_input.txt')
input_file = open(file_path, 'r')

GRID = []
for i, line in enumerate(input_file):
    GRID.append([int(c) for c in line.strip()])

def get_neighbors(grid, node):
    neighbors = []
    i, j, same_dir_count, dir = node
    di, dj = dir
    left, right = (-dj, di), (dj,-di)
    if same_dir_count < 3 and 0 <= i + di < len(grid) and 0 <= j + dj < len(grid[0]):
        neighbors.append(((i + di, j + dj, same_dir_count + 1, dir), grid[i + di][j + dj]))
    for di, dj in left, right:
        if 0 <= i + di < len(grid) and 0 <= j + dj <len(grid[0]):
            neighbors.append(((i + di, j + dj, 1, (di,dj)), grid[i + di][j + dj]))
    return neighbors 

def pathfind(grid):
    visited = set()
    start_down, start_right = (0,0,0,(1,0)),(0,0,0,(0,1))
    dist = {start_down: 0, start_right: 0}
    Q = [(0, start_down), (0, start_right)]
    target = (len(grid) - 1, len(grid[0]) - 1)
    while len(Q):
        _, node = heapq.heappop(Q)
        if node in visited:
            continue
        visited.add(node)
        if node[:2] == target:
            target = node
            break
        for v, cost in get_neighbors(grid, node):
            if v in visited:
                continue
            new_dist = dist[node] + cost
            if not v in dist or new_dist < dist[v]:              
                dist[v] = new_dist
                heapq.heappush(Q, (new_dist, v))
    return dist[target]

print(pathfind(GRID))