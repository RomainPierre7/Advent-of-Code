import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '18_input.txt')
input_file = open(file_path, 'r')

DATA = []
for line in input_file:
    _, _, color = line.strip().split()
    val = int(color[2:-2], 16)
    dir = color[-2]
    match dir:
        case '0':
            dir = 'R'
        case '1':
            dir = 'D'
        case '2':
            dir = 'L'
        case '3':
            dir = 'U'
    DATA.append((dir, int(val)))

current = (0, 0)
path = []
count = 0
for line in DATA:
    dir, val = line
    if dir ==   'L':
        path.append((current[0], current[1] - val))
        current = (current[0], current[1] - val)
    elif dir == 'R':
        path.append((current[0], current[1] + val))
        current = (current[0], current[1] + val)
    elif dir == 'U':
        path.append((current[0] - val, current[1]))
        current = (current[0] - val, current[1])
    elif dir == 'D':
        path.append((current[0] + val, current[1]))
        current = (current[0] + val, current[1])
    count += val

def shoelace_area(path):
    res = 0
    for i in range(len(path)):
        x1,y1 = path[i]
        x2, y2 = path[(i + 1) % len(path)]
        res += (x1 * y2) - (x2 * y1)
    return abs(res) // 2

area = shoelace_area(path)

print(area + (count // 2) + 1)