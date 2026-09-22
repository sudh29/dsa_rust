from typing import List


def dfs(val, graph, visited, parent):
    visited[val] = True
    for i in graph[val]:
        if not visited[i]:
            if dfs(i, graph, visited, val):
                return True
        elif i != parent:
            return True
    return False


class Solution:
    # Function to detect cycle in an undirected graph.
    def isCycle(self, V: int, adj: List[List[int]]) -> bool:
        visited = [False] * V
        for i in range(V):
            if not visited[i]:
                if dfs(i, adj, visited, -1):
                    return True
        return False


# {
# Driver Code Starts

if __name__ == "__main__":
    T = int(input())
    for i in range(T):
        V, E = map(int, input().split())
        adj = [[] for i in range(V)]
        for _ in range(E):
            u, v = map(int, input().split())
            adj[u].append(v)
            adj[v].append(u)
        obj = Solution()
        ans = obj.isCycle(V, adj)
        if ans:
            print("1")
        else:
            print("0")

# } Driver Code Ends
