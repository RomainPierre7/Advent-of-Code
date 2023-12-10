import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '10_input.txt')
input_file = open(file_path, 'r')

map = []
for line in input_file:
    map.append([c for c in line.strip()])

distance = [[-1 for _ in map[0]] for _ in map]

current_tiles = []
for i in range(len(map)):
    for j in range(len(map[0])):
        if map[i][j] == 'S':
            distance[i][j] = 0
            current_tiles.append((i, j))

current_distance = 1
while current_tiles != []:
    new_tiles = []
    for (i, j) in current_tiles:
        if i > 0 and map[i-1][j] in ['7', '|', 'F'] and distance[i-1][j] == -1:
            distance[i-1][j] = current_distance
            new_tiles.append((i-1, j))
        if j < len(map[0]) -1 and map[i][j+1] in ['J', '-', '7'] and distance[i][j+1] == -1:
            distance[i][j+1] = current_distance
            new_tiles.append((i, j+1))
        if i < len(map) - 1 and map[i+1][j] in ['L', '|', 'J'] and distance[i+1][j] == -1:
            distance[i+1][j] = current_distance
            new_tiles.append((i+1, j))
        if j > 0 and map[i][j-1] in ['F', '-', 'L'] and distance[i][j-1] == -1:
            distance[i][j-1] = current_distance
            new_tiles.append((i, j-1))
    current_tiles = new_tiles
    current_distance += 1

print(current_distance - 2)