from typing import List


class Solution:
    def minimumTime(self, n: int, m: int, edges: List[List[int]]) -> int:
        adj = [[] for i in range(n + 1)]
        for i in range(m):
            adj[edges[i][0]].append(edges[i][1])
        indegree = [0] * (n + 1)
        for i in range(1, n + 1):
            for val in adj[i]:
                indegree[val] += 1
        q = []
        ans = [0] * (n + 1)
        for i in range(1, n + 1):
            if indegree[i] == 0:
                q.append(i)
                ans[i] = 1
        while q:
            node = q.pop(0)
            for val in adj[node]:
                indegree[val] -= 1
                if indegree[val] == 0:
                    q.append(val)
                    ans[val] = ans[node] + 1
        ans = ans[1:]
        return ans


# {
# Driver Code Starts
class IntArray:
    def __init__(self) -> None:
        pass

    def Input(self, n):
        arr = [int(i) for i in input().strip().split()]  # array input
        return arr

    def Print(self, arr):
        for i in arr:
            print(i, end=" ")
        print()


class IntMatrix:
    def __init__(self) -> None:
        pass

    def Input(self, n, m):
        matrix = []
        # matrix input
        for _ in range(n):
            matrix.append([int(i) for i in input().strip().split()])
        return matrix

    def Print(self, arr):
        for i in arr:
            for j in i:
                print(j, end=" ")
            print()


if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        a = IntArray().Input(2)

        edges = IntMatrix().Input(a[1], a[1])

        obj = Solution()
        res = obj.minimumTime(a[0], a[1], edges)

        IntArray().Print(res)


# } Driver Code Ends
