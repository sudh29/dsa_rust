# User function Template for python3


class Solution:
    def matrixMultiplication(self, N, arr):
        dp = [[0 for _ in range(N)] for _ in range(N)]
        for l in range(2, N):
            for i in range(1, N - l + 1):
                j = i + l - 1
                dp[i][j] = float("inf")
                for k in range(i, j):
                    cost = dp[i][k] + dp[k + 1][j] + arr[i - 1] * arr[k] * arr[j]
                    if cost < dp[i][j]:
                        dp[i][j] = cost
        return dp[1][N - 1]


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N = int(input())
        arr = input().split()
        for i in range(N):
            arr[i] = int(arr[i])

        ob = Solution()
        print(ob.matrixMultiplication(N, arr))
# } Driver Code Ends
