class Solution:
    def findCatalan(self, N: int) -> int:
        MOD = 1000000007
        dp = [0] * (N + 1)
        dp[0] = dp[1] = 1
        for i in range(2, N + 1):
            for j in range(i):
                dp[i] = (dp[i] + (dp[j] * dp[i - j - 1]) % MOD) % MOD
        return dp[N]


# {
# Driver Code Starts
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        n = int(input())

        obj = Solution()
        res = obj.findCatalan(n)

        print(res)
