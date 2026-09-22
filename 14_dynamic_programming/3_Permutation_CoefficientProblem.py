MOD = 10**9 + 7


class Solution:
    def permutationCoeff(self, n, k):
        # if k > n:
        #     return 0
        # result = 1
        # for i in range(k):
        #     result = (result * (n - i)) % MOD
        # return result

        dp = [0 for _ in range(k + 1)]
        dp[0] = 1
        for i in range(1, n + 1):
            for j in range(min(i, k), 0, -1):
                dp[j] = (dp[j] + (j * dp[j - 1]) % MOD) % MOD
        return dp[k]


# {
# Driver Code Starts
# Initial Template for Python 3


if __name__ == "__main__":
    T = int(input())
    for i in range(T):
        n, k = input().split()
        n = int(n)
        k = int(k)
        ob = Solution()
        ans = ob.permutationCoeff(n, k)
        print(ans)
# } Driver Code Ends
