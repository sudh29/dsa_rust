import heapq as hp
from collections import defaultdict


# shortest path to a node
def shortestpath(graph, src, dest):
    h = []
    # track v with cost
    # pop least cost
    # greedy src-min-min-dest
    hp.heappush(h, (0, src))

    # path = []
    while len(h) != 0:
        # print("heap", h)
        currcost, currv = hp.heappop(h)
        if currv == dest:
            return [src, dest, currcost]
        for neigh, neighcost in graph[currv]:
            hp.heappush(h, (currcost + neighcost, neigh))


graph = defaultdict(list)
v, e = 6, 7  # map(int, input().split())
# for i in range(e):
#     u, v, w = map(str, input().split())
#     graph[u].append((v, int(w)))

graph["a"] = [("b", 4)]
graph["a"].append(("c", 2))
graph["b"] = [("c", 5)]
graph["b"].append(("d", 10))
graph["c"] = [("e", 3)]
graph["d"] = [("f", 11)]
graph["e"] = [("d", 4)]

print()
print(graph)
print()
src, dest = "a", "d"  # map(str, input().split())
x = shortestpath(graph, src, dest)
print(x)
