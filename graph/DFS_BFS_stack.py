# dfs and bfs using queue and stack

from collections import deque


def bfs(graph, start):
    visited, queue = set([start]), deque([start])
    while queue:
        vertex = queue.popleft()  # queue
        print(vertex, end=" ")
        for node in graph[vertex]:
            if node not in visited:
                visited.add(node)
                queue.append(node)


def dfs(graph, start):
    visited, queue = set([start]), deque([start])
    while queue:
        vertex = queue.pop()  # stack
        print(vertex, end=" ")
        for node in graph[vertex]:
            if node not in visited:
                visited.add(node)
                queue.append(node)


graph = {
    "0": set(["1", "2"]),
    "1": set(["0", "3", "4"]),
    "2": set(["0", "4"]),
    "3": set(["1", "4"]),
    "4": set(["1", "2", "3"]),
}


print("BFS:")
bfs(graph, "0")
print()
print("DFS:")
dfs(graph, "0")
print()
