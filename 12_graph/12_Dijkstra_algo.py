import heapq


class Solution:
    # Function to find the shortest distance of all the vertices
    # from the source vertex S.
    def dijkstra(self, V, adj, S):
        # distances = [float('inf')] * V
        # distances[S] = 0
        # visited = set()
        # for _ in range(V):
        #     min_distance = float('inf')
        #     min_vertex = -1
        #     for v in range(V):
        #         if v not in visited and distances[v] < min_distance:
        #             min_distance = distances[v]
        #             min_vertex = v
        #     visited.add(min_vertex)
        #     for neighbor, weight in adj[min_vertex]:
        #         if neighbor not in visited:
        #             distances[neighbor] = min(distances[neighbor], distances[min_vertex] + weight)
        # return distances

        # One loop and min heap
        distances = [float("inf")] * V
        distances[S] = 0
        pq = [(0, S)]
        while pq:
            dist, vertex = heapq.heappop(pq)
            if dist > distances[vertex]:
                continue
            for neighbor, weight in adj[vertex]:
                if distances[vertex] + weight < distances[neighbor]:
                    distances[neighbor] = distances[vertex] + weight
                    heapq.heappush(pq, (distances[neighbor], neighbor))
        return distances


# {
# Driver Code Starts
# Initial Template for Python 3


if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        V, E = map(int, input().strip().split())
        adj = [[] for i in range(V)]
        for i in range(E):
            u, v, w = map(int, input().strip().split())
            adj[u].append([v, w])
            adj[v].append([u, w])
        S = int(input())
        ob = Solution()

        res = ob.dijkstra(V, adj, S)
        for i in res:
            print(i, end=" ")
        print()
# } Driver Code Ends
