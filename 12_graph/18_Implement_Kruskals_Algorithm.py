from heapq import heappop, heappush


class Solution:
    # Function to find sum of weights of edges of the Minimum Spanning Tree.
    def spanningTree(self, V, adj):
        pq = []
        in_mst = [False] * V
        heappush(pq, (0, 0))
        mst_weight = 0
        while pq:
            weight, u = heappop(pq)
            if in_mst[u]:
                continue
            in_mst[u] = True
            mst_weight += weight
            for neighbor, wt in adj[u]:
                if not in_mst[neighbor]:
                    heappush(pq, (wt, neighbor))
        return mst_weight


# {
# Driver Code Starts
# Initial Template for Python 3

# Contributed by : Nagendra Jha

if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        V, E = map(int, input().strip().split())
        adj = [[] for i in range(V)]
        for i in range(E):
            u, v, w = map(int, input().strip().split())
            adj[u].append([v, w])
            adj[v].append([u, w])
        ob = Solution()

        print(ob.spanningTree(V, adj))
# } Driver Code Ends
