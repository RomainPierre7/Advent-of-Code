import os
import sympy

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '24_input.txt')
input_file = open(file_path, 'r')

HAILSTONES = []
for line in input_file:
    coords, speeds = line.split('@')
    x, y, z = [int(c) for c in coords.split(',')]
    vx, vy, vz = [int(c) for c in speeds.split(',')]
    HAILSTONES.append([x, y, z, vx, vy, vz])

x, y, z, dx, dy, dz = sympy.symbols('x y z dx dy dz')
symbols = [x, y, z, dx, dy, dz]
equations = []

for i, h in enumerate(HAILSTONES[:3]): # 3 points are enough to ensure alignment
    xi, yi, zi, vxi, vyi, vzi = h
    ti = sympy.symbols('n' + str(i))
    symbols.append(ti)
    equations.append(sympy.Eq(x + ti * dx, xi + ti * vxi))
    equations.append(sympy.Eq(y + ti * dy, yi + ti * vyi))
    equations.append(sympy.Eq(z + ti * dz, zi + ti * vzi))

sol = sympy.solve(tuple(equations), tuple(symbols))

print(sol[0][0] + sol[0][1] + sol[0][2])