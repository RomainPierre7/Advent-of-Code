import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '6_input.txt')
input_file = open(file_path, 'r')

times = input_file.readline().split(':')[-1].split()
distances = input_file.readline().split(':')[-1].split()

time = ""
for i in range(len(times)):
    time += times[i]
time = int(time)

distance = ""
for i in range(len(distances)):
    distance += distances[i]
distance = int(distance)

def race(hold_time, total_time):
    left_time = total_time - hold_time
    return left_time * hold_time

count = 0
for j in range(time):
    dist = race(j, time)
    if dist > distance:
        count += 1

print(count)