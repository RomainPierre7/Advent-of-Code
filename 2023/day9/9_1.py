import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '9_input.txt')
input_file = open(file_path, 'r')

histories = []
for line in input_file:
    histories.append([int(value) for value in line.split()])

def is_all_zeroes(sequence):
    for num in sequence:
        if num != 0:
            return False
    return True

def next_sequence(sequence):
    next = []
    for i in range (1, len(sequence)):
        next.append(sequence[i] - sequence[i-1])
    return next

result = 0

for history in histories:
    sequences = [history]
    while not is_all_zeroes(sequences[-1]):
        sequences.append(next_sequence(sequences[-1]))
    
    sequences[-1].append(0)
    for i in range(len(sequences) - 2, -1, -1):
        sequences[i].append(sequences[i][-1] + sequences[i+1][-1])
    result += sequences[0][-1]

print(result)