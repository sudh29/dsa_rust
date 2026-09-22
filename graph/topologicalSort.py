# topological sort

from collections import defaultdict


def topologicalU(v, visit, stack, graph):
    visit[v] = True
    for i in graph[v]:
        if visit[i] == False:
            topologicalU(i, visit, stack, graph)
    stack.append(v)


def topologicalSort(g, graph):
    visited = [False] * (g)
    stack = []
    for i in range((g)):
        if visited[i] == False:
            topologicalU(i, visited, stack, graph)
    print(stack[::-1])


graph = defaultdict(list)

graph[5].append(2)
graph[5].append(0)
graph[4].append(0)
graph[4].append(1)
graph[2].append(3)
graph[3].append(1)

# print(graph[5])

topologicalSort(6, graph)
