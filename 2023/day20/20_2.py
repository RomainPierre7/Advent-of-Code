import os
import math

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '20_input.txt')
input_file = open(file_path, 'r')

MODULES = []
for line in input_file:
    mod, dest = line.split('->')
    mod, dest = mod.strip(), dest.split(',')
    dest = [d.strip() for d in dest]
    if mod != "broadcaster":
        func = mod[0]
        mod = mod[1:]
    else:
        func = "broadcast"
    MODULES.append((func, mod, dest))

LOW, HIGH = 0, 1
OFF, ON = 0, 1

messages_to_send = [[] for _ in range(len(MODULES))]
module_status = {i: OFF for i, mod in enumerate(MODULES) if mod[0] == '%'}
conjunction_memory = {i: {} for i, mod in enumerate(MODULES) if mod[0] == '&'}

for i in conjunction_memory.keys():
    conj = MODULES[i]
    for mod in MODULES:
        if conj[1] in mod[2]:
            conjunction_memory[i][mod[1]] = LOW

def clear_messages_to_send():
    global messages_to_send
    messages_to_send = [[] for _ in range(len(MODULES))]

def index_node(node: str):
    for i, mod in enumerate(MODULES):
        if mod[1] == node:
            return i
        
def send(value: int, sender: str, dest: list[str]):
    for d in dest:
        idx = index_node(d)
        if idx != None:
            messages_to_send[idx].append((value, sender))

def process(messages_to_process: list[int], idx: int):
    mod = MODULES[idx]
    for message, sender in messages_to_process[idx]:
        if mod[0] == "broadcast":
            send(message, mod[1], mod[2])
        elif mod[0] == '%':
            if message == LOW:
                if module_status[idx] == OFF:
                    send(HIGH, mod[1], mod[2])
                    module_status[idx] = ON
                else:
                    send(LOW, mod[1], mod[2])
                    module_status[idx] = OFF
        elif mod[0] == '&':
            conjunction_memory[idx][sender] = message
            if all(value == HIGH for value in conjunction_memory[idx].values()):
                send(LOW, mod[1], mod[2])
            else:
                send(HIGH, mod[1], mod[2])

def push_button(nodes: dict[str: int], value: int, count: int):
    send(LOW, "button", ["broadcaster"])
    while(True):
        messages_to_process = messages_to_send.copy()
        clear_messages_to_send()
        for i in range(len(messages_to_process)):
            if messages_to_process[i] != []:
                process(messages_to_process, i)
        for node in nodes:
            idx = index_node(node)
            if idx != None:
                if nodes[node] == -1:
                    for message, _ in messages_to_process[idx]:
                        if message == value:
                            nodes[node] = count              
        if all(m == [] for m in messages_to_send):
            break

nodes = {}
for mod in MODULES:
    if "rx" in mod[2]:
        pre_node = mod[1]
        break
for mod in MODULES:
    if pre_node in mod[2]:
        nodes[mod[1]] = -1

count = 1
while any(nodes[node] == -1 for node in nodes):
    push_button(nodes, LOW, count)
    count += 1

print(math.lcm(*nodes.values()))