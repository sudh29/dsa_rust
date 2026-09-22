# User function Template for python3


class Solution:
    # function should return True/False
    def isInterleave(self, A, B, C):
        n, m, l = len(A), len(B), len(C)
        if n + m != l:
            return False
        dp = [[False] * (m + 1) for _ in range(n + 1)]
        dp[0][0] = True
        for j in range(1, m + 1):
            dp[0][j] = dp[0][j - 1] and B[j - 1] == C[j - 1]
        for i in range(1, n + 1):
            dp[i][0] = dp[i - 1][0] and A[i - 1] == C[i - 1]
        for i in range(1, n + 1):
            for j in range(1, m + 1):
                dp[i][j] = (dp[i - 1][j] and A[i - 1] == C[i + j - 1]) or (
                    dp[i][j - 1] and B[j - 1] == C[i + j - 1]
                )
        return dp[n][m]


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for i in range(t):
        arr = input().strip().split()
        if Solution().isInterleave(arr[0], arr[1], arr[2]):
            print(1)
        else:
            print(0)
# contributed By: Harshit Sidhwa
# } Driver Code Ends
