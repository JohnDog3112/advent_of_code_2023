def parse_input():
    f = open("input.txt")
    inp = f.readlines()
    f.close()

    return inp[0].strip("\n").split(",")

def hash(inp):
    val = 0
    for ch in inp:
        val += ord(ch)
        val *= 17
        val %= 256
    
    return val

def part1(inp):
    total = 0

    for line in inp:
        total += hash(line)
        
    return total


boxes = [[] for _ in range(256)]
def add_lens(label, length):
    global boxes
    box_index = hash(label)
    for lense in boxes[box_index]:
        if lense[0] == label:
            lense[1] = length
            return
    
    boxes[box_index].append([label, length])

def remove_lens(label):
    global boxes
    box_index = hash(label)

    to_pop = -1
    for i, lense in enumerate(boxes[box_index]):
        if lense[0] == label:
            to_pop = i
            break
    
    if to_pop != -1:
        boxes[box_index].pop(to_pop)
    
def part2(inp):
    global boxes
    for line in inp:
        if line[-1] == "-":
            remove_lens(line[:-1])
        else:
            sp = line.split("=")
            add_lens(sp[0], int(sp[1]))
    
    total = 0
    for i, box in enumerate(boxes):
        for j, lense in enumerate(box):
            total += (i+1)*(j+1)*lense[1]
    return total

inp = parse_input()

sol1 = part1(inp)
print("part 1:", sol1)

sol2 = part2(inp)
print("part 2:", sol2)