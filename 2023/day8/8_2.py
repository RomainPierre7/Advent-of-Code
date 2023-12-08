import os
import math

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '8_input.txt')
input_file = open(file_path, 'r')

inst = input_file.readline().strip()
input_file.readline()

nodes = []
nodes_elem = []
for line in input_file:
    node, elems = line.split('=')
    nodes.append(node.strip())
    left, right = elems.strip().split(',')
    nodes_elem.append([left[1:], right[1:-1]])

starting_nodes = [index for index, node in enumerate(nodes) if node.endswith('A')]

def step_to_z(node):
    inst_number = 0
    count = 0
    while not nodes[node].endswith('Z'):
        if inst[inst_number] == 'L':
            node = nodes.index(nodes_elem[node][0])
        else :
            node = nodes.index(nodes_elem[node][1])
        inst_number = (inst_number + 1) % len(inst)
        count += 1
    return count

steps = [step_to_z(node) for node in starting_nodes]

print(math.lcm(*steps))