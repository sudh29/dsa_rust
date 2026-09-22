# User function Template for python3


def dfs(m, n, paths, curr_path, i, j, visited):
    if i < 0 or j < 0 or i >= n or j >= n:
        return
    if m[i][j] == 0 or visited[i][j] == 1:
        return

    if i == n - 1 and j == n - 1:
        paths.append(curr_path)
        return

    visited[i][j] = 1
    dfs(m, n, paths, curr_path + "U", i - 1, j, visited)
    dfs(m, n, paths, curr_path + "D", i + 1, j, visited)
    dfs(m, n, paths, curr_path + "L", i, j - 1, visited)
    dfs(m, n, paths, curr_path + "R", i, j + 1, visited)
    visited[i][j] = 0


class Solution:
    def findPath(self, m, n):
        res = []
        if m[0][0] == 0 or m[n - 1][n - 1] == 0:
            return res
        visited = [[0] * n for _ in range(n)]
        dfs(m, n, res, "", 0, 0, visited)
        res.sort()
        return res


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for i in range(t):
        n = list(map(int, input().strip().split()))
        arr = list(map(int, input().strip().split()))

        matrix = [[0 for i in range(n[0])] for j in range(n[0])]
        k = 0
        for i in range(n[0]):
            for j in range(n[0]):
                matrix[i][j] = arr[k]
                k += 1
        ob = Solution()
        result = ob.findPath(matrix, n[0])
        result.sort()
        if len(result) == 0:
            print(-1)
        else:
            for x in result:
                print(x, end=" ")
            print()
# } Driver Code Ends
