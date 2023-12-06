import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '6_input.txt')
input_file = open(file_path, 'r')

times = input_file.readline().split(':')[-1].split()
times = [int(i) for i in times]

distances = input_file.readline().split(':')[-1].split()
distances = [int(i) for i in distances]

def race(hold_time, total_time):
    left_time = total_time - hold_time
    return left_time * hold_time

total = 1
for i in range(len(times)):
    count = 0
    for j in range(times[i]):
        dist = race(j, times[i])
        if dist > distances[i]:
            count += 1
    total *= count

print(total)