import sys
import math

class Monkey:

    def __init__(self):
        self.items = []
        self.operation = ""
        self.op_value = -1
        self.test = -1
        self.true_throw = -1
        self.false_throw = -1
        self.count = 0


def main():
    print(part1())
    print(part2())
            

def part1():
    monkies = []

    monkey = None

    with open("src/day11/input.txt", "r") as f:        
            for line in f:

                if line.startswith("Monkey"):
                    
                    if monkey:
                        monkies.append(monkey)

                    monkey = Monkey()

                elif line.startswith("  Starting items:"):

                    split = line.split(" ")

                    for x in split[4:]:
                        monkey.items.append(int(x.strip(", \n")))
                    
                elif line.startswith("  Operation:"):

                    split = line.split(" ")
                    monkey.operation = split[-2].strip("\n")
                    monkey.op_value = int(split[-1].strip("\n")) if split[-1].strip("\n") != "old" else split[-1].strip("\n")

                elif line.startswith("  Test:"):
                    split = line.split(" ")
                    monkey.test = int(split[-1].strip("\n"))

                elif line.startswith("    If true:"):
                    split = line.split(" ")
                    monkey.true_throw = int(split[-1].strip("\n"))

                elif line.startswith("    If false:"):
                    split = line.split(" ")
                    monkey.false_throw = int(split[-1].strip("\n"))

    monkies.append(monkey)

    rounds = 20

    for _ in range(rounds):

        for m in monkies:

            i_remove = []

            for i in range(len(m.items)):

                m.count += 1

                item = m.items[i]

                if m.operation == "*":

                    if m.op_value == "old":

                        m.items[i] *= m.items[i]

                    else:

                        m.items[i] *= m.op_value

                if m.operation == "+":

                    if m.op_value == "old":

                        m.items[i] += m.items[i]

                    else:

                        m.items[i] += m.op_value

                m.items[i] //= 3

                if m.items[i] % m.test == 0:

                    i_remove.append(i)
                    monkies[m.true_throw].items.append(m.items[i])

                else:
                    i_remove.append(i)
                    monkies[m.false_throw].items.append(m.items[i])

            for index in sorted(i_remove, reverse=True):
                del m.items[index]

    counts = sorted([m.count for m in monkies])

    return counts[-1] * counts[-2]

def part2():
    monkies = []

    monkey = None

    with open("src/day11/input.txt", "r") as f:        
            for line in f:

                if line.startswith("Monkey"):
                    
                    if monkey:
                        monkies.append(monkey)

                    monkey = Monkey()

                elif line.startswith("  Starting items:"):

                    split = line.split(" ")

                    for x in split[4:]:
                        monkey.items.append(int(x.strip(", \n")))
                    
                elif line.startswith("  Operation:"):

                    split = line.split(" ")
                    monkey.operation = split[-2].strip("\n")
                    monkey.op_value = int(split[-1].strip("\n")) if split[-1].strip("\n") != "old" else split[-1].strip("\n")

                elif line.startswith("  Test:"):
                    split = line.split(" ")
                    monkey.test = int(split[-1].strip("\n"))

                elif line.startswith("    If true:"):
                    split = line.split(" ")
                    monkey.true_throw = int(split[-1].strip("\n"))

                elif line.startswith("    If false:"):
                    split = line.split(" ")
                    monkey.false_throw = int(split[-1].strip("\n"))

    monkies.append(monkey)

    rounds = 10000

    mod_factor = math.prod([monkey.test for monkey in monkies])

    for _ in range(rounds):

        for m in monkies:

            i_remove = []

            for i in range(len(m.items)):

                m.count += 1

                item = m.items[i]

                if m.operation == "*":

                    if m.op_value == "old":

                        m.items[i] *= m.items[i]

                    else:

                        m.items[i] *= m.op_value

                if m.operation == "+":

                    if m.op_value == "old":

                        m.items[i] += m.items[i]

                    else:

                        m.items[i] += m.op_value

                m.items[i] %= mod_factor

                if m.items[i] % m.test == 0:

                    i_remove.append(i)
                    monkies[m.true_throw].items.append(m.items[i])

                else:
                    i_remove.append(i)
                    monkies[m.false_throw].items.append(m.items[i])

            for index in sorted(i_remove, reverse=True):
                del m.items[index]

    counts = sorted([m.count for m in monkies])

    return counts[-1] * counts[-2]

if __name__ == "__main__":
    main()

