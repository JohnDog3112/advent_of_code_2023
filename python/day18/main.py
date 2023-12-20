def parse_input():
    f = open("test_input.txt")
    inp = f.readlines()
    f.close()

    return list(map(lambda x: x.strip("\n").split(" "), inp))


offset_map = {
    "R": (1,0),
    "D": (0,1),
    "L": (-1,0),
    "U": (0,-1)
}
def expand_grid(grid, width):
    for line in grid:
        for _ in range(len(line), width):
            line.append('.')
def part1(inp):
    grid = [[]]
    cur_pos = (0,0)
    grid_width = 1
    grid[0].append('S')
    start_pos = (0,0)
    for line in inp:
        offset = offset_map[line[0]]
        x = cur_pos[0] + offset[0]*int(line[1])
        y = cur_pos[1] + offset[1]*int(line[1])
        new_pos = (x,y)

        if x < 0:
            grid_width += abs(x)
            for line in grid:
                for _ in range(abs(x)):
                    line.insert(0,'.')
            new_pos = (0,new_pos[1])
            cur_pos = (cur_pos[0]-x, cur_pos[1])
            start_pos = (start_pos[0]-x, start_pos[1])
        if y < 0:
            for _ in range(abs(y)):
                grid.insert(0, [*("."*grid_width)])
            new_pos = (new_pos[0], 0)
            cur_pos = (cur_pos[0], cur_pos[1]-y)
            start_pos = (start_pos[0], start_pos[1]-y)
        
        for x in range(cur_pos[0]+1, new_pos[0]+1):
            if grid_width <= x:
                grid_width = x+1
                expand_grid(grid, grid_width)
            grid[new_pos[1]][x] = '#'
        
        for x in range(new_pos[0], cur_pos[0]):
            grid[new_pos[1]][x] = '#'
        
        for y in range(cur_pos[1]+1, new_pos[1]+1):
            if len(grid) <= y:
                grid.append([*("." * grid_width)])
            grid[y][new_pos[0]] = '#'
        
        for y in range(new_pos[1], cur_pos[1]):
            #print(y, new_pos[0], ":", len(grid), grid_width)
            
            grid[y][new_pos[0]] = '#'
        cur_pos = new_pos
    
    #for line in grid:
    #    print(''.join(line))
    grid[start_pos[1]][start_pos[0]] = "S"
    print("-------------------------")
    
    points = []
    for i, ch in enumerate(grid[0]):
        if ch == "#" and grid[1][i] == ".":
            points.append((i,1))
            grid[1][i] = "O"
            break
    
        
    while len(points) > 0:
        point = points.pop(0)
        offsets = [(1,0), (0,1), (-1,0),(0,-1)]
        for offset in offsets:
            x = point[0] + offset[0]
            y = point[1] + offset[1]
            if x < 0 or y < 0 or y >= len(grid) or x >= len(grid[0]):
                continue
            if grid[y][x] == ".":
                grid[y][x] = "O"
                points.append((x,y))
    
    for line in grid:
        print(''.join(line))
    outer_count = 0
    inner_count = 0
    for line in grid:
        outer_count += line.count("#")
        inner_count += line.count("O")
    
    print("inner:",inner_count)
    print("outer:", outer_count)
    return outer_count + inner_count



def try_collapse(parts):
    print(parts)
    if len(parts) < 2:
        print(parts)
        print("--------------------")
        return 0
    
    a = parts[-1]
    b = parts[-2]
    flipped = False
    if b[2] < a[2]:
        c = a
        a = b
        b = c
        flipped = True
    
    if not (not a[1] and b[1]):
        print(parts)
        print("--------------------")
        return 0
    
    a_range = a[3]
    b_range = b[3]
    
    min = 0
    max = 0
    if a_range[0] == b_range[0]:
        min = a_range[0]
        max = b_range[1]
        if a_range[1] < b_range[1]:
            max = a_range[1]
    elif a_range[1] == b_range[1]:
        max = a_range[1]
        min = b_range[0]
        if a_range[0] > b_range[0]:
            max = a_range[0]
    
    print(a_range, b_range, (min, max), (a[2], b[2]))

    total = (b[2] - a[2] + 1) * (max - min + 1)

    l = len(parts)-1

    if parts[l][3][0] == min and parts[l][3][1] == max:
        del parts[l]
    elif parts[l][3][0] != min:
        parts[l][3]= (parts[l][3][0], min)
    else:
        parts[l][3] = (max, parts[l][3][1])
    
    l -= 1

    if parts[l][3][0] == min and parts[l][3][1] == max:
        del parts[l]
    elif parts[l][3][0] != min:
        parts[l][3]= (parts[l][3][0], min)
    else:
        parts[l][3] = (max, parts[l][3][1])

    print("total", total)
    print(parts)
    print("--------------------")
    return total + try_collapse(parts)

class Horizontal:
    def __init__(self, waiting_prev, prev_dir, x_start, x_end, y_pos):

        self.x_prev = x_start

        self.x_min = min(x_start, x_end)
        self.x_max = max(x_start, x_end)

        self.waiting_prev = waiting_prev
        self.prev_dir = prev_dir
        self.y_pos = y_pos

    def __repr__(self):
        return f'(H, {self.waiting_prev}, {self.prev_dir}, ({self.x_min, self.x_max}), {self.y_pos})'

class Vertical:
    def __init__(self, waiting_prev, dir, x_pos, y_start, y_end):
        self.waiting_prev = waiting_prev
        self.dir = dir

        self.x_pos = x_pos

        self.y_min = min(y_start, y_end)+1
        self.y_max = max(y_start, y_end)-1
    
    def __repr__(self):
        return f'(V, {self.waiting_prev}, {self.dir}, {self.x_pos}, ({self.y_min, self.y_max}))'


def collapse(parts):
    print(parts)
    if len(parts) < 2:
        print(parts)
        print("-------------------")
        return 0
    
    if type(parts[-1]) is Horizontal:
        print(parts)
        print("-------------------")
        return 0
    
    if type(parts[-2]) is Horizontal:
        left_part = Vertical(parts[-2].waiting_prev, parts[-2].prev_dir, parts[-2].x_prev, parts[-2].y_pos+1, parts[-2].y_pos-1)
        right_part = parts[-1]
        flipped = False
        if left_part.x_pos > right_part.x_pos:
            tmp = left_part
            left_part = right_part
            right_part = tmp
            flipped = True
        
        #total including ends
        total = parts[-2].x_max - parts[-2].x_min + 1
        # .###. and  .#O#.
        # .#O#.      .###.
        if not left_part.waiting_prev and right_part.waiting_prev:
            del parts[-2]

        # O###O and O#.#O
        # O#.#O     O###O
        elif left_part.waiting_prev and not right_part.waiting_prev:
            # take out sides
            total -= 2
            print("here!!!")
            #split horizontal into 2 vertical pieces
            y = parts[-2].y_pos
            left = Vertical(True, left_part.dir, parts[-2].x_min, y+1, y-1)
            right = Vertical(False, right_part.dir, parts[-2].x_max, y+1, y-1)
            
            first = left
            second = right
            if flipped:
                tmp = first
                first = second
                second = tmp
                
            #since we hadd two parts here, we need to collapse the other part too
            print("split horizontal")
            print("total", total)
            print(parts[:-2] + [first,second,parts[-1]] )
            print("-------------------")
            
            last = parts.pop()
            parts[-1] = first
            total += collapse(parts)
            parts.append(second)
            total += collapse(parts)
            parts.append(last)
            
            return total + collapse(parts)

        
        # 0#... and 000#.
        # 0###.     0###.
        # 000#.     0#...
        elif left_part.waiting_prev and right_part.waiting_prev:
            #take out 1 corner
            total -= 1
            
            #leave side furthest to left
            y = parts[-2].y_pos
            parts[-2] = Vertical(True, left_part.dir, parts[-2].x_min, y+1, y-1)
        
        # .#000 and ...#0
        # .###0     .###0
        # ...#0     .#000
        else:
            #take out 1 corner
            total -= 1

            #leave side furthest to right
            y = parts[-2].y_pos
            parts[-2] = Vertical(False, right_part.dir, parts[-2].x_max, y+1, y-1)
        
        print("split horizontal")
        print("total", total)
        print(parts)
        print("-------------------")
        #need to ensure that the horizontal piece doesn't need to collapse
        last = parts.pop()
        total += collapse(parts)
        parts.append(last)
        return total + collapse(parts)

    a = parts[-1]
    b = parts[-2]
    flipped = False
    if b.x_pos < a.x_pos:
        c = a
        a = b
        b = c
        flipped = True
    
    if not (not a.waiting_prev and b.waiting_prev):
        print(parts)
        print("--------------------")
        return 0
    
    min_v = 0
    max_v = 0
    if a.y_min == b.y_min:
        min_v = a.y_min
        max_v = min(a.y_max, b.y_max)
    elif a.y_max == b.y_max:
        max_v = a.y_max
        min_v = max(a.y_min, b.y_min)
    
    print((a.y_min, a.y_max), (b.y_min, b.y_max), (min_v, max_v), (a.x_pos, b.x_pos))

    total = (b.x_pos - a.x_pos + 1) * (max_v - min_v + 1)

    l = len(parts)-1

    if parts[l].y_min == min_v and parts[l].y_max == max_v:
        del parts[l]
    elif parts[l].y_min != min_v:
        parts[l].y_max = min_v - 1
    else:
        parts[l].y_min = max_v + 1
    
    l -= 1

    if parts[l].y_min == min_v and parts[l].y_max == max_v:
        del parts[l]
    elif parts[l].y_min != min_v:
        parts[l].y_max = min_v - 1
    else:
        parts[l].y_min = max_v + 1

    print("total", total)
    print(parts)
    print("--------------------")
    return total + collapse(parts)

def better_part1(inp):
    parts = []
    cur_pos = (0,0)
    total = 0

    starting_prev = False

    initialized = False

    last_dir = "O"
    last_waiting_prev = False
    
    for row, line in enumerate(inp):
        offset = offset_map[line[0]]
        x = cur_pos[0] + offset[0]*int(line[1])
        y = cur_pos[1] + offset[1]*int(line[1])

        new_pos = (x, y)
        
        if row == 0 and line[0] == "R":
            total += int(line[1])
            starting_prev = True
            cur_pos = new_pos
            continue
        
        waiting_prev = False
        if not initialized:
            initialized = True
            waiting_prev = False
            parts.append(Vertical(False, "D", cur_pos[0],cur_pos[1]+1,cur_pos[1]-1))
            """if line[0] == "D":
                waiting_prev = starting_prev
            else:
                total -= 1
                parts.append(Vertical(False, "U", cur_pos[0],cur_pos[1]+1,cur_pos[1]-1))
                waiting_prev = not starting_prev"""
        elif line[0] == "R" or line[0] == "L":
            waiting_prev = last_waiting_prev
        elif last_dir == "L" or last_dir == "R":
            if parts[-1].prev_dir == line[0]:
                waiting_prev = last_waiting_prev
            else:
                waiting_prev = not last_waiting_prev
        else:
            if last_dir == line[0]:
                waiting_prev = last_waiting_prev
            else:
                waiting_prev = not last_waiting_prev
        
        if line[0] == "R" or line[0] == "L":
            parts.append(Horizontal(waiting_prev, last_dir, cur_pos[0], new_pos[0], cur_pos[1]))
        elif row == len(inp)-1:
            tmp_dir = {"D": 1, "U": -1}
            parts.append(Vertical(waiting_prev, line[0], cur_pos[0], cur_pos[1], new_pos[1]+tmp_dir[line[0]]))
        else:
            parts.append(Vertical(waiting_prev, line[0], cur_pos[0], cur_pos[1], new_pos[1]))
        

        if row == len(inp)-3:
            print("HERE!!!!!!!!!!!11")
            print(parts[-1], cur_pos, new_pos)
        last_dir = line[0]
        last_waiting_prev = waiting_prev

        total += collapse(parts)

        cur_pos = new_pos

        if len(parts) < 3:
            print("hiiiiiiii", row)

    
    #print(parts)
    return total

def get_dir(point1, point2):
    if point1[0] < point2[0]:
        return "R"
    elif point1[0] > point2[0]:
        return "L"
    elif point1[1] < point2[1]:
        return "D"
    else:
        return "U"
def take2(inp):
    points = []
    cur_pos = (0.5, 0.5)
    for line in inp:
        points.append(cur_pos)
        offset = offset_map[line[0]]
        x = cur_pos[0] + offset[0]*int(line[1])
        y = cur_pos[1] + offset[1]*int(line[1])


        
        new_pos = (x, y)

        cur_pos = new_pos
    
    new_points = []

    for i, point in enumerate(points):
        prev_point = points[0]
        if i != 0:
            prev_point = points[i-1]
        
        next_point = points[0]
        if i != len(points)-1:
            next_point = points[i+1]
        
        #print(prev_point, point, next_point)
        a = get_dir(prev_point, point)
        b = get_dir(point, next_point)
        
        #print(a,b)
        new_point = (point[0],point[1])
        if a == "U" and b == "R":
            new_point = (point[0]-.5, point[1]-.5)
        elif a == "R" and b == "D":
            new_point = (point[0]+.5, point[1]-.5)
        elif a == "D" and b == "L":
            new_point = (point[0]+.5, point[1]+.5)
        elif a == "L" and b == "D":
            new_point = (point[0]+.5, point[1]+.5)
        elif a == "D" and b == "R":
            new_point = (point[0]+.5, point[1]-.5)
        elif a == "L" and b == "U":
            new_point = (point[0]-.5,point[1]+.5)
        elif a == "U" and b == "L":
            new_point = (point[0]-.5,point[1]+.5)
        elif a == "R" and b == "U":
            new_point = (point[0]-.5,point[1]-.5)

        new_points.append(new_point)
        
        

    #points = [(0,0),(7,0),(7,6),(5,6),(5,7),(7,7),(7,10),(1,10),(1,8),(0,8),(0,5),(2,5),(2,3),(0,3)]
    
    points = new_points

    total = 0
    #shoelace formula
    for i, a in enumerate(points):
        b = points[0]
        if i != len(points)-1:
            b = points[i+1]
        det = a[0]*b[1] - b[0]*a[1]
        total += det

    total /= 2
    return int(total) - 1
    
def part2(inp):
    formatted_inp = []
    dir_map = {"0":"R", "1":"D","2":"L","3":"U"}
    for line in inp:
        code = line[2][2:-1]
        #print(code)
        #print(code[-1])
        direction = dir_map[code[-1]]

        amount = int(code[:-1], 16)
        #print(direction, amount)
        formatted_inp.append([direction, amount])
    return take2(formatted_inp)+1
inp = parse_input()

sol1 = take2(inp)
print("part 1:", sol1)

sol2 = part2(inp)
print("part 2:", sol2)