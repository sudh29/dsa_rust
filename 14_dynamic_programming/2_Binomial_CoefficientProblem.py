MOD = 10**9 + 7


class Solution:
    def nCr(self, n, r):
        if r > n:
            return 0
        dp = [0] * (r + 1)
        dp[0] = 1
        for i in range(1, n + 1):
            for j in range(min(i, r), 0, -1):
                dp[j] = (dp[j] + dp[j - 1]) % MOD
        return dp[r]


# {
# Driver Code Starts
# Initial Template for Python 3

import sys

sys.setrecursionlimit(10**6)

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        n, r = [int(x) for x in input().split()]

        ob = Solution()
        print(ob.nCr(n, r))
# } Driver Code Ends
