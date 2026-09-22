# User function Template for python3
class Solution:
    def countBT(self, h):
        MOD = 10**9 + 7
        if h == 0 or h == 1:
            return 1
        dp = [0] * (h + 1)
        dp[0] = 1
        dp[1] = 1
        for i in range(2, h + 1):
            dp[i] = (dp[i - 1] * dp[i - 1] + 2 * dp[i - 1] * dp[i - 2]) % MOD
        return dp[h]


# {
# Driver Code Starts
# Initial Template for Python 3
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        h = int(input())

        ob = Solution()
        print(ob.countBT(h))
# } Driver Code Ends
