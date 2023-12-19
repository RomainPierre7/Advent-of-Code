import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '19_input.txt')
input_file = open(file_path, 'r')

WORKFLOWS = []
ACCEPTED = []

X, M, A, S = 0, 1, 2, 3

for line in input_file:
    if line.strip() == "":
        break
    name, rest = line.strip().split('{')
    rest = rest[:-1].split(',')
    for i, wf in enumerate(rest):
        wf = wf.split(':')
        if len(wf) == 1:
            rule, dest = None, *wf
        else:
            rule, dest = wf[0], wf[1]
        rest[i] = (rule, dest)
    WORKFLOWS.append((name, rest))

def combination_number(ranges):
    is_0 = True
    res = 1
    for rng in ranges:
        if rng[1] - 1 > rng[0]:
            is_0 = False
            res *= rng[1] - rng[0]
    return res if not is_0 else 0

def combinations(ranges, wf_name):
    if wf_name == 'A':
        return combination_number(ranges)
    if wf_name == 'R':
        return 0
    
    res = 0
    current_wf = next((wf for wf in WORKFLOWS if wf[0] == wf_name), None)
    for rule in current_wf[1]:
        cond, dest = rule[0], rule[1]

        if cond == None:
            res += combinations(ranges, dest)
            break
        
        match cond[0]:
            case 'x':
                category = X
            case 'm':
                category = M
            case 'a':
                category = A
            case 's':
                category = S

        value = int(cond[2:])
        new_ranges = ranges.copy()
        if cond[1] == '>':
            if ranges[category][1] - 1 > value:
                new_ranges[category] = (value + 1, ranges[category][1])
                res += combinations(new_ranges, dest)
            if ranges[category][0] < value:
                ranges[category] = (ranges[category][0], value + 1)
        elif cond[1] == '<':
            if ranges[category][0] < value:
                new_ranges[category] = (ranges[category][0], value)
                res += combinations(new_ranges, dest)
            if ranges[category][1] - 1 > value:
                ranges[category] = (value, ranges[category][1])
    return res

print(combinations([(1, 4001), (1, 4001), (1, 4001), (1, 4001)], "in"))