def solve_dfs(val, graph, visited):
    visited[val] = True
    for i in graph[val]:
        if not visited[i]:
            solve_dfs(i, graph, visited)


class Solution:
    def makeConnected(self, n: int, connections: List[List[int]]) -> int:
        m = len(connections)
        if m < n - 1:
            return -1
        graph = [[] for _ in range(n)]
        for u, v in connections:
            graph[u].append(v)
            graph[v].append(u)
        visited = [False] * n
        res = 0
        for i in range(n):
            if not visited[i]:
                res += 1
                solve_dfs(i, graph, visited)
        return res - 1
