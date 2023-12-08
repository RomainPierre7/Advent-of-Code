import os

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

count = 0
current_node = nodes.index("AAA")
inst_number = 0

while current_node != nodes.index("ZZZ"):
    if inst[inst_number] == 'L':
        current_node = nodes.index(nodes_elem[current_node][0])
    else :
        current_node = nodes.index(nodes_elem[current_node][1])
    inst_number = (inst_number + 1) % len(inst)
    count += 1

print(count)