import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '13_input.txt')
input_file = open(file_path, 'r')

PATTERNS = [[]]

for line in input_file:
    if line != "\n":
        PATTERNS[-1].append(line.strip())
    else :
        PATTERNS.append([])

def horizontal_reflection(pattern):
    n = len(pattern)
    is_horizontal = False
    for i in range(n-1):
        top = i
        bot = i + 1
        while top >= 0 and bot < n:
            if pattern[top] == pattern[bot]:
                is_horizontal = True
            else:
                is_horizontal = False
                break
            top -= 1
            bot += 1
        if is_horizontal:
            return True, 100 * (i+1)
    return False, 0

def vertical_reflection(pattern):
    n = len(pattern[0])
    is_vertical = False
    for i in range(n-1):
        left = i
        right = i + 1
        while left >= 0 and right < n:
            left_pattern = [line[left] for line in pattern]
            right_pattern = [line[right] for line in pattern]
            if left_pattern == right_pattern:
                is_vertical = True
            else:
                is_vertical = False
                break
            left -= 1
            right += 1
        if is_vertical:
            return i+1
    return 0

res = 0
for pattern in PATTERNS:
    is_horizontal, horiz_res = horizontal_reflection(pattern)
    if is_horizontal:
        res += horiz_res
    else:
        res += vertical_reflection(pattern)

print(res)