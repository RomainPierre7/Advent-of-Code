import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '14_input.txt')
input_file = open(file_path, 'r')

PLATFORM = []
for line in input_file:
    PLATFORM.append([c for c in line.strip()])

PLATFORM = tuple(tuple(line) for line in PLATFORM)

def north_tilt(platform):
    res = list(list(c) for c in platform)
    for i in range(len(res)):
        for j in range(len(res[0])):
            if res[i][j] == 'O':
                row, col = i, j
                while row > 0:
                    if res[row-1][col] == '.':
                        row -= 1
                    else:
                        break
                res[i][j], res[row][col] = res[row][col], res[i][j]
    return tuple(tuple(line) for line in res)

def west_tilt(platform):
    res = list(list(c) for c in platform)
    for i in range(len(res)):
        for j in range(len(res[0])):
            if res[i][j] == 'O':
                row, col = i, j
                while col > 0:
                    if res[row][col-1] == '.':
                        col -= 1
                    else:
                        break
                res[i][j], res[row][col] = res[row][col], res[i][j]
    return tuple(tuple(line) for line in res)

def south_tilt(platform):
    res = list(list(c) for c in platform)
    for i in range(len(res)-1, -1, -1):
        for j in range(len(res[0])):
            if res[i][j] == 'O':
                row, col = i, j
                while row < len(res)-1:
                    if res[row+1][col] == '.':
                        row += 1
                    else:
                        break
                res[i][j], res[row][col] = res[row][col], res[i][j]
    return tuple(tuple(line) for line in res)

def east_tilt(platform):
    res = list(list(c) for c in platform)
    for i in range(len(res)):
        for j in range(len(res[0])-1, -1, -1):
            if res[i][j] == 'O':
                row, col = i, j
                while col < len(res[0])-1:
                    if res[row][col+1] == '.':
                        col += 1
                    else:
                        break
                res[i][j], res[row][col] = res[row][col], res[i][j]
    return tuple(tuple(line) for line in res)

CACHE = {}
is_cycle = False

def tilt_cycle_cached(platform):
    global CACHE, is_cycle

    def tilt_cycle(platform):
        platform = north_tilt(platform)
        platform = west_tilt(platform)
        platform = south_tilt(platform)
        platform = east_tilt(platform)
        return platform
    if platform in CACHE:
        is_cycle = True
        return CACHE[platform]
    else:
        res = tilt_cycle(platform)
        CACHE[platform] = res
        return res

# Filling the cache and the beginning of the cycle
while not is_cycle:
    PLATFORM = tilt_cycle_cached(PLATFORM)
START_CYCLE_PLATFORM = PLATFORM

# Finding the cycle length
cycle_length = 1
PLATFORM = tilt_cycle_cached(PLATFORM)
while PLATFORM != START_CYCLE_PLATFORM:
    PLATFORM = tilt_cycle_cached(PLATFORM)
    cycle_length += 1

# Finding the final platform
start_to_cycle_length = len(CACHE)

left_cycle_number = (1000000000 - start_to_cycle_length - 1) % cycle_length

PLATFORM = START_CYCLE_PLATFORM
for i in range(left_cycle_number):
    PLATFORM = tilt_cycle_cached(PLATFORM)

res = 0
for i in range(len(PLATFORM)):
    for j in range(len(PLATFORM[0])):
        if PLATFORM[i][j] == 'O':
            res += len(PLATFORM) - i

print(res)