# User function Template for python3

from typing import List


class Solution:
    # Function to return Breadth First Traversal of given graph.
    def bfsOfGraph(self, V: int, adj: List[List[int]]) -> List[int]:
        if V < 1:
            return
        visited = [False for _ in range(V)]
        res = []
        q = []
        q.append(0)
        visited[0] = True
        while q:
            value = q.pop(0)
            res.append(value)
            for i in adj[value]:
                if not visited[i]:
                    q.append(i)
                    visited[i] = True
        return res


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
        ob = Solution()
        ans = ob.bfsOfGraph(V, adj)
        for i in range(len(ans)):
            print(ans[i], end=" ")
        print()


# } Driver Code Ends
