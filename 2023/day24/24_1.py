import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '24_input.txt')
input_file = open(file_path, 'r')

MINIMUM = 200000000000000
MAXIMUM = 400000000000000

HAILSTONES = []
for line in input_file:
    coords, speeds = line.split('@')
    x, y, z = [int(c) for c in coords.split(',')]
    vx, vy, vz = [int(c) for c in speeds.split(',')]
    HAILSTONES.append([x, y, z, vx, vy, vz])

count = 0
for i in range(len(HAILSTONES)-1):
    for j in range(i+1, len(HAILSTONES)):
        x1, y1, _, vx1, vy1, _ = HAILSTONES[i]
        x2, y2, _, vx2, vy2, _ = HAILSTONES[j]
        
        a1 = vy1 / vx1
        b1 = y1 - a1 * x1

        a2 = vy2 / vx2
        b2 = y2 - a2 * x2

        if a1 == a2:    
            continue

        x = (b2 - b1) / (a1 - a2)
        y = a1 * x + b1

        # Check if the intersection is in the future
        temp_sens_1 = (x - x1) / vx1
        temp_sens_2 = (x - x2) / vx2

        if temp_sens_1 < 0 or temp_sens_2 < 0:
            continue

        if MINIMUM <= x <= MAXIMUM and MINIMUM <= y <= MAXIMUM:
            count += 1

print(count)