directions = [(1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1), (0, 1), (0, -1)]
def getNum(inp, x, y):
    line = inp[y]
    tmp_x = x
    digit_chars = ""
    while tmp_x >= 0 and line[tmp_x].isdigit():
        digit_chars = line[tmp_x] + digit_chars
        tmp_x -= 1
    tmp_x = x+1
    while tmp_x < len(line) and line[tmp_x].isdigit():
        digit_chars += line[tmp_x]
        tmp_x += 1
    #print(digit_chars)

    return int(digit_chars)
def getNumbers(inp, i, j):
    numbers = []
    for dirc in directions:
        y = i+dirc[0]
        x = j+dirc[1]
        if dirc[1] == 0 and (inp[y][x-1].isdigit() or inp[y][x+1].isdigit()):
            continue
        if dirc[1] == 1 and inp[y][x-1].isdigit() and inp[y][x-2].isdigit():
            continue
        if inp[y][x].isdigit():
            numbers.append(getNum(inp, x, y))
    
    return numbers
        
def part1(inp):
    symbols = []
    for i in range(len(inp)):
        for j in range(len(inp[i])):
            if (not inp[i][j].isalnum()) and inp[i][j] != "\n" and inp[i][j] != ".":
                symbols.append((i,j))
    #print(symbols)

    total = 0
    for symbol in symbols:
        for number in getNumbers(inp, symbol[0], symbol[1]):
            total += number
    
    return total


def part2(inp):
    symbols = []
    for i in range(len(inp)):
        for j in range(len(inp[i])):
            if inp[i][j] == "*":
                symbols.append((i,j))
    total = 0
    for symbol in symbols:
        adjacent_gears = getNumbers(inp, symbol[0], symbol[1])
        if len(adjacent_gears) == 2:
            total += adjacent_gears[0] * adjacent_gears[1]
    return total


f = open("input.txt")
inp = f.readlines()
f.close()

sol1 = part1(inp)
print("part1:", sol1)

sol2 = part2(inp)
print("part2:", sol2)