# User function Template for python3


class Solution:
    def eggDrop(self, N, K):
        if N == 1:
            return K
        if K == 0:
            return 0

        dp = [[0 for _ in range(K + 1)] for _ in range(N + 1)]

        for i in range(1, N + 1):
            dp[i][1] = 1
            dp[i][0] = 0

        for j in range(1, K + 1):
            dp[1][j] = j

        for i in range(2, N + 1):
            for j in range(2, K + 1):
                dp[i][j] = float("inf")
                for x in range(1, j + 1):
                    res = 1 + max(dp[i - 1][x - 1], dp[i][j - x])
                    dp[i][j] = min(dp[i][j], res)
        return dp[N][K]


# {
# Driver Code Starts
# Initial Template for Python 3

# Contributed by : Nagendra Jha

if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        n, k = map(int, input().strip().split())
        ob = Solution()
        print(ob.eggDrop(n, k))
# } Driver Code Ends
