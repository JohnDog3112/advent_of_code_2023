def parse_input():
    f = open("input.txt")
    inp = f.readlines()
    f.close()

    return list(map(lambda x: list(map(lambda y: int(y), x.strip("\n").split(" "))),inp))

def check_zero(lis):
    for a in lis:
        if a != 0:
            return False
    return True

def part1(inp):
    added_sums = 0
    
    for hist in inp:
        arr = [hist]
        while not check_zero(arr[-1]):
            new_hist = []
            prev_val = arr[-1][0]
            for val in arr[-1][1:]:
                new_hist.append(val - prev_val)
                prev_val = val
            arr.append(new_hist)
        
        final_val = 0
        for lis in arr:
            final_val += lis[-1]
        
        added_sums += final_val
    
    return added_sums

def part2(inp):
    new = list(map(lambda x: x[::-1], inp))
    return part1(new)


inp = parse_input()

sol1 = part1(inp)
print("part1:", sol1)

sol2 = part2(inp)
print("part2:",sol2)