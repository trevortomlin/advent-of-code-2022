from functools import cmp_to_key

def main():
    print(part1())
    print(part2())

def cmp(l1, l2):

    if type(l1) == int and type(l2) == list:
        l1 = [l1]
    if type(l1) == list and type(l2) == int:
        l2 = [l2]
        
    if type(l1) == int and type(l2) == int:
        
        if l1 < l2:
            return 1
        if l1 == l2:
            return 0

        return -1
    
    else:

        for i in range(max(len(l1), len(l2))):

            if i == len(l1):
                if len(l1) == len(l2):
                    return 0
                return 1
            
            if i == len(l2):
                return -1

            x = cmp(l1[i], l2[i])

            if x == 0:
                continue
            else:
                return x

    return 0

def part1():
    
    res = 0
    pi = 1

    with open("src/day13/input.txt", "r") as f:

        for line in f:

            if line == "\n":
                continue

            l1 = eval(line)

            nextline = next(f, None)

            l2 = eval(nextline)

            x = cmp(l1, l2)

            if x == 1:

                res += pi

            pi += 1


    return res

def part2():

    s = []

    with open("src/day13/input.txt", "r") as f:

        for line in f:

            if line == "\n":
                continue

            l1 = eval(line)

            nextline = next(f, None)

            l2 = eval(nextline)

            s.append(l1)
            s.append(l2)

    sorted_s = sorted(s, key=cmp_to_key(cmp), reverse=True)

    return (sorted_s.index([[2]]) + 1) * (sorted_s.index([[6]]) + 1)

if __name__ == "__main__":
    main()