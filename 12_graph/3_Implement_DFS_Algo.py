# User function Template for python3


def solve_dfs(val, visited, graph, ans):
    visited[val] = True
    ans.append(val)
    for i in graph[val]:
        if not visited[i]:
            solve_dfs(i, visited, graph, ans)


class Solution:
    # Function to return a list containing the DFS traversal of the graph.
    def dfsOfGraph(self, V, adj):
        # print(adj)
        if V < 1:
            return
        visited = [False for _ in range(V)]
        res = []
        solve_dfs(0, visited, adj, res)  # recursion stack
        return res

        # Normal stack
        # if V < 1:
        #     return []
        # visited = [False] * V
        # res = []
        # stack = []
        # stack.append(0)
        # visited[0] = True
        # while stack:
        #     value = stack.pop()
        #     res.append(value)
        #     for i in reversed(adj[value]):
        #         if not visited[i]:
        #             stack.append(i)
        #             visited[i] = True
        # return res


# {
# Driver Code Starts

if __name__ == "__main__":
    T = int(input())
    while T > 0:
        V, E = map(int, input().split())
        adj = [[] for i in range(V + 1)]
        for i in range(E):
            u, v = map(int, input().split())
            adj[u].append(v)
            adj[v].append(u)
        ob = Solution()
        ans = ob.dfsOfGraph(V, adj)
        for i in range(len(ans)):
            print(ans[i], end=" ")
        print()
        T -= 1
# } Driver Code Ends
