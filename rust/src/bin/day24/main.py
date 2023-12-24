# stores hailstones as a tuple
# first three numbers = position, next three = velocity
hailstones = []
with open("src/bin/day24/input.txt", "r") as file:
    hailstones = file.read().splitlines()
    for i, hailstone in enumerate(hailstones):
        hailstone = hailstone.replace(",", "").replace("@", "")
        hailstones[i] = list(map(int, hailstone.split()))
 
# neat observation: if two hailstones have the same velocity in a direction
# there are only so many values that the rock can be thrown to hit them both
# particularly, distance between the stones modulo-ed by the rock's velocity - the hailstone's velocity must be 0
# I find those possible velocities
def get_possible_velocities(a_x, b_x, original_v):
    possibilities = set()
    distance = b_x - a_x
    for v in range(-500, 500):
        if v == original_v:
            possibilities.add(v)
            continue
        if distance % (v - original_v) == 0:
            possibilities.add(v)
    return possibilities
 
# build a list of possible velocities for each pair of hailstones in each axis
# only accept what works for all hailstones, so only keep those within both the discovered and the already existing
# at the end, I should have a set of x, y, and z velocities that shoot through all hailstones
potential_velocities = {'x': None, 'y': None, 'z': None}
for i, a in enumerate(hailstones):
    for b in hailstones[:i]:
        for dim, a_p, b_p, a_v, b_v in zip("xyz", a[:4], b[:4], a[3:], b[3:]):
            if a_v == b_v:
                new_velocities = get_possible_velocities(a_p, b_p, a_v)
                if potential_velocities[dim] is None:
                    potential_velocities[dim] = new_velocities.copy()
                potential_velocities[dim] &= new_velocities
 
# you now have a velocity for the rock
# now get any two hailstones ( I believe any should work ) and reverse-engineer the rock position
rock_vx, rock_vy, rock_vz = [value.pop() for value in potential_velocities.values()]
a_x, a_y, a_z, a_vx, a_vy, a_vz = hailstones[0]
b_x, b_y, b_z, b_vx, b_vy, b_vz = hailstones[1]

print(rock_vx, rock_vy, rock_vz)
# y = mx + b, solve for m and b
ma = (a_vy - rock_vy) / (a_vx - rock_vx)
mb = (b_vy - rock_vy) / (b_vx - rock_vx)
ba = a_y - (ma * a_x)
bb = b_y - (mb * b_x)
 
print(ma, mb)
# set them equal to each other and solve for x
# m_a * x + b_a = m_b * x + b_b
# m_a * x - m_b * x = b_b - b_a
# x(m_a - m_b) = b_b - b_a
# x = (b_b - b_a) / (m_a - m_b)
x = int((bb - ba) / (ma - mb))
 
# plug the discovered x into one of the equations to get y
y = int(ma * x + ba)
 
# get the time the rock takes to hit the hailstone and derive z from that
time = (x - a_x) // (a_vx - rock_vx)
z = a_z + (a_vz - rock_vz) * time

print(x, y, z)
# this was a hard challenge, but the dopamine rush of finally being able to type out x + y + z was amazing
print(x + y + z)