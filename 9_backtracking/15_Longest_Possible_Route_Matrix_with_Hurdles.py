from typing import List


def solve(mat, n, m, i, j, x, y, visited, curr, ans):
    if i == x and j == y:
        ans[0] = max(ans[0], curr)
        return
    if i < 0 or j < 0 or i >= n or j >= m or mat[i][j] == 0 or visited[i][j]:
        return

    visited[i][j] = True
    if j != m - 1 and mat[i][j + 1] > 0:
        solve(mat, n, m, i, j + 1, x, y, visited, curr + 1, ans)
    if i != n - 1 and mat[i + 1][j] > 0:
        solve(mat, n, m, i + 1, j, x, y, visited, curr + 1, ans)
    if j != 0 and mat[i][j - 1] > 0:
        solve(mat, n, m, i, j - 1, x, y, visited, curr + 1, ans)
    if i != 0 and mat[i - 1][j] > 0:
        solve(mat, n, m, i - 1, j, x, y, visited, curr + 1, ans)
    visited[i][j] = False


class Solution:
    def longestPath(
        self, mat: List[List[int]], n: int, m: int, xs: int, ys: int, xd: int, yd: int
    ) -> int:
        if (
            xs < 0
            or xs >= n
            or ys < 0
            or ys >= m
            or xd < 0
            or xd >= n
            or yd < 0
            or yd >= m
        ):
            return -1

        visited = [[False for _ in range(m)] for _ in range(n)]
        ans = [-1]
        solve(mat, n, m, xs, ys, xd, yd, visited, 0, ans)
        return ans[0]


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

        b = IntArray().Input(4)

        mat = IntMatrix().Input(a[0], a[0])

        obj = Solution()
        res = obj.longestPath(mat, a[0], a[1], b[0], b[1], b[2], b[3])

        print(res)


# } Driver Code Ends
