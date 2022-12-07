class TreeNode():
    def __init__(self):
        self.title = ""
        self.size = 0
        self.children = []
        self.parent = None
        self.type = ""

def dfs_print(node, offset = ""):
    if not node:
        return
    
    print(offset + node.title + " (" + str(node.size) + ")")

    for child in node.children:
        dfs_print(child, offset+" ")

def calc_dir_sizes(node):
    if not node:
        return 0
    
    if node.type == "dir":
        s = 0
        for c in node.children:
            s += calc_dir_sizes(c)
        node.size = s
        return s
    else:
        return node.size

def dfs_sum(node):
    if not node:
        return 0
    if node.type != "dir":
        return 0

    s = node.size if node.size <= 100000 else 0
    for c in node.children:
        s += dfs_sum(c)
    return s

def dfs_smallest(node, used):
    if not node:
        return  float("inf")
    if node.type != "dir":
        return  float("inf")

    free = 70000000 - used
    need = 30000000 - free

    m = node.size if node.size > need else float("inf")

    for c in node.children:
        m = min(m, dfs_smallest(c, used))

    return m

def main():

    cd = TreeNode()
    cd.type = "dir"

    root = TreeNode()
    root.title = "/"
    root.parent = cd
    root.type = "dir"

    head = root

    cd.children.append(root)
    
    with open("src/day7/input.txt", "r") as f:        
        for line in f:

            split = line.split(" ")
            split = [x.strip() for x in split]

            if split[0] == "$":
                
                if split[1] == "cd":

                    if split[2] == "..":
                        cd = cd.parent

                    else:

                        for child in cd.children:

                            if child.title == split[2]:
                                cd = child

            elif split[0] == "dir":
                node = TreeNode()
                node.title = split[1]
                node.parent = cd
                node.type = "dir"
                cd.children.append(node)

            else:
                node = TreeNode()
                node.title = split[1]
                node.parent = cd
                node.size = int(split[0])
                node.type = "file"
                cd.children.append(node)

    calc_dir_sizes(head)

    print("Day7 Part1: " + str(part1(head)))
    print("Day7 Part2: " + str(part2(head)))


def part1(head):
    return dfs_sum(head)

def part2(head):
    return dfs_smallest(head, head.size)

if __name__ == "__main__":
    main()