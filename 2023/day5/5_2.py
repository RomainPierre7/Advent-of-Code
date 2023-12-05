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

def map_range(maps, src_range):
    ranges = []
    ranges_output = []
    unmapped = []
    cursor = src_range[0]
    for map in maps:
        dst_rng_start = map[0]
        src_rng_start = map[1]
        map_rng_length = map[2]
        mapped = [max(src_range[0], src_rng_start), min(src_range[1], src_rng_start + map_rng_length - 1)]
        if mapped[0] <= mapped[1]:
            ranges.append(mapped)
        if dst_rng_start <= dst_rng_start + mapped[1] - mapped[0]:
            ranges_output.append([dst_rng_start + mapped[0] - src_rng_start, dst_rng_start + mapped[1] - src_rng_start])
    ranges = sorted(ranges, key=lambda x: x[0])
    if len(ranges) > 0:
        for m in ranges:
            if (cursor < m[0]-1):
                unmapped.append([cursor, m[0]-1])
            cursor = m[1]+1
        unmapped.append([m[1]+1, src_range[1]])
        for un in unmapped:
            ranges_output.append(un)
    else :
        return [src_range]
    return ranges_output

def recur_map(i, ranges):
    result = []
    if i == len(maps):
        return ranges
    for j in range(len(ranges)):
        new_ranges = map_range(maps[i], ranges[j])
        for rng in new_ranges:
            result.append(rng)
    return recur_map(i+1, result)
    
ranges = []
for i in range(0, len(seeds), 2):
    ranges.append([seeds[i], seeds[i] + seeds[i+1] - 1])

result = recur_map(0, ranges)

print(min(result, key=lambda x: x[0])[0])