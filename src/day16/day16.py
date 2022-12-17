from functools import lru_cache
import sys

sys.setrecursionlimit(10000)

with open(r"src/day16/input.txt") as f:
    s = f.read().strip()

tunnels = {}
flows = {}
for line in s.split("\n"):
    name = line[6:8]
    flow = int(line[23:26].replace(";", ""))
    _, r = line.split(";")
    r = r.replace("valves","valve")[len(" tunnels lead to valve "):]
    tunnels[name] = r.split(", ")
    flows[name] = flow

@lru_cache(maxsize=None)
def dfs(name, opened, minutes):
    if minutes <= 0:
        return 0
    
    best = 0

    pressure = (minutes - 1) * flows[name]

    new_opened = set(opened)
    new_opened.add(name)

    for oth in tunnels[name]:
        if name not in opened and pressure != 0:
            best = max(best, pressure + dfs(oth, frozenset(new_opened), minutes - 2))
        best = max(best, dfs(oth, opened, minutes - 1))

    return best

@lru_cache(maxsize=None)
def dfs2(name, opened, minutes):
    if minutes <= 0:
        return dfs("AA", opened, 26)

    best = 0

    pressure = (minutes - 1) * flows[name]

    new_opened = set(opened)
    new_opened.add(name)

    for oth in tunnels[name]:
        if name not in opened and pressure != 0:
            best = max(best, pressure + dfs2(oth, frozenset(new_opened), minutes - 2))
        best = max(best, dfs2(oth, opened, minutes - 1))

    return best

def main():

    part1 = dfs("AA", frozenset(), 30)
    part2 = dfs2("AA", frozenset(), 26)

    print(f"Part1: {part1}")
    print(f"Part2: {part2}")

if __name__ == "__main__":
    main()