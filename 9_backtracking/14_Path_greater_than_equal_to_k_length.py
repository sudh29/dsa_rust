def dfs(val, visited, graph, path_len, K):
    if path_len >= K:
        return 1
    visited.add(val)
    for n, w in graph[val]:
        if n not in visited:
            if dfs(n, visited, graph, path_len + w, K):
                return 1
    visited.remove(val)
    return 0


class Solution:
    def pathMoreThanK(self, V, E, K, A):
        graph = [[] for _ in range(V)]
        for i in range(0, len(A), 3):
            src, dest, weight = A[i], A[i + 1], A[i + 2]
            graph[src].append((dest, weight))
            graph[dest].append((src, weight))
        # print(graph)

        visited = set()
        return dfs(0, visited, graph, 0, K)


# {
# Driver Code Starts


if __name__ == "__main__":
    ob = Solution()
    t = int(input())
    for _ in range(t):
        V, E, K = map(int, input().split())
        A = list(map(int, input().split()))
        ans = ob.pathMoreThanK(V, E, K, A)
        if ans:
            print(1)
        else:
            print(0)


# } Driver Code Ends
