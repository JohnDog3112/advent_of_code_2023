
def parse_input():
    f = open("input.txt")
    inp = f.readlines()
    f.close()

    times = list(map(lambda x: int(x), inp[0].split()[1:]))
    distances = list(map(lambda x: int(x), inp[1].split()[1:]))

    return (times, distances)

def part1(times, distances):
    out = 1
    for i in range(len(times)):
        time = times[i]
        distance = distances[i]

        beats_distance = 0

        for j in range(0, time+1):
            run_distance = j * (time - j)
            if distance < run_distance:
                beats_distance += 1
        
        out *= beats_distance
    return out

def part2(times, distances):
    time_str = ""
    for time in times:
        time_str += str(time)
    distance_str = ""
    for distance in distances:
        distance_str += str(distance)
    
    return part1([int(time_str)], [int(distance_str)])

(times, distances) = parse_input()

sol1 = part1(times, distances)
print("part1:",sol1)

sol2 = part2(times,distances)
print("part2",sol2)