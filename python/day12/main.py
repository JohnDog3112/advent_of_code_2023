
def parse_input():
    f = open("input.txt")
    inp = f.readlines()
    f.close()

    springs = []
    for line in inp:
        line = line.strip("\n")
        parts = line.split(" ")

        nums = list(map(lambda x: int(x), parts[1].split(",")))

        springs.append((parts[0], nums))

    return springs


def check_valid(spring, nums):
    cur_count = 0
    part = 0
    for ch in spring:
        #print(ch)
        if cur_count != 0 and ch == ".":
            #print(nums[part], cur_count)
            if nums[part] != cur_count:
                #print(nums[part], cur_count)
                return False
            part += 1
            cur_count = 0
        if cur_count == 0 and ch == "#" and part > len(nums)-1:
            return False
        if ch == "#":
            cur_count += 1

    if not part > len(nums)-2:
        return False
    if part == len(nums)-1 and not nums[part] == cur_count:
        return False

    #print("Valid!", ''.join(spring))
    return True

parts_checked = {}
def recurse(springs, nums, level, prev):
    global parts_checked
    
    if level == 1:
        parts_checked = {}
    
    key = (springs.strip("."), level)
    if key in parts_checked:
        return parts_checked[key]
    
    if len(nums) == level-1:
        to_check = prev + springs.replace("?",".")
        if not check_valid(to_check, nums):
            parts_checked[key] = 0
            return 0
        #print(to_check)
        parts_checked[key] = 1
        return 1
    
    num_hashes = nums[level-1]
    total = 0
    for i in range(len(springs)):
        ch = springs[i]
        if ch == ".":
            continue
        done = False
        if ch == "#":
            done = True
        
        valid = True
        if not i+1+num_hashes-2 < len(springs):
            break
        
        for j in range(num_hashes-1):
            if springs[i+1+j] == ".":
                valid = False
                break
        if i+num_hashes < len(springs) and springs[i+num_hashes] == "#":
            valid = False
        
        if not valid:
            if done:
                break
            continue
        total += recurse(springs[i+num_hashes+1:], nums, level+1, prev + "."*i + "#"*num_hashes + ".")
        if done:
            break
    parts_checked[key] = total
    return total

def brute_force(spring, nums):
    question_poses = []
    for i in range(len(spring)):
        if spring[i] == "?":
            question_poses.append(i)
    iterations = 2**len(question_poses)

    st = []
    for ch in spring:
        st.append(ch)
    
    total_valid = 0
    for i in range(iterations):
        for pos in question_poses:
            if i%2 == 0:
                st[pos] = "."
            else:
                st[pos] = "#"
            i = int(i/2)
        part = 0
        cur_count = 0
        valid = True
        #print(st)
        #print("----------------")
        if check_valid(st, nums):
            total_valid += 1

    #print("valid:", total_valid)
    return total_valid


def part1(inp):
    total = 0
    for i in range(len(inp)):
        spring = inp[i]
        tmp = recurse(spring[0], spring[1], 1, "")
        #print(tmp)
        #tmp1 = brute_force(spring[0], spring[1])
        #if tmp != tmp1:
        #    print(i, tmp, tmp1)
        total += tmp
    
    return total

def part2(inp):
    total = 0
    for i in range(len(inp)):
        spring = inp[i][0]
        nums = inp[i][1][:]
        for _ in range(4):
            spring += "?" + inp[i][0]
            nums += inp[i][1]
        
        tmp = recurse(spring, nums, 1, "")
        #print(tmp)
        total += tmp
    return total

inp = parse_input()
sol1 = part1(inp)
print("part1:", sol1)

sol2 = part2(inp)
print("part2:", sol2)