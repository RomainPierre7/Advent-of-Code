import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '15_input.txt')
input_file = open(file_path, 'r')

SEQUENCE = []
for line in input_file:
    SEQUENCE.extend(line.strip().split(','))

BOXES = [[] for _ in range(256)]

def HASH(str):
    res = 0
    for c in str:
        res += ord(c)
        res *= 17
        res %= 256
    return res

for str in SEQUENCE:
    if '=' in str:
        lens, value = str.split('=')
        box_number = HASH(lens)
        matching_indices = [idx for idx, element in enumerate(BOXES[box_number]) if element.startswith(lens)]
        if matching_indices:
            BOXES[box_number][matching_indices[0]] = lens + ' ' + value
        else:
            BOXES[HASH(lens)].append(lens + ' ' + value)
    elif '-' in str:
        lens = str[:-1]
        box_number = HASH(lens)
        matching_elements = [element for element in BOXES[box_number] if element.startswith(lens)]
        if matching_elements:
            BOXES[box_number].remove(matching_elements[0])

res = 0
for i in range(len(BOXES)):
    for j in range(len(BOXES[i])):
        focal = int(BOXES[i][j].split()[1])
        res += (i + 1) * (j + 1) * focal

print(res)