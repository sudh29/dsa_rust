import sys
from typing import List


def solve(row, col, mat, visited, i, j, cnt):
    if i == row or i < 0 or j < 0 or j == col or mat[i][j] == 0 or visited[i][j]:
        return float("inf")
    if j == col - 1:
        return cnt
    visited[i][j] = True
    l1 = solve(row, col, mat, visited, i + 1, j, cnt + 1)
    l2 = solve(row, col, mat, visited, i - 1, j, cnt + 1)
    l3 = solve(row, col, mat, visited, i, j + 1, cnt + 1)
    l4 = solve(row, col, mat, visited, i, j - 1, cnt + 1)
    visited[i][j] = False
    return min({l1, l2, l3, l4})


def findPath(mat, rows, cols, visited):
    min_length = float("inf")
    for i in range(rows):
        if mat[i][0]:
            length = solve(rows, cols, mat, visited, i, 0, 0)
            min_length = min(min_length, length)
    return min_length + 1 if min_length != float("inf") else -1


def mark_unsafe(mat, rows, cols):
    st = set()
    for i in range(rows):
        for j in range(cols):
            if mat[i][j] == 0:
                st.add((i, j))
                for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                    nx, ny = i + dx, j + dy
                    if 0 <= nx < rows and 0 <= ny < cols:
                        st.add((nx, ny))

    for i in range(rows):
        for j in range(cols):
            if (i, j) in st:
                mat[i][j] = 0
    return st


class Solution:
    def findShortestPath(self, mat: List[List[int]]) -> int:
        if not mat:
            return -1
        R, C = len(mat), len(mat[0])
        visited = [[False] * C for _ in range(R)]
        # st = mark_unsafe(mat,R,C)
        # return findPath(mat,R, C,visited)

        dx = [-1, 1, 0, 0]
        dy = [0, 0, -1, 1]

        for i in range(R):
            for j in range(C):
                if mat[i][j] == 0:
                    for k in range(4):
                        nx = i + dx[k]
                        ny = j + dy[k]
                        if 0 <= nx < R and 0 <= ny < C and mat[nx][ny] == 1:
                            mat[nx][ny] = -1

        q = []
        for i in range(R):
            if mat[i][0] == 1:
                q.append((0, (i, 0)))
                visited[i][0] = True

        ans = sys.maxsize
        while len(q) != 0:
            val = q.pop(0)
            dist = val[0]
            r = val[1][0]
            c = val[1][1]

            if c == C - 1:
                ans = min(ans, dist + 1)
            for k in range(4):
                nx = r + dx[k]
                ny = c + dy[k]
                if (
                    nx >= 0
                    and ny >= 0
                    and nx < R
                    and ny < C
                    and mat[nx][ny] == 1
                    and not visited[nx][ny]
                ):
                    visited[nx][ny] = True
                    q.append((dist + 1, (nx, ny)))

        return ans if ans != sys.maxsize else -1


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

        mat = IntMatrix().Input(a[0], a[1])

        obj = Solution()
        res = obj.findShortestPath(mat)

        print(res)


# } Driver Code Ends
