class Solution:
    def countFriendsPairings(self, n):
        MOD = 10**9 + 7
        if n == 0:
            return 1
        elif n == 1:
            return 1
        elif n == 2:
            return 2

        dp = [0] * (n + 1)
        dp[0] = 1
        dp[1] = 1
        dp[2] = 2

        for i in range(3, n + 1):
            dp[i] = (dp[i - 1] + (i - 1) * dp[i - 2]) % MOD
        return dp[n]


# {
# Driver Code Starts
# Initial Template for Python 3

import sys

sys.setrecursionlimit(10**6)

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        n = int(input())
        ob = Solution()
        print(ob.countFriendsPairings(n))
# } Driver Code Ends
