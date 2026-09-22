# User function Template for python3
from typing import List


def dfs(val, graph, visited, rec_stack):
    visited[val] = True
    rec_stack[val] = True
    for i in graph[val]:
        if not visited[i]:
            if dfs(i, graph, visited, rec_stack):
                return True
        elif rec_stack[i]:
            return True
    rec_stack[val] = False
    return False


class Solution:
    # Function to detect cycle in a directed graph.
    def isCyclic(self, V: int, adj: List[List[int]]) -> bool:
        visited = [False] * V
        rec_stack = [False] * V
        for i in range(V):
            if not visited[i]:
                if dfs(i, adj, visited, rec_stack):
                    return True
        return False


# {
# Driver Code Starts
# Initial Template for Python 3

import sys

sys.setrecursionlimit(10**6)

if __name__ == "__main__":
    t = int(input())
    for i in range(t):
        V, E = list(map(int, input().strip().split()))
        adj = [[] for i in range(V)]
        for i in range(E):
            a, b = map(int, input().strip().split())
            adj[a].append(b)
        ob = Solution()

        if ob.isCyclic(V, adj):
            print(1)
        else:
            print(0)

# } Driver Code Ends
