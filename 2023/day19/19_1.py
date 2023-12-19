import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '19_input.txt')
input_file = open(file_path, 'r')

WORKFLOWS = []
PIECES = []
ACCEPTED = []

X, M, A, S = 0, 1, 2, 3

add_piece = False
for line in input_file:
    if not add_piece:
        if line.strip() == "":
            add_piece = True
            continue
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
    else:
        line = line.strip()[1:-1].split(',')
        piece_stats = []
        for i in range(4):
            piece_stats.append(int(line[i][2:]))
        PIECES.append(tuple(piece_stats))

def is_accepted(piece, wf_name):
    if wf_name == 'A':
        return True
    if wf_name == 'R':
        return False
    
    current_wf = next((wf for wf in WORKFLOWS if wf[0] == wf_name), None)
    for rule in current_wf[1]:
        cond, dest = rule[0], rule[1]

        if cond == None:
            return is_accepted(piece, dest)
        
        match cond[0]:
            case 'x':
                category = X
            case 'm':
                category = M
            case 'a':
                category = A
            case 's':
                category = S

        if cond[1] == '>':
            if piece[category] > int(cond[2:]):
                return is_accepted(piece, dest)
        if cond[1] == '<':
            if piece[category] < int(cond[2:]):
                return is_accepted(piece, dest)
    return False

for piece in PIECES:
    if is_accepted(piece, "in"):
        ACCEPTED.append(piece)

result = sum(sum(p) for p in ACCEPTED)

print(result)