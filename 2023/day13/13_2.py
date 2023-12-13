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

def is_smudge_equal(line1, line2):
    smudge = 0
    for i in range(len(line1)):
        if line1[i] != line2[i]:
            if smudge == 1:
                return False
            smudge += 1
    return True

def horizontal_reflection(pattern, with_smudge, exclude_line):
    n = len(pattern)
    is_horizontal = False
    smudge_used = False
    for i in range(n-1):
        if i == exclude_line:
            continue
        top = i
        bot = i + 1
        while top >= 0 and bot < n:
            if pattern[top] == pattern[bot]:
                is_horizontal = True
            elif with_smudge and not smudge_used and is_smudge_equal(pattern[top], pattern[bot]):
                is_horizontal = True
                smudge_used = True
            else:
                is_horizontal = False
                smudge_used = False
                break
            top -= 1
            bot += 1
        if is_horizontal:
            return True, 100 * (i+1), i
    return False, 0, 0

def vertical_reflection(pattern, with_smudge, exclude_line):    
    n = len(pattern[0])
    is_vertical = False
    smudge_used = False
    for i in range(n-1):
        if i == exclude_line:
            continue
        left = i
        right = i + 1
        while left >= 0 and right < n:
            left_pattern = [line[left] for line in pattern]
            right_pattern = [line[right] for line in pattern]
            if left_pattern == right_pattern:
                is_vertical = True
            elif with_smudge and not smudge_used and is_smudge_equal(left_pattern, right_pattern):
                is_vertical = True
                smudge_used = True
            else:
                is_vertical = False
                smudge_used = False
                break
            left -= 1
            right += 1
        if is_vertical:
            return True, i+1, i
    return False, 0, 0

res = 0
for pattern in PATTERNS:
    is_horizontal, _, horiz_exclude = horizontal_reflection(pattern, with_smudge=False, exclude_line=-1)

    if is_horizontal:
        is_horizontal, horiz_res, _ = horizontal_reflection(pattern, with_smudge=True, exclude_line=horiz_exclude)
        if is_horizontal:
            res += horiz_res
        else:
            res += vertical_reflection(pattern, with_smudge=True, exclude_line=-1)[1]
    
    else:
        is_vertical, _, vert_exclude = vertical_reflection(pattern, with_smudge=False, exclude_line=-1)
        if is_vertical:
            is_vertical, vert_res, _ = vertical_reflection(pattern, with_smudge=True, exclude_line=vert_exclude)
            if is_vertical:
                res += vert_res
            else:
                res += horizontal_reflection(pattern, with_smudge=True, exclude_line=-1)[1]
            
print(res)