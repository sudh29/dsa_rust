def solve(n, m, X, Y, ans):
    if m == 0 or n == 0:
        return 0
    if ans[n][m] != -1:
        return ans[n][m]
    if X[n - 1] == Y[m - 1]:
        ans[n][m] = 1 + solve(n - 1, m - 1, X, Y, ans)
    else:
        ans[n][m] = max(solve(n - 1, m, X, Y, ans), solve(n, m - 1, X, Y, ans))
    return ans[n][m]


class Solution:
    def lcs(self, n, m, X, Y):
        # ans = [[-1 for _ in range(m+1)] for _ in range(n+1)]
        # return solve(n, m, X, Y, ans)

        # DP
        prev = [0] * (m + 1)
        curr = [0] * (m + 1)
        for i in range(1, n + 1):
            for j in range(1, m + 1):
                if X[i - 1] == Y[j - 1]:
                    curr[j] = 1 + prev[j - 1]
                else:
                    curr[j] = max(prev[j], curr[j - 1])
            prev, curr = curr, [0] * (m + 1)
        return prev[m]


# {
# Driver Code Starts
# Initial Template for Python 3

# Contributed by : Nagendra Jha

if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        n, m = map(int, input().strip().split())
        str1 = str(input())
        str2 = str(input())
        ob = Solution()
        print(ob.lcs(n, m, str1, str2))

# } Driver Code Ends
