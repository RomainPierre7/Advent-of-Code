import os

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
is_running = False

module_status = {i: OFF for i, mod in enumerate(MODULES) if mod[0] == '%'}
conjunction_memory = {i: {} for i, mod in enumerate(MODULES) if mod[0] == '&'}
messages_to_send = [[] for _ in MODULES]

def clear_messages_to_send():
    global messages_to_send
    messages_to_send = [[] for _ in MODULES]

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

def count(messages_to_send):
    low_count, high_count = 0, 0
    for messages in messages_to_send:
        for message in messages:
            if message[0] == 0:
                low_count += 1
            else:
                high_count += 1
    return low_count, high_count


def start():
    global is_running
    is_running = True
    send(LOW, "button", ["broadcaster"])
    low_count, high_count = 1, 0
    while(is_running):
        messages_to_process = messages_to_send.copy()
        clear_messages_to_send()
        for i in range(len(messages_to_process)):
            if messages_to_process[i] != []:
                process(messages_to_process, i)
        if all(m == [] for m in messages_to_send):
            is_running = False
        new_low_count, new_high_count = count(messages_to_send)
        low_count += new_low_count
        high_count += new_high_count
    return low_count, high_count

low_count, high_count = 0, 0
for _ in range(1000):
    l, h = start()
    low_count += l
    high_count += h

print(low_count * high_count)