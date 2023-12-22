import os

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '22_input.txt')
input_file = open(file_path, 'r')

BRICKS = []
X, Y, Z = 0, 1, 2

for line in input_file:
    coord1, coord2 = line.strip().split('~')
    coord1 = [int(c) for c in coord1.split(',')]
    coord2 = [int(c) for c in coord2.split(',')]
    sum_coord1 = sum(coord1)
    sum_coord2 = sum(coord2)
    if sum_coord1 > sum_coord2:
        coord1, coord2 = coord2, coord1
    BRICKS.append((coord1, coord2))
BRICKS = sorted(BRICKS, key=lambda brick: brick[0][2])

def is_occupied(x, y, z, bricks=BRICKS):
    for brick in bricks:
        if brick[0][X] <= x <= brick[1][X]:
            if brick[0][Y] <= y <= brick[1][Y]:
                if brick[0][Z] <= z <= brick[1][Z]:
                    return True
    return False

def is_falling(brick_id, bricks=BRICKS):
    brick = bricks[brick_id]
    if brick[0][Z] == 1:
        return False
    coord_to_check = []
    # brick along x-axis
    if brick[0][X] != brick[1][X]:
        for i in range(brick[0][X], brick[1][X] + 1):
            coord_to_check.append((i, brick[0][Y], brick[0][Z] - 1))
    # brick along y-axis
    elif brick[0][1] != brick[1][1]:
        for i in range(brick[0][Y], brick[1][Y] + 1):
            coord_to_check.append((brick[0][X], i, brick[0][Z] - 1))
    # brick along z-axis or contains only one cube
    else:
        coord_to_check.append((brick[0][X], brick[0][Y], brick[0][Z] - 1))
    for coord in coord_to_check:
        if is_occupied(*coord, bricks):
            return False
    return True

print("Falling bricks...")

for brick_id in range(len(BRICKS)):
    while is_falling(brick_id):
        brick = BRICKS[brick_id]
        BRICKS[brick_id] = ((brick[0][X], brick[0][Y], brick[0][Z] - 1), (brick[1][X], brick[1][Y], brick[1][Z] - 1))

total_count = 0
percentage = 0
for brick_id in range(len(BRICKS)):
    if brick_id / len(BRICKS) * 100 > percentage:
        percentage += 1
        print(percentage, '%')
    count = 0
    new_bricks = BRICKS.copy()
    del new_bricks[brick_id]
    for i in range(len(new_bricks)):
        if is_falling(i, new_bricks):
            count += 1
            brick = new_bricks[i]
            new_bricks[i] = ((brick[0][X], brick[0][Y], brick[0][Z] - 1), (brick[1][X], brick[1][Y], brick[1][Z] - 1))
        while is_falling(i, new_bricks):
            brick = new_bricks[i]
            new_bricks[i] = ((brick[0][X], brick[0][Y], brick[0][Z] - 1), (brick[1][X], brick[1][Y], brick[1][Z] - 1))
    total_count += count

print(total_count)