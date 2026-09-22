# User function Template for python3


class Solution:
    def maximumPath(self, n, mat):
        dp = [[0] * n for _ in range(n)]
        for c in range(n):
            dp[0][c] = mat[0][c]

        for r in range(1, n):
            for c in range(n):
                if c == 0:
                    dp[r][c] = mat[r][c] + max(dp[r - 1][c], dp[r - 1][c + 1])
                elif c == n - 1:
                    dp[r][c] = mat[r][c] + max(dp[r - 1][c], dp[r - 1][c - 1])
                else:
                    dp[r][c] = mat[r][c] + max(
                        dp[r - 1][c], dp[r - 1][c - 1], dp[r - 1][c + 1]
                    )
        return max(dp[-1])


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N = int(input())
        arr = input().split()
        Matrix = [[0] * N for i in range(N)]
        for itr in range(N * N):
            Matrix[(itr // N)][itr % N] = int(arr[itr])

        ob = Solution()
        print(ob.maximumPath(N, Matrix))

# } Driver Code Ends
