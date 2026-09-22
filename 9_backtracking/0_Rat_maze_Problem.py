# User function Template for python3
def dfs(i, j, s, matrix, n, visited_mat, ans):
    if i < 0 or j < 0 or i >= n or j >= n:
        return
    if matrix[i][j] == 0 or visited_mat[i][j] == 1:
        return
    if i == n - 1 and j == n - 1:
        ans.append(s)
        return
    visited_mat[i][j] = 1

    dfs(i - 1, j, s + "U", matrix, n, visited_mat, ans)  # Up
    dfs(i + 1, j, s + "D", matrix, n, visited_mat, ans)  # Down
    dfs(i, j - 1, s + "L", matrix, n, visited_mat, ans)  # Left
    dfs(i, j + 1, s + "R", matrix, n, visited_mat, ans)  # Right

    visited_mat[i][j] = 0  # Backtrack


class Solution:
    def findPath(self, m, n):
        visited = [[0 for _ in range(n)] for _ in range(n)]
        ans = []
        string = ""
        dfs(0, 0, string, m, n, visited, ans)
        ans.sort()
        return ans if ans else ["-1"]


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
