class Point:

    def __init__(self):
        self.pos = (0, 0)
        self.parent = None


def bfs(graph, start, end):
    
    visited = set()
    queue = [start]

    visited.add(start.pos)

    while queue:

        point = queue.pop(0)

        if point.pos == end.pos:
            return point
        
        for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            
            n_pos = (point.pos[0] + i, point.pos[1] + j)

            if n_pos[1] < 0 or n_pos[1] >= len(graph):
                continue
            if n_pos[0] < 0 or n_pos[0] >= len(graph[0]): 
                continue

            if n_pos in visited:
                continue

            if ord(graph[n_pos[1]][n_pos[0]]) - ord(graph[point.pos[1]][point.pos[0]]) > 1:
                continue

            n_p = Point()
            n_p.pos = n_pos
            visited.add(n_p.pos)
            n_p.parent = point
            queue.append(n_p)



def main():
    print(part1())
    print(part2())

def part1():
    
    graph = []

    start = Point()
    end = Point()

    with open("src/day12/input.txt", "r") as f:        
        for j, line in enumerate(f):
            row = []
            for i, c in enumerate(line.strip("\n")):
                row.append(c)

                if c == 'S':
                    start.pos = (i, j)

                elif c == 'E':
                    end.pos = (i, j)

            graph.append(row)

    graph[start.pos[1]][start.pos[0]] = "a"
    graph[end.pos[1]][end.pos[0]] = "z"

    cur = bfs(graph, start, end)

    length = -1

    while cur:
        length += 1
        cur = cur.parent

    return length

def part2():
    
    graph = []

    start = Point()
    end = Point()

    with open("src/day12/input.txt", "r") as f:        
        for j, line in enumerate(f):
            row = []
            for i, c in enumerate(line.strip("\n")):
                row.append(c)

                if c == 'S':
                    start.pos = (i, j)

                elif c == 'E':
                    end.pos = (i, j)

            graph.append(row)

    graph[start.pos[1]][start.pos[0]] = "a"
    graph[end.pos[1]][end.pos[0]] = "z"


    res = float("inf")

    for x in range(len(graph)):
        for y in range(len(graph[0])):
            if graph[x][y] == "a":

                start.pos = (y, x)

                cur = bfs(graph, start, end)

                length = -1

                while cur:
                    length += 1
                    cur = cur.parent

                if length != -1:
                    res = min(res, length)

    return res

if __name__ == "__main__":
    main()