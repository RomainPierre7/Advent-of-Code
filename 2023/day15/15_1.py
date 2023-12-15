import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '15_input.txt')
input_file = open(file_path, 'r')

SEQUENCE = []
for line in input_file:
    SEQUENCE.extend(line.strip().split(','))

def HASH(str):
    res = 0
    for c in str:
        res += ord(c)
        res *= 17
        res %= 256
    return res

res = 0
for str in SEQUENCE:
    res += HASH(str)

print(res)