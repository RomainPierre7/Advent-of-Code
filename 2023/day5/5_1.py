import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '5_input.txt')
input_file = open(file_path, 'r')

input_length = len(input_file.readlines())
input_file.seek(0)

seeds = input_file.readline().split(':')[1].split()
seeds = [int(i) for i in seeds]
input_file.readline()

maps = []

new_map = []
for i in range(input_length - 2):
    line = input_file.readline()
    if (line[0].isdigit()):
        new_map.append(line.strip())
        if i == input_length - 3:
            maps.append(new_map)
    else:
        if len(new_map) > 0:
            maps.append(new_map)
            new_map = []

for i in range(len(maps)):
    for j in range(len(maps[i])):
        maps[i][j] = maps[i][j].split()
        for k in range(len(maps[i][j])):
            maps[i][j][k] = int(maps[i][j][k])

def map_seed(maps, src_number):
    for map in maps:
        dst_rng_start = map[0]
        src_rng_start = map[1]
        rng_length = map[2]
        if src_number >= src_rng_start and src_number < src_rng_start + rng_length:
            return dst_rng_start + (src_number - src_rng_start)
    return src_number

locations = []
for seed in seeds:
    for i in range(len(maps)):
        seed = map_seed(maps[i], seed)
    locations.append(seed)

print(min(locations))