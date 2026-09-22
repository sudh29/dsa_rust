# User function Template for python3


class Solution:
    def countWays(self, n, k):
        MOD = 10**9 + 7
        if n == 1:
            return k
        if n == 2:
            return k * k % MOD
        # dp = [0] * (n + 1)
        # dp[1] = k
        # dp[2] = k * k
        # for i in range(3, n + 1):
        #     dp[i] = (k - 1) * (dp[i-1] + dp[i-2]) % MOD
        # return dp[n]

        prev2 = k
        prev1 = k * k % MOD
        for i in range(3, n + 1):
            current = (k - 1) * (prev1 + prev2) % MOD
            prev2 = prev1
            prev1 = current
        return prev1


# {
# Driver Code Starts

# Initial Template for Python 3


t = int(input())
for _ in range(0, t):
    x = list(map(int, input().split()))
    n = x[0]
    k = x[1]
    ob = Solution()
    ans = ob.countWays(n, k)
    print(ans)

# } Driver Code Ends
